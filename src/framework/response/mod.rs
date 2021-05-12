mod apifail;
pub use apifail::*;

/// Some endpoints return nothing. That's OK.
impl ApiResult for () {}

pub type ApiResponse<ResultType> = eyre::Result<ResultType>;
