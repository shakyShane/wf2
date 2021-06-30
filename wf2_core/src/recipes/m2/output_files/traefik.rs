use crate::context::Context;
use crate::file::File;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TraefikFile {
    file_path: PathBuf,
}

impl File<TraefikFile> for TraefikFile {
    const DESCRIPTION: &'static str = "Writes the traefix file";
    const HOST_OUTPUT_PATH: &'static str = "traefik/traefik.toml";

    fn from_ctx(ctx: &Context) -> Result<TraefikFile, failure::Error> {
        Ok(TraefikFile {
            file_path: ctx.output_file_path(Self::HOST_OUTPUT_PATH),
        })
    }

    fn file_path(&self) -> PathBuf {
        self.file_path.clone()
    }

    fn bytes(&self) -> Vec<u8> {
        include_bytes!("traefik.toml").to_vec()
    }
}

#[derive(Debug, Clone)]
pub struct TraefikRedirectFile {
    file_path: PathBuf,
}

impl File<TraefikRedirectFile> for TraefikRedirectFile {
    const DESCRIPTION: &'static str = "Writes the traefik redirect file";
    const HOST_OUTPUT_PATH: &'static str = "traefik/dynamic/redirect.toml";

    fn from_ctx(ctx: &Context) -> Result<TraefikRedirectFile, failure::Error> {
        Ok(TraefikRedirectFile {
            file_path: ctx.output_file_path(Self::HOST_OUTPUT_PATH),
        })
    }

    fn file_path(&self) -> PathBuf {
        self.file_path.clone()
    }

    fn bytes(&self) -> Vec<u8> {
        include_bytes!("traefik/redirect.toml").to_vec()
    }
}
