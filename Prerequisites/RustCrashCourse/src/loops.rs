pub fn run(){
    //infinite loop
    let mut count=0;
    loop{
        count+=1;
        println!("count : {}",count);

        if count==10{
            break;
        }
    }

    //while loop, fizzbuzz program
    let mut val=1;
    while val<=20{
        if val%3 ==0 && val%5==0{
            println!("{} is FizzBuzz",val);
        }
        else if val%3==0{
            println!("{} is Fizz",val);
        }else if val%5==0{
            println!("{} is Buzz",val);
        }
        val+=1;
    }

    //for loop
    for i in 0..10{
        println!("{}",i);
    }
}