fn main() {
    stack_start();
}

#[inline(never)]
fn stack_start() {

    let first: i32 = 1;
    let first_ptr: usize = &first as *const i32 as usize;
    // Note the following would also work
    // let first_ptr: usize = &first as *const _ as usize;
    
    stack_recurse(first_ptr);  
}

#[inline(never)]
fn stack_recurse(first_ptr: usize) {

    let second: i32 = 1;
    let second_ptr: usize = &second as *const i32 as usize;
    // Note the following would also work
    // let second_ptr: usize = &second as *const _ as usize;
    
    if first_ptr ==  second_ptr {
        println!("Unexpected stack pointer is the same {first_ptr} == {second_ptr}");
    } else if first_ptr < second_ptr {
        println!("Stack pointer increases {first_ptr} < {second_ptr}");
    } else {
        println!("Stack pointer decreases {first_ptr} > {second_ptr}");
    }

}

