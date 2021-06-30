use crate::context::Context;
use crate::file::File;

use crate::output_files::OutputFiles;
use crate::recipes::m2::services::M2RecipeOptions;
use crate::recipes::m2::M2Recipe;
use crate::task::Task;
use db_conf::DbConf;
use db_init::DbInit;
use m2_runtime_env_file::M2RuntimeEnvFile;
use nginx_m2::NginxM2;
use nginx_pwa::NginxPwa;
use nginx_upstream::NginxUpstream;
use traefik::{TraefikFile, TraefikRedirectFile};
use unison::UnisonFile;

pub mod auth;
pub mod composer;
pub mod db_conf;
pub mod db_init;
pub mod m2_runtime_env_file;
pub mod nginx_m2;
pub mod nginx_pwa;
pub mod nginx_upstream;
pub mod traefik;
pub mod unison;
pub mod varnish_enabled;

impl OutputFiles for M2Recipe {
    fn output_files(&self, ctx: &Context) -> Result<Vec<Task>, failure::Error> {
        let mut files = vec![
            M2RuntimeEnvFile::from_ctx(&ctx)?.write_task(),
            UnisonFile::from_ctx(&ctx)?.write_task(),
            TraefikFile::from_ctx(&ctx)?.write_task(),
            TraefikRedirectFile::from_ctx(&ctx)?.write_task(),
            NginxUpstream::from_ctx(&ctx)?.write_task(),
            NginxM2::from_ctx(&ctx)?.write_task(),
            DbConf::from_ctx(&ctx)?.write_task(),
            DbInit::from_ctx(&ctx)?.write_task(),
            // NginxPwa::from_ctx(&ctx)?.write_task(),
            // VarnishEnabled::from_ctx(&ctx)?.write_task(),
        ];

        if M2RecipeOptions::has_pwa_options(ctx) {
            files.push(NginxPwa::from_ctx(&ctx)?.write_task())
        }

        Ok(files)
    }
}
