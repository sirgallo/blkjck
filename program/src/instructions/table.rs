use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  pubkey::Pubkey,
  sysvar::{rent::Rent, Sysvar}
};


// create a new program derived account, where the data argument contains metadata for the table
pub fn open(program_id: &Pubkey, accounts: &[AccountInfo], __data: &[u8]) -> ProgramResult {
  let __rent: Rent = Rent::get()?; // system variable, get global rent val
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let table_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA
  let __owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // user's account
    
  if ! table_account.is_writable { return Err(ProgramError::InvalidAccountData); }
  if table_account.owner != program_id { return Err(ProgramError::IncorrectProgramId); }

  Ok(())
}

// make a table unusable
pub fn close(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// start a round of blackjack
pub fn start_round(accounts: &[AccountInfo]) -> ProgramResult {
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let __owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  Ok(())
}

// finish the round of blackjack
pub fn finish_round(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// payout winners from the liquidity pool according to the size of their bets
pub fn payout(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// collect bets from losing players and deposit into the liquidity pool
pub fn collect(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}