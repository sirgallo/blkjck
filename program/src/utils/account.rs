use solana_program::{
  account_info::AccountInfo,
  program_error:: ProgramError
};


// get_account_data_as_buffer gets the account metadata as well as the account info as a byte slice
pub fn get_account_data_as_vec<'a>(account: &'a AccountInfo) -> Result<Vec<u8>, ProgramError> {
  match account.data_is_empty() {
    true => return Err(ProgramError::AccountDataTooSmall),
    false => {
      let account_ref = account.data.borrow();
      Ok(account_ref[..].to_vec())
    }
  }
}