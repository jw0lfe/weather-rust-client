//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    username: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Greeting {
    greeting: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "access-token")]
    pub(crate) access_token: String,
    expires: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}
