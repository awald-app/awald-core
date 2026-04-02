use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Data error: {0}")]
    Data(#[from] awald_data::Error),

    #[error("Engine error: {0}")]
    Engine(#[from] awald_engine::Error),
}
