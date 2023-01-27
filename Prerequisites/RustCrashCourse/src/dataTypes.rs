/*
Primitive types
Integers : u8,u16,u32,u64,u128,i8,i16,i32,i64,i128
Floats: f32,f64
Boolean(bool): true,false
Characters(char)
Tuples,Array
*/

//Rust is statically types means it should know datatypes at compile time , however
// compiler can infer types based on the value
pub fn run(){

    let a=1; //default type is i32
    let b=1.2;  //default type is f64

    let c: i16 =20;  //explicit type
    let d=true;
    let e="a";

    println!("Max i32 value: {}",std::i32::MAX);

    println!("Types value : {:?}",(a,b,c,d,e));
}