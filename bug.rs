fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let ptr = vec.as_ptr();

    // This is unsafe because we are dropping the vector before the pointer goes out of scope. 
    drop(vec);

    // Accessing memory through a dangling pointer will lead to undefined behavior.
    println!("Value at ptr: {}", unsafe { *ptr });
}