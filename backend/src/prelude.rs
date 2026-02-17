pub(crate) use crate::application::app_error::AppError;
pub type Result<T> = core::result::Result<T, AppError>;

//Generic Wrapper tuple struct for newtype pattern
pub struct W<T>(pub T);