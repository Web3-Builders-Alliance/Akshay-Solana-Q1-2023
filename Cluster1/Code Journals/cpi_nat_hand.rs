use borsh::BorshDeserialize;
use lever::SetPowerStatus;
use solana_program::{
    account_info::{
        next_account_info, AccountInfo
    },
    entrypoint, 
    entrypoint::ProgramResult, 
    instruction::{ AccountMeta, Instruction },
    pubkey::Pubkey,
    program::invoke,
};


entrypoint!(pull_lever);


fn pull_lever(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();
    let power = next_account_info(accounts_iter)?;
    let lever_program = next_account_info(accounts_iter)?;

    let set_power_status_instruction = SetPowerStatus::try_from_slice(instruction_data)?;

    let ix = Instruction::new_with_borsh(
        lever_program.key.clone(),                          // Our lever program's ID
        &set_power_status_instruction,                      // Passing instructions through
        vec![AccountMeta::new(power.key.clone(), false)],   // Just the required account for the other program
    );

    invoke(&ix, &[power.clone()])
}


/*
//Code Journal Summary:

1. What are the concepts (borrowing, ownership, vectors etc)?
    - Bringing paths into scope with the `use` keyword (modules), Traits, macro calls, variable declaration and mutability,
    arrays, structs, enums, STD types (iterators/vec), referencing/borrowing, ownership, functions/arguements, error handling with "?"

2. What is the organization?
    - This code is organized first by bringing the crates into scope, defining the entrypoint, then creating the `pull_lever`
    function. All the main logic is inside the pull_lever function, which is the entry point to the program. Aside from the program ID,
    this function takes in the accounts and instruction_data as well, and returns the ProgramResult Enum. Inside the function,
    the account_iter and accounts are assigned to variables, as well as the SetPowerStatus object which holds the instruction data.
    Final step is the Instruction object is created and then invoked! This is the general organization of this code.

3. What is the contract doing? What is the mechanism?
    This piece of code seems to be creating a new instructions using the lever program and invoking that
    instruction. To be more precise, it cycles over the accounts and gives the first two accounts to 
    the appropriate programmes, "power" and "lever program," respectively. The instruction data is 
    first stored in a new "SetPowerStatus" object, which is then used to build a new "Instruction" 
    object (together with power metadata and lever program key). The instruction is then finally
    executed! The purpose of this code appears to be to allow a caller to activate an instruction
    from a separate programme by providing the appropriate data (accounts and instruction data) to
        reconstruct the Instruction object.

    
4. How could it be better? More efficient? Safer?
    This code can be effiocient in the usage of .clone function. I am aware that.clone() 
    creates a more expensive deep copy in the heap (in terms of speed and cost). The use of 
    references, which would be quicker and less resource-intensive than making deep copies, may be
    an alternative.*/