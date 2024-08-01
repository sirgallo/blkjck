use solana_program::{
  account_info::AccountInfo,
  entrypoint::ProgramResult,
  pubkey::Pubkey
};

// process_instruction is the entrypoint for the smart contract
pub fn process_instruction<'a>(__program_id: &Pubkey, __accounts: &'a [AccountInfo<'a>], __instruction_data: &[u8]) -> ProgramResult {
  Ok(())
}