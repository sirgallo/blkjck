use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  program_error::ProgramError,
  entrypoint::ProgramResult,
  pubkey::Pubkey,
  sysvar::{rent::Rent, Sysvar}
};

use crate::state::{Repository, User};
use crate::utils::account::get_account_data_as_vec;


// create_user_account initializes a new user account that can be mapped to different repositories through a solana wallet
pub fn create_user_account(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  let _ = next_account_info(accounts_iter)?; // account to pay for the creation
  let user_account = next_account_info(accounts_iter)?; // account to be created
  let system_program = next_account_info(accounts_iter)?; // system program account

  if user_account.owner != system_program.key { return Err(ProgramError::IncorrectProgramId); }

  let user_data: User = User::try_from_slice(data)?; // deserialize input data to User
  let rent = Rent::get()?; // calculate the space and rent exemption
  let required_lamports = rent.minimum_balance(user_account.data_len());

  user_data.serialize(&mut *user_account.data.borrow_mut())?; // serialize user data into the account's data space
  **user_account.lamports.borrow_mut() = required_lamports; // funding the account to make it rent-exempt

  Ok(())
}


// add_repository adds the given repository to a user account
pub fn add_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();

  let user_account = next_account_info(accounts_iter)?; // the account of the user
  let repository_account = next_account_info(accounts_iter)?; // the repository account
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  let repository_account_vec = get_account_data_as_vec(repository_account)?;
  let repository: Repository = Repository::deserialize(&mut &repository_account_vec[..])?;

  match owner_account.is_signer { // verify that the signer is the repository owner
    true => {
      let user_account_vec = get_account_data_as_vec(user_account)?;
      let mut user_data: User = User::try_from_slice(&user_account_vec)?;
      
      if user_data.repositories.get(repository_account.key.to_string()).is_some() {
        return Err(ProgramError::Custom(0)); // repository already added, update error code later
      }
      
      match user_data.repositories.put(repository.name, repository_account.key.to_owned()) {
        Some(_) => { 
          user_data.serialize(&mut *user_account.data.borrow_mut())?;
          Ok(())
        }, // serialize the updated user data
        None => Err(ProgramError::Custom(1)) // error putting repository in btreemap
      }
    },
    false => return Err(ProgramError::IllegalOwner)
  }
}


// remove_repository removes the given repository from a user account
pub fn remove_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult {
  let accounts_iter = &mut accounts.iter();
  
  let user_account = next_account_info(accounts_iter)?;
  let repository_account = next_account_info(accounts_iter)?; // the repository account
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  let repository_account_vec = get_account_data_as_vec(repository_account)?;
  let repository: Repository = Repository::deserialize(&mut &repository_account_vec[..])?;

  if ! user_account.is_signer || ! owner_account.is_signer { // verify that the signer is the user
    return Err(ProgramError::MissingRequiredSignature);
  }

  let mut user_data: User = User::try_from_slice(&user_account.data.borrow())?;
  match user_data.repositories.del(repository.name) {
    Some(_) => { 
      user_data.serialize(&mut *user_account.data.borrow_mut())?;
      Ok(())
    },
    None => Err(ProgramError::Custom(2)) // repository not found, update error code later
  }
}