fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Creating a copy to avoid dangling pointer. 
    let vec_copy = vec.clone();
    let ptr = vec_copy.as_ptr();

    //Dropping the original vector will not lead to a dangling pointer as vec_copy holds a copy.
    drop(vec);

    // Accessing memory through the pointer obtained from vec_copy which is not dropped yet.
    println!("Value at ptr: {}", unsafe { *ptr });
} 