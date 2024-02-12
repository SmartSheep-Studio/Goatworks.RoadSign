use actix_web::http::header::HeaderMap;
use actix_web::http::{Method, Uri};
use regex::Regex;
use wildmatch::WildMatch;

use crate::warden::WardenInstance;

use self::{
    config::{Location, Region},
    metrics::RoadMetrics,
};

pub mod config;
pub mod loader;
pub mod metrics;
pub mod responder;
pub mod route;

#[derive(Debug, Clone)]
pub struct RoadInstance {
    pub regions: Vec<Region>,
    pub metrics: RoadMetrics,
    pub warden: WardenInstance,
}

impl RoadInstance {
    pub fn new() -> RoadInstance {
        RoadInstance {
            regions: vec![],
            warden: WardenInstance {
                applications: vec![],
            },
            metrics: RoadMetrics::new(),
        }
    }

    pub fn filter(
        &self,
        uri: &Uri,
        method: &Method,
        headers: &HeaderMap,
    ) -> Option<(&Region, &Location)> {
        self.regions.iter().find_map(|region| {
            let location = region.locations.iter().find(|location| {
                let mut hosts = location.hosts.iter();
                if !hosts.any(|item| {
                    WildMatch::new(item.as_str()).matches(uri.host().unwrap_or("localhost"))
                }) {
                    return false;
                }

                let mut paths = location.paths.iter();
                if !paths.any(|item| {
                    uri.path().starts_with(item)
                        || Regex::new(item.as_str()).unwrap().is_match(uri.path())
                }) {
                    return false;
                }

                if let Some(val) = location.methods.clone() {
                    if !val.iter().any(|item| *item == method.to_string()) {
                        return false;
                    }
                }

                if let Some(val) = location.headers.clone() {
                    match !val.keys().all(|item| {
                        headers.get(item).unwrap()
                            == location.headers.clone().unwrap().get(item).unwrap()
                    }) {
                        true => return false,
                        false => (),
                    }
                };

                if let Some(val) = location.queries.clone() {
                    let queries: Vec<&str> = uri.query().unwrap_or("").split('&').collect();
                    if !val.iter().all(|item| queries.contains(&item.as_str())) {
                        return false;
                    }
                }

                true
            });

            location.map(|location| (region, location))
        })
    }
}
