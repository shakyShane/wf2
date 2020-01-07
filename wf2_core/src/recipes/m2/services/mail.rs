use crate::context::Context;
use crate::dc_service::DcService;
use crate::recipes::m2::m2_vars::M2Vars;
use crate::recipes::m2::services::M2Service;
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

impl M2Service for MailService {
    const NAME: &'static str = "mail";
    const IMAGE: &'static str = "mailhog/mailhog";

    fn dc_service(&self, ctx: &Context, _vars: &M2Vars) -> DcService {
        DcService::new(ctx.name.clone(), Self::NAME, Self::IMAGE)
            .set_ports(vec!["1025"])
            .set_labels(vec![
                format!("traefik.frontend.rule=Host:{}", MailService::DOMAIN),
                String::from("traefik.port=8025"),
            ])
            .build()
    }
}