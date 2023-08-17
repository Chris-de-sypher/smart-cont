use anchor_lang::prelude::*;
use solana_program::system_instruction;

#[program]
mod deploy_kallo_view_registry {
    use super::*;

    pub fn set_up(ctx: Context<SetUp>) -> ProgramResult {
        Ok(())
    }

    pub fn run(ctx: Context<Run>) -> ProgramResult {
        let deployer_private_key: [u8; 32] = ctx.accounts.deployer.key();

        let lamports = 1000; // Initial lamports for the account
        let size = solana_program::borsh::get_packed_len::<KalloViewRegistry>() as u64;

        let create_account_instruction = system_instruction::create_account(
            &ctx.accounts.deployer.key(),
            &ctx.accounts.kallo.to_account_info().key,
            lamports,
            size,
            &kallo_view_registry::id(),
        );

        // Create a transaction and add instructions to it
        let mut transaction = Transaction::new_with_payer(
            &[create_account_instruction],
            Some(&ctx.accounts.deployer.key()),
        );

        // Sign and send the transaction
        transaction.sign(&[&ctx.accounts.deployer]);

        let recent_blockhash = ctx.accounts.deployer.recent_blockhash();
        let result = ctx.accounts.deployer.send_transaction(transaction, recent_blockhash);

        if result.is_ok() {
            // Transaction was sent successfully
            let transaction_result = result.unwrap();
            if transaction_result.status != Ok(()) {
            return Err(ProgramError::Custom(0));
        } else {
            // Handle error
            return Err(ProgramError::Custom(1));
        }

        let kallo = KalloViewRegistry::create(&mut ctx.accounts.kallo)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetUp {}

#[derive(Accounts)]
pub struct Run<'info> {
    #[account(signer)]
    pub deployer: AccountInfo<'info>,
    pub system_program: AccountInfo<'info>,
    pub kallo: Loader<'info, KalloViewRegistry>,
}

#[account]
pub struct KalloViewRegistry {}

impl KalloViewRegistry {
    pub fn create(ctx: &mut Context<Run>) -> Result<Self, ProgramError> {
        let kallo = KalloViewRegistry {};
        // Copy data into the account's data field using Solana's methods
        ctx.accounts.kallo.try_borrow_mut_data()?.copy_from_slice(&kallo.try_to_vec()?);
        Ok(kallo)
    }
}