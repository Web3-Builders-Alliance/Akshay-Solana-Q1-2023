pub fn run(){
    let mut arr:[i32;5]=[1,2,3,4,5];

    arr[2]=0;  //modifying array items
    println!("Array : {:?} , array len : {},  array[1] : {}, 
    bytes occupied : {}",arr,arr.len(), arr[1], std::mem::size_of_val(&arr));

    //Get slices from arrays
    let slice: &[i32] = &arr[0..2];
    println!("Sliced array : {:?}",slice);
}