#[derive(Debug)]
pub enum OracleError {
    UnknownTarget,
    ConfigError,
    BuildFailed,
    SshConnFailed,
    SshCloseFailed,
    SshCommandFailed,
    BinaryCopyFailed,
    InvalidClientHost,
    InvalidServerHost,
    NotImplemented,
}
