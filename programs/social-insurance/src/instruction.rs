use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    program_error::ProgramError,
};

pub enum SocialInsuranceInstruction {
    Initialize {
        policy_holder: Pubkey,
        beneficiary: Pubkey,
        monthly_contribution: u64,
        withdrawal_period: i64,
        monthly_withdrawal: u64,
    },
    Contribute {
        amount: u64,
    },
    Withdraw,
}

impl SocialInsuranceInstruction {
    pub fn pack(&self) -> Vec<u8> {
        // Implement serialization logic here
        unimplemented!()
    }

    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // Implement deserialization logic here
        unimplemented!()
    }
}