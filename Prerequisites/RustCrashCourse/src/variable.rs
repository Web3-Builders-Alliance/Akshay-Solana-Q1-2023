//variables store primtive data or refrence to data
//variable are immutable by default in rust
//Rust is also block scopes

pub fn run(){
    let name="Akshay";

    //mutable variables
    let mut age=20;
    age=25;
    println!("Hello {} and age is: {}",name,age);

    let (first,second)=("Ram","Kumar");
    println!("First name : {} and second name : {}",first,second);

    //const Variables
    const ID:i32=100;
    println!("ID : {}",ID);
}