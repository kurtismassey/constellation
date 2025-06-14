pub struct Settings {
    pub host: String,
    pub port: u16,
    pub openai_api_key: String,
    pub google_search_cx: String,
    pub google_api_key: String,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number"),
            openai_api_key: std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"),
            google_search_cx: std::env::var("GOOGLE_SEARCH_CX")
                .expect("GOOGLE_SEARCH_CX must be set"),
            google_api_key: std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set"),
        }
    }
}
