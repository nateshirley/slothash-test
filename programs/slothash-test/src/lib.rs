use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_lang::solana_program::sysvar::SysvarId;
use anchor_lang::solana_program::hash::Hash;

declare_id!("Cp7p6JK7mBz6txjitU9bYfGup3wboiXoqvpwnXtEU9Yy");
//better to run with skip-local-validator
#[program]
pub mod slothash_test {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let data = ctx.accounts.slot_hashes.try_borrow_data()?;
        let num_slot_hashes = u64::from_le_bytes(data[0..8].try_into().unwrap());
        let hashes_to_iterate: u64 = 6;
        msg!("num slot hashes: {}", num_slot_hashes);
        let mut pos = 8;
        // //haven't checked this against a network
        // //https://discord.com/channels/428295358100013066/517163444747894795/877969355558781008
        for _i in 0..hashes_to_iterate {
            let slot = u64::from_le_bytes(data[pos..pos + 8].try_into().unwrap());
            pos += 8;
            let hash = &data[pos..pos + 32];
            pos += 32;
            msg!("Found slot {}, hash {}", slot, Hash::new(hash));
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    system_program: Program<'info, System>,
    ///CHECK: tbd
    #[account(
        address = solana_program::slot_hashes::SlotHashes::id(),
    )]
    slot_hashes: AccountInfo<'info>,
}
