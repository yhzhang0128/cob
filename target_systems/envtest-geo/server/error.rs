#[derive(Debug)]
pub enum EnvTestError {
    ConfigError,
    FileOpError,
    TcpReadError,
    TcpWriteError,
}
