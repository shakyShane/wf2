use crate::context::Context;
use crate::dc_service::DcService;
use crate::services::traefik::TraefikService;
use crate::services::Service;
use std::fmt;

pub struct RabbitMqService;

impl RabbitMqService {
    pub const DOMAIN: &'static str = "queue.jh";
    pub const PORT_PUBLIC: u16 = 15672;
    pub const PORT_INTERNAL: u16 = 5672;
}

impl fmt::Display for RabbitMqService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "RabbitMQ: https://{} (user: docker, password: docker)",
            RabbitMqService::DOMAIN
        )
    }
}

impl Service for RabbitMqService {
    const NAME: &'static str = "rabbitmq";
    const IMAGE: &'static str = "rabbitmq:3.7-management-alpine";

    fn dc_service(&self, ctx: &Context, _: &()) -> DcService {
        DcService::new(ctx.name(), Self::NAME, Self::IMAGE)
            .set_ports(vec![
                format!("{port}:{port}", port = RabbitMqService::PORT_PUBLIC),
                format!("{port}:{port}", port = RabbitMqService::PORT_INTERNAL),
            ])
            .set_labels(TraefikService::route_to_svc(
                RabbitMqService::NAME,
                vec![RabbitMqService::DOMAIN.into()],
                false,
                RabbitMqService::PORT_PUBLIC,
            ))
            .finish()
    }
}
