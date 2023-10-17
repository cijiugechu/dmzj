use snafu::prelude::*;

pub type DmzjResult<T> = Result<T, DmzjError>;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum DmzjError {
    #[snafu(display("Failed to request: {}", source))]
    Request { source: reqwest_middleware::Error },
    #[snafu(display("Failed to parse response: {}", source))]
    Parse { source: reqwest::Error },
    #[snafu(display("Failed to decrypt response: {}", source))]
    Decrypt { source: rsa::Error },
    #[snafu(display("Failed to parse protobuf: {}", source))]
    ProtoBuf { source: protobuf::Error },
}
