use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  pubkey::Pubkey
};

use crate::instructions::repository::{
  create_repository,
  add_contributor,
  remove_contributor,
  update_data_addr,
  update_is_public
};
use crate::instructions::player::{
  create_user_account,
  add_repository,
  remove_repository
};


#[derive(BorshSerialize, BorshDeserialize)]
pub enum AthnInstruction {
  // repository account instructions
  CreateRepository(Vec<u8>),
  AddContributor,
  RemoveContributor,
  UpdateDataAddr(Vec<u8>),
  UpdateIsPublic(Vec<u8>),

  // user account instructions
  CreateUser(Vec<u8>),
  AddRepository,
  RemoveRepository
}


// process_instruction is the entrypoint for the smart contract
pub fn process_instruction<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>], instruction_data: &[u8]) -> ProgramResult {
  let instruction = AthnInstruction::try_from_slice(instruction_data)?;
  match instruction {
    // repository account instructions
    AthnInstruction::CreateRepository(data) => create_repository(program_id, accounts, &data),
    AthnInstruction::AddContributor => add_contributor(accounts),
    AthnInstruction::RemoveContributor =>  remove_contributor(accounts),
    AthnInstruction::UpdateDataAddr(data) => update_data_addr(accounts, &data),
    AthnInstruction::UpdateIsPublic(data) => update_is_public(accounts, &data),
    
    // user account instructions
    AthnInstruction::CreateUser(data) => create_user_account(program_id, accounts, &data),
    AthnInstruction::AddRepository => add_repository(accounts),
    AthnInstruction::RemoveRepository => remove_repository(accounts)
  }
}