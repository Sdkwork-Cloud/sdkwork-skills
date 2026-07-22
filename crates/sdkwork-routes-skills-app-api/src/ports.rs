use async_trait::async_trait;
use sdkwork_skills_contract::SkillInstallationSubjectKind;

#[derive(Debug, Clone)]
pub struct SkillsAppRequestContext {
    pub tenant_id: u64,
    pub actor_id: u64,
    pub organization_id: u64,
}

#[async_trait]
pub trait SkillInstallationTargetAuthorizer: Send + Sync {
    async fn authorize(
        &self,
        context: &SkillsAppRequestContext,
        subject_kind: SkillInstallationSubjectKind,
        subject_id: u64,
    ) -> bool;
}

#[derive(Debug, Default)]
pub struct DenyExternalInstallationTargets;

#[async_trait]
impl SkillInstallationTargetAuthorizer for DenyExternalInstallationTargets {
    async fn authorize(
        &self,
        _context: &SkillsAppRequestContext,
        _subject_kind: SkillInstallationSubjectKind,
        _subject_id: u64,
    ) -> bool {
        false
    }
}
