pub mod rpc;

#[derive(Debug, thiserror::Error)]
pub enum ClaraError {
    #[error(transparent)]
    ClientError(#[from] jsonrpsee::core::ClientError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl From<ClaraError> for jsonrpsee::types::ErrorObject<'static> {
    fn from(error: ClaraError) -> Self {
        jsonrpsee::types::ErrorObject::owned(-32000, error.to_string(), None::<()>)
    }
}
