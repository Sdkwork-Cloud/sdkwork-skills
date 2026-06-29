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
        Ok(())
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
