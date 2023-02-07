use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, 
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    pubkey::Pubkey,
};


entrypoint!(process_instruction);


fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    // Attempt to serialize the BPF format to our struct
    //  using Borsh
    //
    let instruction_data_object = InstructionData::try_from_slice(&instruction_data)?;

    msg!("Welcome to the park, {}!", instruction_data_object.name);
    if instruction_data_object.height > 5 {
        msg!("You are tall enough to ride this ride. Congratulations.");
    } else {
        msg!("You are NOT tall enough to ride this ride. Sorry mate.");
    };

    Ok(())
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    name: String,
    height: u32,
}


/*Code Journal

1) What are the concepts (borrowing, ownership, vectors etc)
    The serialisation and deserialization of data is the central idea. On-chain, all data is saved 
    in the form of byte representations, but because the binary format is difficult for humans to 
    grasp, we represent data in high level language formats utilising different data types, data 
    structures, classes, objects, and functions, among other things. This human readable format must
     be serialised into binary format in order to be sent to the blockchain for use or storage. 
     Deserialization is the opposite of serialisation; it takes the binary data that is delivered 
     and transforms it into high-level data that is simple to read and comprehend.
 
2) What is the organization?
    This code is organized first by bringing the crates into scope, defining the entrypoint, then 
    building out the process_instruction function.The process instruction function, which is also
     the program's entry point, contains the main logic. This function accepts the accounts and 
     instruction data in addition to the program ID, and it returns the ProgramResult Enum. 
     The function creates the InstructionData object using the 'instruction data' parameter and 
     assigns it to a variable. The height is logically checked, and the necessary messages are 
     then returned. We also see the InstructionData struct defined at the end, following the function
      definition. It has the characteristics "name," "height," and the "derive" trait, which gives 
      it extended/shared behaviour and functionality.

3)What is the contract doing? What is the mechanism? 
    The program is basically taking the serialized input passed into the process_instruction function
    and using try_from_slice method deserializing it into struct object and performing comparisions on its
    fields and logging it
    
4)How could it be better? More efficient? Safer?
    One small improvement can be made is reducing the number of calls to "msg!".

*/