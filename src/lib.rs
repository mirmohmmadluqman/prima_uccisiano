use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let instruction_data = _instruction_data;

    match instruction_data[0] {
        0 => deposit(_program_id, _accounts, &instruction_data[1..]),
        1 => withdraw(_program_id, _accounts, &instruction_data[1..]),
        _ => Err(ProgramError::InvalidInstructionData)
    }

}

fn deposit(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Deposit");
    // TODO...
    Ok(())
}

fn withdraw(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("Withdraw");
    // TODO...
    Ok(())
} 

// Testing Git changes into study branch...

