//!
//! Stop all containers, but don't remove them.
//!
//! This command differs from `down` since it will not remove any containers
//! and it will also not clean up any created networks.
//!
//! Basically, use `wf2 stop` when you just want containers to stop, but be
//! able to quickly bring them back up (with their networks and data) at a later
//! point
//!
//! # Example
//!
//! ```
//! # use wf2_core::test::Test;
//! # use wf2_core::cli::cli_input::CLIInput;
//! # let cmd = r#"
//! wf2 stop
//! # "#;
//! # let (commands, ..) = Test::from_cmd(cmd)
//! #     .with_file("../fixtures/config_01.yaml")
//! #     .with_cli_input(CLIInput::from_cwd("/users/shane"))
//! #     .file_ops_commands();
//! # assert_eq!(commands, vec!["docker-compose -f /users/shane/.wf2_m2_shane/docker-compose.yml stop"])
//! ```
use crate::commands::CliCommand;
use crate::context::Context;
use crate::recipes::recipe_kinds::RecipeKinds;
use crate::task::Task;
use clap::{App, ArgMatches};

pub struct DcStop;

impl DcStop {
    pub const NAME: &'static str = "stop";
    pub const ABOUT: &'static str = "Take down containers & retain data";
    pub fn cmd(&self, ctx: &Context) -> Result<Vec<Task>, failure::Error> {
        let recipe = RecipeKinds::from_ctx(&ctx);
        let dc_tasks = recipe.dc_tasks(&ctx)?;
        Ok(vec![dc_tasks.cmd_task(vec![Self::NAME.to_string()])])
    }
}

impl<'a, 'b> CliCommand<'a, 'b> for DcStop {
    fn name(&self) -> String {
        String::from(Self::NAME)
    }
    fn exec(&self, _matches: Option<&ArgMatches>, ctx: &Context) -> Option<Vec<Task>> {
        Some(self.cmd(&ctx).unwrap_or_else(Task::task_err_vec))
    }
    fn subcommands(&self, _ctx: &Context) -> Vec<App<'a, 'b>> {
        vec![App::new(Self::NAME)
            .about(Self::ABOUT)
            .arg_from_usage("-v --volumes 'also remove volumes'")]
    }
}
