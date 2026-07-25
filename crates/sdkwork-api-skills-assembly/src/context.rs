use sdkwork_routes_skills_app_api::SkillsAppRequestContext;
use sdkwork_routes_skills_backend_api::SkillsBackendRequestContext;
use sdkwork_web_core::{DomainContextInjector, WebApiSurface, WebRequestContext};

#[derive(Clone, Copy, Debug, Default)]
pub struct SkillsDomainContextInjector;

impl DomainContextInjector for SkillsDomainContextInjector {
    fn inject(&self, request: &mut axum::extract::Request, context: &WebRequestContext) {
        let Some(principal) = context.principal.as_ref() else {
            return;
        };
        let Some(tenant_id) = positive_id(principal.tenant_id()) else {
            return;
        };
        let Some(actor_id) = positive_id(principal.user_id()) else {
            return;
        };

        match context.api_surface {
            WebApiSurface::AppApi => {
                let organization_id = principal
                    .organization_id()
                    .and_then(positive_id)
                    .unwrap_or(0);
                request.extensions_mut().insert(SkillsAppRequestContext {
                    tenant_id,
                    actor_id,
                    organization_id,
                });
            }
            WebApiSurface::BackendApi => {
                let Some(organization_id) = principal.organization_id().and_then(positive_id)
                else {
                    return;
                };
                request
                    .extensions_mut()
                    .insert(SkillsBackendRequestContext {
                        tenant_id,
                        organization_id,
                        operator_id: actor_id,
                    });
            }
            _ => {}
        }
    }
}

fn positive_id(value: &str) -> Option<u64> {
    value.parse().ok().filter(|value| *value > 0)
}
