use crate::commands::timelog::jira_worklog::Worklog;
use core::fmt;
use std::fmt::Error;
use std::fmt::Formatter;

#[derive(Deserialize, Debug, Clone)]
pub struct JiraField {
    pub issuetype: JiraIssueType,
    pub status: JiraStatus,
    pub summary: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JiraIssueType {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JiraStatus {
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct JiraWorklog {
    pub worklogs: Vec<Worklog>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WorkType {
    Normal,
    Overtime,
}

impl fmt::Display for WorkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            WorkType::Normal => write!(f, "Normal"),
            WorkType::Overtime => write!(f, "Overtime"),
        }
    }
}
