use crate::context::Context;
use crate::dc_service::DcService;

use crate::services::traefik::TraefikService;
use crate::services::Service;
use std::fmt;

pub struct MailService;

impl MailService {
    pub const DOMAIN: &'static str = "mail.jh";
}

impl fmt::Display for MailService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "MailHog: https://{}", MailService::DOMAIN)
    }
}

impl Service for MailService {
    const NAME: &'static str = "mail";
    const IMAGE: &'static str = "mailhog/mailhog";

    fn dc_service(&self, ctx: &Context, _: &()) -> DcService {
        let traefik_label = TraefikService::route_to_svc(
            MailService::NAME,
            vec![MailService::DOMAIN.into()],
            true,
            8025,
        );
        DcService::new(ctx.name(), Self::NAME, Self::IMAGE)
            .set_ports(vec!["1025"])
            .set_labels(traefik_label)
            .finish()
    }
}
