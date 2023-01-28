pub fn run(){
    println!("Print run fn from print.rs");

    println!("Hello {}","Akshay");

    //positional argument
    println!("Hello {2} from {0}, bye {1}","Ram","Shyam","Anu");
    println!("Bye {name}",name="Ramu");

    //placehilder for debug traits
    println!("Tuple items : {:?}",(1,true,"Priya",2.3))

}