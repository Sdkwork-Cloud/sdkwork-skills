use sdkwork_web_bootstrap::application_security_policy_from_env;
use sdkwork_web_core::{HttpMetricsDimensions, SecurityPolicy, WebEnvironment};

const DEFAULT_BIND_ADDRESS: &str = "127.0.0.1:18092";
const SERVICE_NAME: &str = "sdkwork-api-skills-standalone-gateway";

#[derive(Clone)]
pub(crate) struct GatewayRuntimeConfig {
    pub bind_address: String,
    pub environment: WebEnvironment,
    pub security_policy: SecurityPolicy,
    pub metrics_dimensions: HttpMetricsDimensions,
}

impl GatewayRuntimeConfig {
    pub fn from_env() -> Result<Self, String> {
        let deployment_profile = env_value(
            &[
                "SDKWORK_DEPLOYMENT_PROFILE",
                "SDKWORK_SKILLS_DEPLOYMENT_PROFILE",
            ],
            "standalone",
        )
        .to_ascii_lowercase();
        if deployment_profile != "standalone" {
            return Err(format!(
                "Skills standalone gateway requires deployment profile `standalone`, found `{deployment_profile}`"
            ));
        }

        let runtime_target = env_value(
            &["SDKWORK_RUNTIME_TARGET", "SDKWORK_SKILLS_RUNTIME_TARGET"],
            "server",
        )
        .to_ascii_lowercase();
        if !matches!(runtime_target.as_str(), "server" | "container") {
            return Err(format!(
                "Skills standalone gateway runtime target must be `server` or `container`, found `{runtime_target}`"
            ));
        }

        let (environment, security_policy) = application_security_policy_from_env(
            &["SDKWORK_ENVIRONMENT", "SDKWORK_SKILLS_ENVIRONMENT"],
            &["SDKWORK_CORS_ALLOWED_ORIGINS"],
        );
        let metrics_dimensions =
            HttpMetricsDimensions::from_profile_environment(environment.clone())
                .with_service(SERVICE_NAME)
                .with_deployment_profile(deployment_profile)
                .with_runtime_target(runtime_target)
                .with_runtime_profile("postgresql");

        Ok(Self {
            bind_address: env_value(
                &["SDKWORK_SKILLS_APPLICATION_PUBLIC_INGRESS_BIND"],
                DEFAULT_BIND_ADDRESS,
            ),
            environment,
            security_policy,
            metrics_dimensions,
        })
    }

    #[cfg(test)]
    pub fn test() -> Self {
        let environment = WebEnvironment::Test;
        Self {
            bind_address: "127.0.0.1:0".to_owned(),
            security_policy: sdkwork_web_bootstrap::security_policy_for_environment(
                &environment,
                Vec::new(),
            ),
            metrics_dimensions: HttpMetricsDimensions::from_profile_environment(
                environment.clone(),
            )
            .with_service(SERVICE_NAME)
            .with_deployment_profile("standalone")
            .with_runtime_target("server")
            .with_runtime_profile("postgresql"),
            environment,
        }
    }
}

fn env_value(keys: &[&str], default: &str) -> String {
    keys.iter()
        .find_map(|key| std::env::var(key).ok())
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_declares_all_required_observability_dimensions() {
        let dimensions = GatewayRuntimeConfig::test().metrics_dimensions;
        assert_eq!(dimensions.service, SERVICE_NAME);
        assert_eq!(dimensions.environment, "test");
        assert_eq!(dimensions.deployment_profile, "standalone");
        assert_eq!(dimensions.runtime_target, "server");
        assert_eq!(dimensions.runtime_profile, "postgresql");
    }
}
