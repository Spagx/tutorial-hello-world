use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(hello_world);

pub fn hello_world(
    _program_id: &Pubkey,
    accounts: &[AccountInfo], 
    _instruction_data: &[u8], 
) -> ProgramResult {
    msg!("Hello {:}!!", accounts[0].key);
    Ok(())
}
