use {
    crate::processor,
    solana_program::{
        account_info::AccountInfo, entrypoint::ProgramResult, program_error::PrintProgramError,
        pubkey::Pubkey,
    },
    spl_transfer_hook_interface::error::TransferHookError,
};

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TransferHookError>();
        return Err(error);
    }
    Ok(())
}