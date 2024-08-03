use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};


// join a table
pub fn join_table(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// leave a table
pub fn leave_table(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// place a bet for the current round
pub fn bet(__accounts: &[AccountInfo], __data: &[u8]) -> ProgramResult {
  Ok(())
}

// request a new card on player hand
pub fn hit(__accounts: &[AccountInfo]) -> ProgramResult {
  Ok(())
}

// refuse a new card on player hand
pub fn stand(__accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  Ok(())
}

// double the current bet on player hand
pub fn double(__accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  Ok(())
}