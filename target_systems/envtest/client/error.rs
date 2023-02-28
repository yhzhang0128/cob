#[derive(Debug)]
pub enum EnvTestError {
    ConfigError,
    FileOpError,
    TcpConnError,
    TcpReadError,
    SigTermHandlerError,
}

