
fn main() {
    println!("Success!");
    get_option(10);
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
            println!("Garv")
        }
        _ => {
            // TODO
            println!("Not Garv")
        }
    };
    
    // Rather than returning a None, we use a diverging function instead
    never_return_fn() //null 
}

// IMPLEMENT this function in THREE ways
fn never_return_fn() -> ! {
    
    return panic!("panic");
}

