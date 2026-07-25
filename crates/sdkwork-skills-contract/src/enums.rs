use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SkillInvocationKind {
    LocalWorkflow,
    ProcessAdapter,
    McpTool,
    KernelProvider,
}

impl SkillInvocationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkflow => "local-workflow",
            Self::ProcessAdapter => "process-adapter",
            Self::McpTool => "mcp-tool",
            Self::KernelProvider => "kernel-provider",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "local-workflow" => Some(Self::LocalWorkflow),
            "process-adapter" => Some(Self::ProcessAdapter),
            "mcp-tool" => Some(Self::McpTool),
            "kernel-provider" => Some(Self::KernelProvider),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillLifecycleStatus {
    Draft,
    Active,
    Disabled,
    Archived,
    Deleted,
}

impl SkillLifecycleStatus {
    pub fn as_db_code(self) -> i16 {
        match self {
            Self::Draft => 0,
            Self::Active => 1,
            Self::Disabled => 2,
            Self::Archived => 3,
            Self::Deleted => 4,
        }
    }

    pub fn from_db_code(value: i16) -> Option<Self> {
        match value {
            0 => Some(Self::Draft),
            1 => Some(Self::Active),
            2 => Some(Self::Disabled),
            3 => Some(Self::Archived),
            4 => Some(Self::Deleted),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillVisibility {
    Private,
    Tenant,
    Organization,
    Public,
}

impl SkillVisibility {
    pub fn as_db_code(self) -> i16 {
        match self {
            Self::Private => 0,
            Self::Tenant => 1,
            Self::Organization => 2,
            Self::Public => 3,
        }
    }

    pub fn from_db_code(value: i16) -> Option<Self> {
        match value {
            0 => Some(Self::Private),
            1 => Some(Self::Tenant),
            2 => Some(Self::Organization),
            3 => Some(Self::Public),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SkillCategoryType {
    SkillMarket,
    SkillsCollection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillArtifactStatus {
    Draft,
    Published,
    Yanked,
}

impl SkillArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Published => "published",
            Self::Yanked => "yanked",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "draft" => Some(Self::Draft),
            "published" => Some(Self::Published),
            "yanked" => Some(Self::Yanked),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillInstallationSubjectKind {
    User,
    Organization,
    Project,
    Agent,
}

impl SkillInstallationSubjectKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::Organization => "organization",
            Self::Project => "project",
            Self::Agent => "agent",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "user" => Some(Self::User),
            "organization" => Some(Self::Organization),
            "project" => Some(Self::Project),
            "agent" => Some(Self::Agent),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SkillCapabilityRiskLevel {
    Standard,
    Sensitive,
    Privileged,
}

impl SkillCapabilityRiskLevel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Sensitive => "sensitive",
            Self::Privileged => "privileged",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "standard" => Some(Self::Standard),
            "sensitive" => Some(Self::Sensitive),
            "privileged" => Some(Self::Privileged),
            _ => None,
        }
    }
}

impl SkillCategoryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SkillMarket => "skill_market",
            Self::SkillsCollection => "skills_collection",
        }
    }
}
