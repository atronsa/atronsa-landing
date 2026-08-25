/// No real SMS gateway is wired in yet (see `shared::sms`) — `from_number`
/// only stands in as the sender identity a future provider integration
/// would use, and is logged rather than dialed out today.
#[derive(Debug, Clone)]
pub struct SmsConfig {
    pub from_number: String,
}

impl SmsConfig {
    pub fn init() -> Self {
        let from_number =
            std::env::var("SMS_FROM_NUMBER").unwrap_or_else(|_| "+0000000000".to_string());

        SmsConfig { from_number }
    }
}
