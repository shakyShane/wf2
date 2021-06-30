use crate::context::Context;
use crate::dc_service::DcService;

use crate::recipes::m2::services::php::PhpService;
use crate::recipes::m2::services::M2RecipeOptions;
use crate::services::nginx::NginxService;
use crate::services::pwa::PwaService;
use crate::services::traefik::TraefikService;
use crate::services::Service;

pub struct VarnishService;

impl Service for VarnishService {
    const NAME: &'static str = "varnish";
    const IMAGE: &'static str = "wearejh/varnish:latest";

    fn dc_service(&self, ctx: &Context, _: &()) -> DcService {
        let mut base_domains = ctx.domains();
        let mut depends_on = vec![NginxService::NAME, PhpService::NAME];

        if let Some(opts) = M2RecipeOptions::get_pwa_options(ctx) {
            base_domains.extend(opts.domains);
            depends_on.push(PwaService::NAME);
        }

        DcService::new(ctx.name(), Self::NAME, Self::IMAGE)
            .set_depends_on(depends_on)
            .set_labels(TraefikService::route_to_svc(
                Self::NAME,
                base_domains,
                true,
                80,
            ))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dc_service::DcService;

    #[test]
    fn test_varnish_service() {
        let ctx = Context::default();
        let actual_dc = (VarnishService).dc_service(&ctx, &());
        let expected = r#"

            name: varnish
            container_name: wf2__wf2_default__varnish
            image: "wearejh/varnish:latest"
            labels:
              - "traefik.http.routers.local-m2.rule=Host(`local.m2`)"
              - "traefik.http.routers.local-m2.service=varnish_svc"
              - "traefik.http.routers.local-m2.tls=true"
              - "traefik.enable=true"
              - "traefik.http.services.varnish_svc.loadBalancer.server.port=80"
            depends_on:
              - nginx
              - php
        "#;
        let expected_dc: DcService = serde_yaml::from_str(expected).expect("test yaml");
        assert_eq!(actual_dc, expected_dc);
    }

    #[test]
    fn test_varnish_with_pwa_service() {
        let ctx = Context::new_from_file("../fixtures/pwa.yml")
            .unwrap()
            .unwrap();
        let actual_dc = (VarnishService).dc_service(&ctx, &());
        let expected = r#"

            name: varnish
            container_name: wf2__wf2_default__varnish
            image: "wearejh/varnish:latest"
            labels:
              - "traefik.http.routers.example-m2.rule=Host(`example.m2`)"
              - "traefik.http.routers.example-m2.service=varnish_svc"
              - "traefik.http.routers.example-m2.tls=true"
              - "traefik.http.routers.example-pwa.rule=Host(`example.pwa`)"
              - "traefik.http.routers.example-pwa.service=varnish_svc"
              - "traefik.http.routers.example-pwa.tls=true"
              - "traefik.http.routers.test-ngrok-io.rule=Host(`test.ngrok.io`)"
              - "traefik.http.routers.test-ngrok-io.service=varnish_svc"
              - "traefik.http.routers.test-ngrok-io.tls=true"
              - "traefik.enable=true"
              - "traefik.http.services.varnish_svc.loadBalancer.server.port=80"
            depends_on:
              - nginx
              - php
              - pwa

        "#;
        let expected_dc: DcService = serde_yaml::from_str(expected).expect("test yaml");
        assert_eq!(actual_dc, expected_dc);
    }
}
