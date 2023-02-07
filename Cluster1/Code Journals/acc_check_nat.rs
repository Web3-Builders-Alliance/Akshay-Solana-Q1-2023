use solana_program::{
    account_info::{ AccountInfo, next_account_info }, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
};


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    // You can verify the program ID from the instruction is in fact 
    //      the program ID of your program.
    if system_program::check_id(program_id) {
        return Err(ProgramError::IncorrectProgramId)
    };
    
    // You can verify the list has the correct number of accounts.
    // This error will get thrown by default if you 
    //      try to reach past the end of the iter.
    if accounts.len() < 4 {
        msg!("This instruction requires 4 accounts:");
        msg!("  payer, account_to_create, account_to_change, system_program");
        return Err(ProgramError::NotEnoughAccountKeys)
    };

    // Accounts passed in a vector must be in the expected order.
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let account_to_create = next_account_info(accounts_iter)?;
    let account_to_change = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // You can make sure an account has NOT been initialized.
    
    msg!("New account: {}", account_to_create.key);
    if account_to_create.lamports() != 0 {
        msg!("The program expected the account to create to not yet be initialized.");
        return Err(ProgramError::AccountAlreadyInitialized)
    };
    // (Create account...)

    // You can also make sure an account has been initialized.
    msg!("Account to change: {}", account_to_change.key);
    if account_to_change.lamports() == 0 {
        msg!("The program expected the account to change to be initialized.");
        return Err(ProgramError::UninitializedAccount)
    };

    // If we want to modify an account's data, it must be owned by our program.
    if account_to_change.owner != program_id {
        msg!("Account to change does not have the correct program id.");
        return Err(ProgramError::IncorrectProgramId)
    };

    // You can also check pubkeys against constants.
    if system_program.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId)
    };

    Ok(())
}


/*CODE JOURNAL

1) What are the concepts (borrowing, ownership, vectors etc)
    iterators - next_account_info iterates through the account array and returns it
    a mutable account iterator is used in this program to assign the account info to variables 
    macro calls-like msg, entrypoint etc
    variable(with constants) declaration and mutability,
    arrays, structs,, referencing/borrowing, ownership, 
    control flow (with if statements), functions/arguements

2) What is the organization?
    The program requires you to pass the required arguments and for accounts you need to pass 
    the right number of accounts in right order

3)What is the contract doing? What is the mechanism? 
    This program is used to perform checks on insturctions passed in to ensure that the only the required
    accounts in exactly specified order is passed. 
    We are also checking the lamports of accounts to check if they're initialized or not

4)How could it be better? More efficient? Safer?
    This doesn't utilize the Rust concept of ownership
    we can make this more efficient is bundle the logic for checking the `IncorrectProgramID`
    into one logical conditional check. It is inefficient to have to go through 3 separate conditionals to check for the
    same error. We can put it into one conditional, where that conditional checks all the cases of `IncorrectProgramID
*/