use super::global;

pub mod tx;
pub mod v0;

#[derive(Debug, clap::Subcommand)]
pub enum Cmd {
    /// set options for transaction
    Tx(tx::Cmd),
    /// set options for transactionv0
    V0(v0::Cmd),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Tx(#[from] tx::Error),
    #[error(transparent)]
    V0(#[from] v0::Error),
    #[error(transparent)]
    Xdr(#[from] super::xdr::Error),
}

impl Cmd {
    pub async fn run(&self, global_args: &global::Args) -> Result<(), Error> {
        let mut tx = super::xdr::tx_envelope_from_stdin()?;
        match self {
            Cmd::Tx(cmd) => tx.run(global_args)?,
            Cmd::V0(cmd) => cmd.run(global_args)?,
        };
        Ok(())
    }
}
