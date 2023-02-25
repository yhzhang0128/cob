#[derive(Debug)]
pub enum OracleError {
    ConfigError,
    SshConnFailed,
    SshCloseFailed,
    SshCommandFailed
}
