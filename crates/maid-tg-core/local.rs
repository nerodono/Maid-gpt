struct GetUpdatesRequest<'client> {
    client: &'client reqwest::Client,
    url: reqwest::Url,
    pub offset: i64,
    pub timeout: Option<std::num::NonZeroU8>,
    pub sex: Option<i32>,
    pub limit: std::num::NonZeroU8,
    pub allowed_updates: Vec<AllowedUpdate>,
}

impl GetUpdatesRequest {
}

