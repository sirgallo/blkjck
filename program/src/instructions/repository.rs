use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  account_info::{next_account_info, AccountInfo},
  entrypoint::ProgramResult,
  program_error::ProgramError,
  pubkey::Pubkey,
  sysvar::{rent::Rent, Sysvar}
};

use crate::instructions::user::{add_repository, remove_repository};
use crate::state::Repository;
use crate::utils::account::get_account_data_as_vec;


// create_repository creates a new program derived account, where the data argument contains metadata like the address and visibility
pub fn create_repository(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  let rent: Rent = Rent::get()?; // system variable, get global rent val

  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let repository_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // user's account
    
  if ! repository_account.is_writable { return Err(ProgramError::InvalidAccountData); }
  if repository_account.owner != program_id { return Err(ProgramError::IncorrectProgramId); }

  let is_public: bool = match data.get(0) {
    Some(&1) => true,
    Some(_) => false,
    None => return Err(ProgramError::InvalidInstructionData),
  };

  let name: String = match std::str::from_utf8(&data[1..]) {
    Ok(name) => name.to_string(),
    Err(_err) => return Err(ProgramError::InvalidInstructionData)
  };

  let seeds: &[&[u8]; 2] = &[b"repository", owner_account.key.as_ref()]; // initialize a PDA for the repository
  let (repository_pda_key, _bump_seed) = Pubkey::find_program_address(seeds, program_id);
  
  if repository_account.key != &repository_pda_key { return Err(ProgramError::InvalidSeeds); }

  let repository: Repository = Repository { 
    repo_owner: *owner_account.key, 
    name,
    is_public, 
    data_addr: None, // initialize data_addr after repository initialization
    contributors: Vec::new()
  };
  
  if **repository_account.lamports.borrow() < rent.minimum_balance(repository_account.data_len()) {
    return Err(ProgramError::InsufficientFunds); // ensure the account has enough lamports to be rent-exempt
  }

  match serialize_repository(&repository, repository_account) {
    Ok(_) => add_repository(&[ owner_account.clone(), repository_account.clone() ]),
    Err(err) => Err(err)
  }
}


// add_contributor adds new users as writeable users to the chosen repository
pub fn add_contributor(accounts: &[AccountInfo]) -> ProgramResult { // just owner should be able to add contributors
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let repository_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account
  let contributor_account = next_account_info(accounts_iter)?; // contributor to add account

  let repository_account_vec = get_account_data_as_vec(repository_account)?;
  let mut repository: Repository = Repository::deserialize(&mut &repository_account_vec[..])?;

  if repository.repo_owner != *owner_account.key { return Err(ProgramError::IllegalOwner); } // ensure the current user is the repo_owner
  if ! owner_account.is_signer { return Err(ProgramError::MissingRequiredSignature); } // ensure the owner account is the signer

  match repository.contributors.contains(contributor_account.key) {
    true => Ok(()), // not an error, just means contributor already exists
    false => {
      repository.contributors.push(*contributor_account.key); // add new contributor
      add_repository(&[ owner_account.to_owned(), contributor_account.to_owned() ])?;
      serialize_repository(&repository, &repository_account)
    }
  }
}


// remove_contributor removes an existing contributor from the chosen repository
pub fn remove_contributor(accounts: &[AccountInfo]) -> ProgramResult { // we want both contributor to be removed and owner to be able to remove contributor
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();

  let repository_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account
  let contributor_account = next_account_info(accounts_iter)?; 

  let repository_account_vec = get_account_data_as_vec(repository_account)?;
  let mut repository: Repository = Repository::deserialize(&mut &repository_account_vec[..])?;

  if repository.repo_owner != *owner_account.key { return Err(ProgramError::IllegalOwner); } // ensure the current user is the repo_owner
  if ! owner_account.is_signer || ! contributor_account.is_signer { //either owner or contributor to be removed can be the signer
    return Err(ProgramError::MissingRequiredSignature)
  }

  match repository.contributors.iter().position(|x| x == contributor_account.key) { // remove the contributor if they exist in the list
    Some(index) => {
      match remove_repository(&[ contributor_account.to_owned(), repository_account.to_owned() ]) {
        Ok(_) => { 
          repository.contributors.remove(index);
          serialize_repository(&repository, &repository_account)
        },
        Err(err) => Err(err)
      }
    },
    None => Ok(()) // just means contributor already has been removed or does not exist
  }
}


// update_data_addr updates the link to the location of the repository. Is not initialized on initial repository creation, but needs to be done when a link is generated.
pub fn update_data_addr(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  match deserialize_repository(accounts_iter) {
    Ok((mut repository, account_info)) => {
      if repository.repo_owner != *owner_account.key { return Err(ProgramError::IllegalOwner); } // ensure the current user is the repo_owner
      if ! owner_account.is_signer { return Err(ProgramError::MissingRequiredSignature); }

      let data_addr: &str = std::str::from_utf8(&data[0..])
        .map_err(|_| ProgramError::InvalidInstructionData)?;
      
      repository.data_addr = Some(data_addr.to_string());
      serialize_repository(&repository, &account_info)
    },
    Err(err) => return Err(err)
  }
}


// update_is_public updates repository visibility. If is_public is false, only the owner and contributors can view the repository.
pub fn update_is_public(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
  let accounts_iter: &mut std::slice::Iter<'_, AccountInfo<'_>> = &mut accounts.iter();
  let owner_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // owner's account

  match deserialize_repository(accounts_iter) {
    Ok((mut repository, account_info)) => {
      if repository.repo_owner != *owner_account.key { return Err(ProgramError::IllegalOwner); } // ensure the current user is the repo_owner
      if ! owner_account.is_signer { return Err(ProgramError::MissingRequiredSignature); }

      let is_public: bool = match data.get(0) {
        Some(&1) => true,
        Some(_) => false,
        None => return Err(ProgramError::InvalidInstructionData),
      };

      repository.is_public = is_public;
      serialize_repository(&repository, &account_info)
    },
    Err(err) => return Err(err)
  }
}


// deserialize_repository is a helper function for deserializing account data using Borsh Serialize on the incoming account data
fn deserialize_repository<'a, 'b>(accounts_iter: &'a mut std::slice::Iter<'_, AccountInfo<'b>>) -> Result<(Repository, &'a AccountInfo<'b>), ProgramError> {
  let repository_account: &AccountInfo<'_> = next_account_info(accounts_iter)?; // PDA

  let repository_account_vec = get_account_data_as_vec(repository_account)?;
  let mut account_buffer = &repository_account_vec[..];

  let repository: Repository = Repository::deserialize(&mut account_buffer)
    .map_err(|err| ProgramError::BorshIoError(err.to_string()))?;
   
  Ok((repository, repository_account))
}


// serialize_repository is a helper function for serializing account data using Borsh Serialize on the incoming account data
fn serialize_repository(repository: &Repository, repository_account: &AccountInfo<'_>) -> ProgramResult {
  let mut data = &mut repository_account.data.borrow_mut()[..]; // extract the account data
  
  repository.serialize(&mut data)
    .map_err(|err| ProgramError::BorshIoError(err.to_string()))?;

  Ok(())
}