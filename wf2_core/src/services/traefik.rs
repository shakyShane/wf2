use crate::context::Context;
use crate::dc_service::DcService;
use crate::dc_service_network::DcServiceNetwork;

use crate::file::File;
use crate::recipes::m2::output_files::traefik::{TraefikFile, TraefikRedirectFile};
use crate::services::Service;

pub struct TraefikService;

#[derive(Default)]
pub struct TraefikServiceVars;

impl TraefikService {
    // MVP implementation to allow upgrade to Traefik2
    pub fn route_to_svc(
        name: impl Into<String>,
        domains: Vec<String>,
        tls: bool,
        port: u16,
    ) -> Vec<String> {
        let name = name.into();
        let service_name = format!("{}_svc", name.clone());
        let mut routes: Vec<String> = domains
            .iter()
            .map(|domain| TraefikService::route_domain_to_svc(&name, domain.to_string(), tls))
            .flatten()
            .collect();

        let mut val = vec![
            "traefik.enable=true".to_owned(),
            format!(
                "traefik.http.services.{}.loadBalancer.server.port={}",
                service_name, port
            ),
        ];
        routes.append(&mut val);
        routes
    }
    pub fn route_domain_to_svc(name: &str, domain: String, tls: bool) -> Vec<String> {
        let service_name = format!("{}_svc", name);
        let sdomain = domain.replace('.', "-");
        vec![
            format!("traefik.http.routers.{}.rule=Host(`{}`)", sdomain, domain),
            format!(
                "traefik.http.routers.{}.service={}",
                sdomain,
                service_name.to_owned()
            )
            .to_string(),
            format!("traefik.http.routers.{}.tls={}", sdomain, tls),
        ]
    }
}

impl Service for TraefikService {
    const NAME: &'static str = "traefik";
    const IMAGE: &'static str = "traefik:2.2";

    fn dc_service(&self, ctx: &Context, _vars: &()) -> DcService {
        DcService::new(ctx.name(), Self::NAME, Self::IMAGE)
            .set_volumes(vec![
                "/var/run/docker.sock:/var/run/docker.sock".to_string(),
                format!(
                    "{}:/etc/traefik/traefik.toml",
                    TraefikFile::from_ctx(&ctx)
                        .expect("cannot fail to get traefik file")
                        .file_path_string(),
                ),
                format!(
                    "{}:/etc/traefik/dynamic/redirect.toml",
                    TraefikRedirectFile::from_ctx(&ctx)
                        .expect("cannot fail to get traefik redirect file")
                        .file_path_string(),
                ),
            ])
            .set_ports(vec!["80:80", "443:443", "8080:8080"])
            .set_labels(vec![Self::TRAEFIK_DISABLE_LABEL])
            .set_network(
                "default",
                DcServiceNetwork::with_aliases(ctx.domains.clone()),
            )
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_host_entry() {
        let labels = TraefikService::route_to_svc("mailhog", vec!["mail.jh".into()], true, 8080);
        assert_eq!(
            labels,
            vec![
                "traefik.http.routers.mail-jh.rule=Host(`mail.jh`)",
                "traefik.http.routers.mail-jh.service=mailhog_svc",
                "traefik.http.routers.mail-jh.tls=true",
                "traefik.enable=true",
                "traefik.http.services.mailhog_svc.loadBalancer.server.port=8080"
            ]
        )
    }

    #[test]
    fn test_aliases() {
        let ctx = Context::default();
        let actual = (TraefikService).dc_service(&ctx, &());
        let expected = r#"

            name: "traefik"
            container_name: wf2__wf2_default__traefik
            image: "traefik:2.2"
            volumes:
              - "/var/run/docker.sock:/var/run/docker.sock"
              - "./.wf2_default/traefik/traefik.toml:/etc/traefik/traefik.toml"
              - "./.wf2_default/traefik/dynamic/redirect.toml:/etc/traefik/dynamic/redirect.toml"
            labels:
              - traefik.enable=false
            ports:
              - "80:80"
              - "443:443"
              - "8080:8080"
            networks:
              default:
                aliases:
                  - local.m2

        "#;
        let expected_dc: DcService = serde_yaml::from_str(expected).expect("deserialize");
        assert_eq!(actual, expected_dc);
    }
}
