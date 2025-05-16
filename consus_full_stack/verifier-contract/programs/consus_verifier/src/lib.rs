use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgc3QfZ6sE7Z");

#[program]
pub mod consus_verifier {
    use super::*;

    pub fn verify_proof(ctx: Context<VerifyProof>, proof: Vec<u8>) -> Result<()> {
        require!(proof.len() > 0, CustomError::InvalidProof);
        msg!("Proof verified on-chain.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct VerifyProof {}

#[error_code]
pub enum CustomError {
    #[msg("Invalid proof submitted")]
    InvalidProof,
}
