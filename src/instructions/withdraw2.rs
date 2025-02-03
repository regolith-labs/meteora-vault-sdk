//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Withdraw2 {
    pub vault: solana_program::pubkey::Pubkey,

    pub token_vault: solana_program::pubkey::Pubkey,

    pub lp_mint: solana_program::pubkey::Pubkey,

    pub user_token: solana_program::pubkey::Pubkey,

    pub user_lp: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub token_program: solana_program::pubkey::Pubkey,
}

impl Withdraw2 {
    pub fn instruction(
        &self,
        args: Withdraw2InstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: Withdraw2InstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.token_vault,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lp_mint,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_token,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_lp,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.user, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.token_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = Withdraw2InstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::VAULT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Withdraw2InstructionData {
    discriminator: [u8; 8],
}

impl Withdraw2InstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [80, 6, 111, 73, 174, 211, 66, 132],
        }
    }
}

impl Default for Withdraw2InstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Withdraw2InstructionArgs {
    pub unmint_amount: u64,
    pub min_out_amount: u64,
}

/// Instruction builder for `Withdraw2`.
///
/// ### Accounts:
///
///   0. `[writable]` vault
///   1. `[writable]` token_vault
///   2. `[writable]` lp_mint
///   3. `[writable]` user_token
///   4. `[writable]` user_lp
///   5. `[signer]` user
///   6. `[optional]` token_program (default to `TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA`)
#[derive(Clone, Debug, Default)]
pub struct Withdraw2Builder {
    vault: Option<solana_program::pubkey::Pubkey>,
    token_vault: Option<solana_program::pubkey::Pubkey>,
    lp_mint: Option<solana_program::pubkey::Pubkey>,
    user_token: Option<solana_program::pubkey::Pubkey>,
    user_lp: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    token_program: Option<solana_program::pubkey::Pubkey>,
    unmint_amount: Option<u64>,
    min_out_amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl Withdraw2Builder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn token_vault(&mut self, token_vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_vault = Some(token_vault);
        self
    }
    #[inline(always)]
    pub fn lp_mint(&mut self, lp_mint: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lp_mint = Some(lp_mint);
        self
    }
    #[inline(always)]
    pub fn user_token(&mut self, user_token: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_token = Some(user_token);
        self
    }
    #[inline(always)]
    pub fn user_lp(&mut self, user_lp: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_lp = Some(user_lp);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    /// `[optional account, default to 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA']`
    #[inline(always)]
    pub fn token_program(&mut self, token_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn unmint_amount(&mut self, unmint_amount: u64) -> &mut Self {
        self.unmint_amount = Some(unmint_amount);
        self
    }
    #[inline(always)]
    pub fn min_out_amount(&mut self, min_out_amount: u64) -> &mut Self {
        self.min_out_amount = Some(min_out_amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Withdraw2 {
            vault: self.vault.expect("vault is not set"),
            token_vault: self.token_vault.expect("token_vault is not set"),
            lp_mint: self.lp_mint.expect("lp_mint is not set"),
            user_token: self.user_token.expect("user_token is not set"),
            user_lp: self.user_lp.expect("user_lp is not set"),
            user: self.user.expect("user is not set"),
            token_program: self.token_program.unwrap_or(solana_program::pubkey!(
                "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
            )),
        };
        let args = Withdraw2InstructionArgs {
            unmint_amount: self
                .unmint_amount
                .clone()
                .expect("unmint_amount is not set"),
            min_out_amount: self
                .min_out_amount
                .clone()
                .expect("min_out_amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `withdraw2` CPI accounts.
pub struct Withdraw2CpiAccounts<'a, 'b> {
    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_lp: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `withdraw2` CPI instruction.
pub struct Withdraw2Cpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub lp_mint: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_token: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_lp: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub token_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: Withdraw2InstructionArgs,
}

impl<'a, 'b> Withdraw2Cpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: Withdraw2CpiAccounts<'a, 'b>,
        args: Withdraw2InstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            vault: accounts.vault,
            token_vault: accounts.token_vault,
            lp_mint: accounts.lp_mint,
            user_token: accounts.user_token,
            user_lp: accounts.user_lp,
            user: accounts.user,
            token_program: accounts.token_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(7 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.token_vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lp_mint.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_token.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_lp.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.user.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.token_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = Withdraw2InstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::VAULT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(8 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.token_vault.clone());
        account_infos.push(self.lp_mint.clone());
        account_infos.push(self.user_token.clone());
        account_infos.push(self.user_lp.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.token_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Withdraw2` via CPI.
///
/// ### Accounts:
///
///   0. `[writable]` vault
///   1. `[writable]` token_vault
///   2. `[writable]` lp_mint
///   3. `[writable]` user_token
///   4. `[writable]` user_lp
///   5. `[signer]` user
///   6. `[]` token_program
#[derive(Clone, Debug)]
pub struct Withdraw2CpiBuilder<'a, 'b> {
    instruction: Box<Withdraw2CpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> Withdraw2CpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(Withdraw2CpiBuilderInstruction {
            __program: program,
            vault: None,
            token_vault: None,
            lp_mint: None,
            user_token: None,
            user_lp: None,
            user: None,
            token_program: None,
            unmint_amount: None,
            min_out_amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn token_vault(
        &mut self,
        token_vault: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_vault = Some(token_vault);
        self
    }
    #[inline(always)]
    pub fn lp_mint(
        &mut self,
        lp_mint: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lp_mint = Some(lp_mint);
        self
    }
    #[inline(always)]
    pub fn user_token(
        &mut self,
        user_token: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_token = Some(user_token);
        self
    }
    #[inline(always)]
    pub fn user_lp(
        &mut self,
        user_lp: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_lp = Some(user_lp);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn token_program(
        &mut self,
        token_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.token_program = Some(token_program);
        self
    }
    #[inline(always)]
    pub fn unmint_amount(&mut self, unmint_amount: u64) -> &mut Self {
        self.instruction.unmint_amount = Some(unmint_amount);
        self
    }
    #[inline(always)]
    pub fn min_out_amount(&mut self, min_out_amount: u64) -> &mut Self {
        self.instruction.min_out_amount = Some(min_out_amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = Withdraw2InstructionArgs {
            unmint_amount: self
                .instruction
                .unmint_amount
                .clone()
                .expect("unmint_amount is not set"),
            min_out_amount: self
                .instruction
                .min_out_amount
                .clone()
                .expect("min_out_amount is not set"),
        };
        let instruction = Withdraw2Cpi {
            __program: self.instruction.__program,

            vault: self.instruction.vault.expect("vault is not set"),

            token_vault: self
                .instruction
                .token_vault
                .expect("token_vault is not set"),

            lp_mint: self.instruction.lp_mint.expect("lp_mint is not set"),

            user_token: self.instruction.user_token.expect("user_token is not set"),

            user_lp: self.instruction.user_lp.expect("user_lp is not set"),

            user: self.instruction.user.expect("user is not set"),

            token_program: self
                .instruction
                .token_program
                .expect("token_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct Withdraw2CpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lp_mint: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_token: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_lp: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    token_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    unmint_amount: Option<u64>,
    min_out_amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
