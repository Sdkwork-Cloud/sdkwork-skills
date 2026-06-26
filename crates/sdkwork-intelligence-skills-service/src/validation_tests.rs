#[cfg(test)]
mod tests {
    use sdkwork_skills_contract::{
        SkillInvocationKind, SkillLifecycleStatus, SkillPackageRecord, SkillVisibility,
    };

    use crate::validation;

    fn sample_package(skill_id: &str) -> SkillPackageRecord {
        SkillPackageRecord {
            id: 0,
            tenant_id: 100_001,
            organization_id: 0,
            owner_user_id: 0,
            skill_id: skill_id.to_string(),
            package_key: "demo".to_string(),
            code: "demo".to_string(),
            display_name: "Demo Skill".to_string(),
            summary: Some("summary".to_string()),
            description: None,
            invocation_kind: SkillInvocationKind::LocalWorkflow,
            package_ref: "drive://spaces/skills-dev/nodes/demo-package".to_string(),
            entrypoint: "run".to_string(),
            input_schema_json: "{}".to_string(),
            output_schema_json: "{}".to_string(),
            capability_ids: vec!["cap.demo.run".to_string()],
            categories: vec![],
            tags: vec![],
            security_profile_id: None,
            status: SkillLifecycleStatus::Active,
            visibility: SkillVisibility::Tenant,
            version: 1,
            created_at: String::new(),
            updated_at: String::new(),
            deleted_at: None,
        }
    }

    #[test]
    fn validate_skill_id_accepts_standard_prefix() {
        validation::validate_skill_id("skill.demo.run").expect("valid skill id");
    }

    #[test]
    fn validate_skill_id_rejects_non_standard_prefix() {
        let error = validation::validate_skill_id("agent.demo.run")
            .expect_err("invalid skill id must fail");
        assert!(error.to_string().contains("skill_id must match"));
    }

    #[test]
    fn validate_skill_package_record_rejects_empty_code() {
        let mut record = sample_package("skill.demo.run");
        record.code = "   ".to_string();
        let error = validation::validate_skill_package_record(&record)
            .expect_err("empty code must fail");
        assert!(error.to_string().contains("code must not be empty"));
    }

    #[test]
    fn validate_skill_package_record_rejects_invalid_schema_json() {
        let mut record = sample_package("skill.demo.run");
        record.input_schema_json = "not-json".to_string();
        validation::validate_skill_package_record(&record).expect_err("invalid json must fail");
    }
}
