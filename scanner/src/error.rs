use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error{
    #[error("Usage: scanner <target.com>")]
    CliUsage,
}