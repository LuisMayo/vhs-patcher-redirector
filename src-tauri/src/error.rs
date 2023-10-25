use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct OwnStaticError {
    pub(crate) msg: &'static str,
}
