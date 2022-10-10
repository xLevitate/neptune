use thiserror::Error;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Test Error")]
    TestError,
}
