use snafu::prelude::*;

pub type DmzjResult<T> = Result<T, DmzjError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DmzjError {
    #[snafu(display("Failed to request {}: {}", url, source))]
    Request {
        url: String,
        source: reqwest_middleware::Error,
    },
    #[snafu(display("Failed to parse response {}: {}", url, source))]
    Parse { url: String, source: reqwest::Error },
    #[snafu(display("Failed to decrypt response: {}", source))]
    Decrypt { source: rsa::Error },
}
