// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display(12,10)
}

fn add(k:i32, m:i32) -> i32{
    k+m
}

fn display(k:i32, m:i32) ->(){
    let ans = add(k,m);
    println!("{:?}",ans);
}