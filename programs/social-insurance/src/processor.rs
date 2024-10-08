use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    clock::Clock,
    sysvar::Sysvar,
    program_error::ProgramError,
};

use crate::{
    instruction::SocialInsuranceInstruction,
    state::InsuranceAccount,
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = SocialInsuranceInstruction::unpack(instruction_data)?;

        match instruction {
            SocialInsuranceInstruction::Initialize { policy_holder, beneficiary, monthly_contribution, withdrawal_period, monthly_withdrawal } => {
                Self::process_initialize(accounts, policy_holder, beneficiary, monthly_contribution, withdrawal_period, monthly_withdrawal)
            },
            SocialInsuranceInstruction::Contribute { amount } => {
                Self::process_contribute(accounts, amount)
            },
            SocialInsuranceInstruction::Withdraw => {
                Self::process_withdraw(accounts)
            },
        }
    }

    fn process_initialize(
        accounts: &[AccountInfo],
        policy_holder: Pubkey,
        beneficiary: Pubkey,
        monthly_contribution: u64,
        withdrawal_period: i64,
        monthly_withdrawal: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let insurance_account = next_account_info(account_info_iter)?;
        let user = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        let mut insurance_account_data = InsuranceAccount::unpack_unchecked(&insurance_account.data.borrow())?;
        insurance_account_data.policy_holder = policy_holder;
        insurance_account_data.beneficiary = beneficiary;
        insurance_account_data.monthly_contribution = monthly_contribution;
        insurance_account_data.withdrawal_period = withdrawal_period;
        insurance_account_data.monthly_withdrawal = monthly_withdrawal;
        insurance_account_data.balance = 0;
        insurance_account_data.last_contribution = Clock::get()?.unix_timestamp;
        insurance_account_data.withdrawal_start = Clock::get()?.unix_timestamp + withdrawal_period;
        
        InsuranceAccount::pack(insurance_account_data, &mut insurance_account.data.borrow_mut())?;

        Ok(())
    }

    fn process_contribute(accounts: &[AccountInfo], amount: u64) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let insurance_account = next_account_info(account_info_iter)?;
        let policy_holder = next_account_info(account_info_iter)?;

        let mut insurance_account_data = InsuranceAccount::unpack(&insurance_account.data.borrow())?;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time >= insurance_account_data.withdrawal_start {
            return Err(ProgramError::Custom(1)); // WithdrawalPeriodStarted
        }

        insurance_account_data.balance += amount;
        insurance_account_data.last_contribution = current_time;

        InsuranceAccount::pack(insurance_account_data, &mut insurance_account.data.borrow_mut())?;

        Ok(())
    }

    fn process_withdraw(accounts: &[AccountInfo]) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let insurance_account = next_account_info(account_info_iter)?;
        let beneficiary = next_account_info(account_info_iter)?;

        let mut insurance_account_data = InsuranceAccount::unpack(&insurance_account.data.borrow())?;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time < insurance_account_data.withdrawal_start {
            return Err(ProgramError::Custom(2)); // WithdrawalPeriodNotStarted
        }

        let months_since_last_withdrawal = (current_time - insurance_account_data.last_withdrawal) / (30 * 24 * 60 * 60);
        let withdrawal_amount = insurance_account_data.monthly_withdrawal.saturating_mul(months_since_last_withdrawal as u64);

        if withdrawal_amount == 0 {
            return Err(ProgramError::Custom(3)); // NoFundsToWithdraw
        }

        if insurance_account_data.balance < withdrawal_amount {
            return Err(ProgramError::Custom(4)); // InsufficientFunds
        }

        insurance_account_data.balance -= withdrawal_amount;
        insurance_account_data.last_withdrawal = current_time;

        InsuranceAccount::pack(insurance_account_data, &mut insurance_account.data.borrow_mut())?;

        // Transfer funds logic here

        Ok(())
    }
}