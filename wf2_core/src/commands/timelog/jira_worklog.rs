use crate::commands::timelog::jira::Jira;
use crate::commands::timelog::jira_types::{JiraWorklog, WorkType};
use crate::commands::timelog::jira_user::JiraUser;
use chrono::{Date, DateTime, Utc};
use reqwest::header::AUTHORIZATION;

use crate::commands::timelog::jira_issues::JiraIssue;
use std::sync::Arc;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default)]
pub struct Worklog {
    pub started: String,

    #[serde(skip_serializing)]
    pub author: JiraUser,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,

    #[serde(rename = "timeSpentSeconds", skip_serializing)]
    pub time_spent_seconds: u64,

    #[serde(rename = "timeSpent")]
    pub time_spent: String,

    #[serde(skip_serializing)]
    pub ticket_key: Option<String>,

    #[serde(skip_serializing)]
    pub ticket_status: Option<String>,

    #[serde(skip_serializing)]
    pub link: Option<String>,
}

#[derive(Debug, Fail, PartialEq)]
enum WorklogError {
    #[fail(display = "Create failed: {}", _0)]
    CreateFailed(String),
}

impl Worklog {
    pub fn date(&self) -> Date<Utc> {
        let date_time = self.started.parse::<DateTime<Utc>>().expect("can parse");
        date_time.date()
    }
    pub fn display_started_time(&self) -> String {
        let date_time = self.started.parse::<DateTime<Utc>>().expect("can parse");
        date_time.format("%a, %d %b %Y, %H:%M:%S").to_string()
    }
    pub fn work_type(&self) -> WorkType {
        if let Some(comment) = self.comment.as_ref() {
            if comment.contains("overtime") {
                return WorkType::Overtime;
            }
        }
        WorkType::Normal
    }
    pub fn from_issue(jira: Arc<Jira>, issue: JiraIssue) -> Result<Vec<Worklog>, String> {
        fetch_worklog(jira.domain.clone(), jira.basic_auth(), issue.key.clone()).map(move |wl| {
            wl.into_iter()
                .map(move |wl| {
                    let link = jira.issue_link(&issue.key);
                    Worklog {
                        ticket_key: Some(issue.key.clone()),
                        ticket_status: Some(issue.fields.status.name.clone()),
                        link: Some(link),
                        ..wl
                    }
                })
                .collect::<Vec<Worklog>>()
        })
    }
    pub fn create(
        date: Option<impl Into<String>>,
        time: Option<impl Into<String>>,
        spent: impl Into<String>,
        comment: Option<impl Into<String>>,
    ) -> Result<Worklog, failure::Error> {
        let started = get_time_started(Utc::now(), date, time)?;
        Ok(Worklog {
            time_spent: spent.into(),
            started: started.format("%Y-%m-%dT%H:%M:%S.222+0000").to_string(),
            comment: comment.map(|s| s.into()),
            ..Worklog::default()
        })
    }
}

fn fetch_worklog(
    domain: String,
    basic_auth: String,
    issue_id: impl Into<String>,
) -> Result<Vec<Worklog>, String> {
    let client = reqwest::Client::new();
    let id = issue_id.into();
    let issue_url = format!(
        "https://{}.atlassian.net/rest/api/2/issue/{}/worklog",
        domain, id
    );
    let mut res = client
        .get(&issue_url)
        .header(AUTHORIZATION, basic_auth)
        .send()
        .map_err(|e| e.to_string())?;
    let bytes = res.text().map_err(|e| e.to_string())?;
    let worklog: JiraWorklog =
        serde_json::from_str(&bytes).map_err(|e| format!("issue_id = {}, error = {}", id, e))?;
    Ok(worklog.worklogs)
}

pub fn create_worklog(
    domain: String,
    basic_auth: String,
    issue_id: impl Into<String>,
    wl: Worklog,
) -> Result<(), failure::Error> {
    let client = reqwest::Client::new();
    let id = issue_id.into();
    let issue_url = format!(
        "https://{}.atlassian.net/rest/api/2/issue/{}/worklog",
        domain, id
    );
    let mut res = client
        .post(&issue_url)
        .header(AUTHORIZATION, basic_auth)
        .json(&wl)
        .send()?;
    if res.status().is_success() {
        let _res_text = res.text()?;
        Ok(())
    } else {
        let res_text = res.text()?;
        Err(WorklogError::CreateFailed(res_text).into())
    }
}

///
/// Take optional date & time inputs and produce a new date/time
///
/// Examples
///
/// ```rust
/// use chrono::{Utc, TimeZone, Timelike};
/// use wf2_core::commands::timelog::jira_worklog::get_time_started;
/// let now = Utc.ymd(2019, 11, 30).and_hms(12, 0, 0);
///
/// // No date or time given
/// let actual = get_time_started(now, None, None);
/// assert_eq!(now, actual.expect("test"));
///
/// // Just a date given
/// let actual = get_time_started(now, Some("2019-11-01"), None);
/// let expected = Utc.ymd(2019, 11, 1).and_hms(12, 0, 0);
/// assert_eq!(expected, actual.expect("test"));
///
/// // Just a time given
/// let actual = get_time_started(now, None, Some("09:01:10"));
/// let expected = Utc.ymd(2019, 11, 30).and_hms(9, 1, 10);
/// assert_eq!(expected, actual.expect("test"));
///
/// // Data + time given
/// let actual = get_time_started(now, Some("2019-11-01"), Some("09:01:10"));
/// let expected = Utc.ymd(2019, 11, 1).and_hms(9, 1, 10);
/// assert_eq!(expected, actual.expect("test"))
/// ```
///
pub fn get_time_started(
    now: DateTime<Utc>,
    date: Option<impl Into<String>>,
    time: Option<impl Into<String>>,
) -> Result<DateTime<Utc>, failure::Error> {
    let now_date_str = now.format("%Y-%m-%d").to_string();

    match (date, time) {
        // no inputs, default to now + today
        (None, None) => Ok(now),
        // has date, use 12pm as a default
        (Some(date), None) => format!("{}T12:00:00+0000", date.into()).parse::<DateTime<Utc>>(),
        // has a time only, use today
        (None, Some(time)) => {
            format!("{}T{}+0000", now_date_str, time.into()).parse::<DateTime<Utc>>()
        }
        // has both date+time, try to use both
        (Some(date), Some(time)) => {
            let date = format!("{}T{}+0000", date.into(), time.into());
            date.parse::<DateTime<Utc>>()
        }
    }
    .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use crate::commands::timelog::jira_worklog::Worklog;

    #[test]
    fn test_serialize() -> Result<(), failure::Error> {
        let _expected = "2020-02-20T07:36:38.222+0000";
        let wl = Worklog::create(Some("2020-02-20"), Some("07:36:38"), "3h", Some("overtime"))
            .expect("test");
        let as_json = serde_json::to_string_pretty(&wl).expect("serde");

        let example = r#"{
  "started": "2020-02-20T07:36:38.222+0000",
  "comment": "overtime",
  "timeSpent": "3h"
}"#;
        assert_eq!(example, as_json);
        Ok(())
    }
}
