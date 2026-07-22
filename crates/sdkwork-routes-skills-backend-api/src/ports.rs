#[derive(Debug, Clone)]
pub struct SkillsBackendRequestContext {
    pub tenant_id: u64,
    pub organization_id: u64,
    pub operator_id: u64,
}
