use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};


// add_contributor adds new users as writeable users to the chosen repository
pub fn join(__accounts: &[AccountInfo]) -> ProgramResult { // just owner should be able to add contributors
  Ok(())
}

// remove_contributor removes an existing contributor from the chosen repository
pub fn leave(__accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  Ok(())
}