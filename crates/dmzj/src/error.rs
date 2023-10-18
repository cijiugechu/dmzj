use snafu::prelude::*;

pub type DmzjResult<T> = Result<T, DmzjError>;

/// Dmzj errors
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DmzjError {
    /// An error that can occur while requesting data from dmzj
    #[snafu(display("Failed to request: {}", source))]
    Request { source: reqwest_middleware::Error },
    /// An error that can occur while parsing the response
    #[snafu(display("Failed to parse response: {}", source))]
    Parse { source: reqwest::Error },
    /// An error that can occur while decrypting the response
    #[snafu(display("Failed to decrypt response: {}", source))]
    Decrypt { source: rsa::Error },
    /// An error that can occur while parsing protobuf
    #[snafu(display("Failed to parse protobuf: {}", source))]
    ProtoBuf { source: protobuf::Error },
}
