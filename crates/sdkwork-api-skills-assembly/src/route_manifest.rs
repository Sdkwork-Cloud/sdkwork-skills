use std::sync::OnceLock;

use sdkwork_web_core::{HttpRoute, HttpRouteManifest};

pub fn skills_api_route_manifest() -> HttpRouteManifest {
    static ROUTES: OnceLock<&'static [HttpRoute]> = OnceLock::new();
    let routes = ROUTES.get_or_init(|| {
        let mut routes = sdkwork_routes_skills_app_api::app_route_manifest()
            .routes()
            .to_vec();
        routes.extend_from_slice(
            sdkwork_routes_skills_backend_api::backend_route_manifest().routes(),
        );
        Box::leak(routes.into_boxed_slice())
    });
    HttpRouteManifest::new(routes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combined_manifest_contains_each_owned_route_once() {
        let manifest = skills_api_route_manifest();
        assert_eq!(manifest.routes().len(), 24);

        let mut identities = std::collections::HashSet::new();
        for route in manifest.routes() {
            let identity = format!("{:?} {}", route.method, route.path);
            assert!(
                identities.insert(identity),
                "duplicate route manifest entry: {:?} {}",
                route.method,
                route.path
            );
        }
    }
}
