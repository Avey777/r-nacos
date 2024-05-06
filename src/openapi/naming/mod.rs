use actix_web::{web, Scope};

use crate::openapi::constant::{NAMING_V1_BASE_PATH, SLASH};
use crate::openapi::RouteConf;

mod catalog;
pub(crate) mod instance;
mod operator;
pub(crate) mod service;
mod v2;

pub fn openapi_service(conf: RouteConf) -> Scope {
    web::scope(SLASH).service(openapi_route(conf))
}

pub fn openapi_route(_conf: RouteConf) -> Scope {
    web::scope(NAMING_V1_BASE_PATH)
        .service(instance::service())
        .service(service::service())
        .service(operator::service())
        .service(catalog::service())
}
