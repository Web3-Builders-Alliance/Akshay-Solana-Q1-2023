pub fn run(){
    let num=100;
    if num<10{                        
        println!("Single digit number");
    }else if num>=10 && num<100{                    //and condition - && , or condition- ||
        println!("Double dgit number");
    }else{
        println!("bigger than 99");
    }
}