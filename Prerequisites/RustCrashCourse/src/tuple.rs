//tuple group diff datatypes elements, max 12 elements
pub fn run(){
    let a:(&str, &str, i32)=("Ram","Kumar",20);   //&str is string literals

    println!("{} {} is {} years old",a.0,a.1,a.2);  //accessing tuple items
}