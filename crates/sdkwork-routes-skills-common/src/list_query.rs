use sdkwork_skills_contract::{SkillCategoryType, SkillInstallationSubjectKind};
use sdkwork_utils_rust::{validated_offset_list_params, OffsetListPageParams};
use serde::Deserialize;

use crate::response::ApiProblem;

/// Standard GET list query parameters (`API_SPEC.md` §14.1).
#[derive(Debug, Default, Deserialize)]
pub struct SdkWorkListQuery {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub cursor: Option<String>,
    pub q: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
pub struct SkillInstallationListQuery {
    #[serde(flatten)]
    pub pagination: SdkWorkListQuery,
    pub subject_kind: Option<SkillInstallationSubjectKind>,
    #[serde(default, with = "sdkwork_utils_rust::serde_uint64::option")]
    pub subject_id: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
pub struct SkillCategoryListQuery {
    #[serde(flatten)]
    pub pagination: SdkWorkListQuery,
    pub category_type: Option<SkillCategoryType>,
}

impl SdkWorkListQuery {
    pub fn validate(&self) -> Result<(), ApiProblem> {
        let cursor = self
            .cursor
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());
        if cursor.is_some() {
            return Err(ApiProblem::bad_request(
                "cursor pagination is not supported yet; use page and page_size",
            ));
        }
        self.offset_params().map(|_| ())
    }

    pub fn offset_params(&self) -> Result<OffsetListPageParams, ApiProblem> {
        validated_offset_list_params(self.page.map(i64::from), self.page_size.map(i64::from))
            .map_err(|_| {
                ApiProblem::bad_request("page must be >= 1 and page_size must be between 1 and 200")
            })
    }

    pub fn effective_page_size(&self) -> i32 {
        self.page_size.unwrap_or(20).clamp(1, 200)
    }

    pub fn effective_page(&self) -> i32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn search_keyword(&self) -> Option<&str> {
        self.q
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }
}
