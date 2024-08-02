use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  pubkey::Pubkey,
  sysvar::{rent::Rent, Sysvar}
};


// create_repository creates a new program derived account, where the data argument contains metadata like the address and visibility
pub fn create(program_id: &Pubkey, accounts: &[AccountInfo], __data: &[u8]) -> ProgramResult {
  let __rent: Rent = Rent::get()?; // system variable, get global rent val
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let table_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA
  let __owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // user's account
    
  if ! table_account.is_writable { return Err(ProgramError::InvalidAccountData); }
  if table_account.owner != program_id { return Err(ProgramError::IncorrectProgramId); }

  Ok(())
}


// update_is_public updates repository visibility. If is_public is false, only the owner and contributors can view the repository.
pub fn deal(accounts: &[AccountInfo], __data: &[u8]) -> ProgramResult {
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let __owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  Ok(())
}