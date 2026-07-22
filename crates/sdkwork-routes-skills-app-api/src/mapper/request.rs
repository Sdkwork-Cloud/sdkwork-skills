use sdkwork_skills_contract::{SkillInstallationRecord, SkillInstallationSubjectKind};
use serde_json::Value;

use crate::SkillsAppRequestContext;

pub(crate) fn installation_record(
    context: &SkillsAppRequestContext,
    package_id: u64,
    artifact_id: u64,
    subject_kind: SkillInstallationSubjectKind,
    subject_id: u64,
    config: Value,
) -> SkillInstallationRecord {
    SkillInstallationRecord {
        id: 0,
        uuid: String::new(),
        tenant_id: context.tenant_id,
        organization_id: context.organization_id,
        subject_kind,
        subject_id,
        skill_id: 0,
        package_id,
        artifact_id,
        installed_by_user_id: context.actor_id,
        install_status: "installed".to_string(),
        enabled: true,
        config,
        version: 0,
        installed_at: String::new(),
        updated_at: String::new(),
    }
}
