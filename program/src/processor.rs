use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  pubkey::Pubkey
};

use crate::instructions;


#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instruction {
  // table account instructions
  Open(Vec<u8>),
  Close,
  StartRound,
  FinishRound,
  Payout,
  Collect,
  
  // player account instructions
  Join,
  Leave,
  Bet(Vec<u8>),
  Hit,
  Stand,
  Double
}


// process_instruction is the entrypoint for the smart contract
pub fn process_instruction<'a>(program_id: &Pubkey, accounts: &'a [AccountInfo<'a>], instruction_data: &[u8]) -> ProgramResult {
  let instruction = Instruction::try_from_slice(instruction_data)?;
  match instruction {
    Instruction::Open(data) => instructions::table::open(program_id, accounts, &data),
    Instruction::Close => instructions::table::close(accounts),
    Instruction::StartRound => instructions::table::start_round(accounts),
    Instruction::FinishRound => instructions::table::finish_round(accounts),
    Instruction::Payout => instructions::table::payout(accounts),
    Instruction::Collect => instructions::table::collect(accounts),

    Instruction::Join => instructions::player::join_table(accounts),
    Instruction::Leave => instructions::player::leave_table(accounts),
    Instruction::Bet(data) => instructions::player::bet(accounts, &data),
    Instruction::Hit => instructions::player::hit(accounts),
    Instruction::Stand => instructions::player::stand(accounts),
    Instruction::Double => instructions::player::double(accounts)
  }
}