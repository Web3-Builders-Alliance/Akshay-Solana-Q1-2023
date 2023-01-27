/*2 types of strings are :
Primitive - Imuttable fixed size strings somewehre in memory
String - Mutable , Growable,heap allocated data syructure- best for modification
*/

pub fn run(){
    let name="Akshay"; //type 1 , primitive string, imutable

    let mut name2=String::from("Akshay");  //type 2 String, mutable

    name2.push('x');    //pushinf char to string
    name2.push_str(" Dhayal");    //pushing string

    println!("Hello {}, String length: {},  
    string isEmpty : {}, string capacity : {}",name2,name2.len(),name2.is_empty(), name2.capacity());

    for word in name2.split_whitespace(){
        println!("{}",word);
    }

    //create String with capacity set
    let mut s=String::with_capacity(10);

    assert_eq!(10,s.capacity()); //assertion cons, gives error only if condition don't matchs

}