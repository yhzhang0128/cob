#[derive(Debug)]
pub enum OracleError {
    UnknownTarget,
    ConfigError,
    SshConnFailed,
    SshCloseFailed,
    SshCommandFailed,
    BinaryCopyFailed,
    InvalidClientHost,
    InvalidServerHost,
}
