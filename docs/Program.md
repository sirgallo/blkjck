# blkjck

### Program


## Overview

### Accounts

There are two basic accounts that manage state within `blkjck`, which include:

  - player
  - table

#### Table

The table account is a program derived account which contains the following struct signature:

```rs
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Table {
  pub repo_owner: Pubkey,
  pub name: String,
  pub contributors: Vec<Pubkey>,
  pub is_public: bool,
  pub data_addr: Option<String>
}
```

The public key of pda for the repository account is implicitly linked to the account and does not need to be explicitly defined in the repository account object.

available instructions are as follows:

```rs
pub fn create_repository(program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
pub fn add_contributor(accounts: &[AccountInfo]) -> ProgramResult
pub fn remove_contributor(accounts: &[AccountInfo]) -> ProgramResult
pub fn update_data_addr(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
pub fn update_is_public(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
```

#### User

The user account contains the following struct signature:

```rs
#[derive(BorshSerialize, BorshDeserialize)]
pub struct User {
  pub display_name: String,
  pub repositories: DataMap<String, Pubkey>
}

pub fn try_from_slice(mut data: &[u8]) -> std::io::Result<User> {
  User::deserialize(&mut data)
}
```

The public key of the owner of the user account is implicitly linked to the account and does not need to be explicitly defined in the user account object.

available instructions are as follows:

```rs
pub fn create_user_account(_program_id: &Pubkey, accounts: &[AccountInfo], data: &[u8]) -> ProgramResult
pub fn add_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult
pub fn remove_repository(accounts: &[AccountInfo<'_>]) -> ProgramResult
```