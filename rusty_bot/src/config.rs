pub struct SuiConfig {
    url: String,
}

pub struct BotConfig {
    interval: u64,
    sui: SuiConfig,
}

impl BotConfig {
    pub fn new() -> Self {
        Self {
            interval: (),
            sui: (),
        }
    }
}
