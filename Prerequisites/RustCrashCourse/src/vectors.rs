//Vectors are growable arrays

pub fn run(){
    let mut v:Vec<i32> = vec![1,2,3,4,5];

    v.push(6);   //adding elkemnts to vector
    println!("Vector items : {:?}, len : {}",v,v.len());

    v.pop();  //removong last lements of vector
    println!("Vector items : {:?}, len : {}",v,v.len());

    //loop through vector
    for num in v.iter(){
        println!("{}",num);
    }
    //loop and mutate values
    for x in v.iter_mut(){
        *x+=1;
    }
    println!("Vector items : {:?}",v);
}