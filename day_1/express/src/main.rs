// use snake_case for function names
// use camelCase for variable names

use std::fmt::Debug;

// will return the type of the item
fn report<T:Debug>(item:T) {
    println!("report: {:?}", item);
}
fn clear(text: &mut String) {
    *text = String::from("");
}

// diverge function
fn dead_end() -> ! {
    panic!("this function never returns");
}

fn forever() -> ! {
    loop {
        println!("forever");
    }
}
fn add_with_extra(x:i32, y: i32) -> i32 {
    let x = x + 1; //statement
    let y = y + 1; //statement
    x + y + 5 //expression
}

fn main() {
    // statement  has no return value
    // expression has return value
    let sum = add_with_extra(1, 2);
    println!("sum: {}", sum);
    println!("Hello, world!");
    let y = {
        let x = 3;
        x + 1
    };
    println!("y: {}", y);
    assert_eq!(ret_unit_type(), ());
    let z = add_three(10);
    println!("z: {}", z);
    let zz = add_three(3);
    println!("zz: {}", zz);
    // dead_end();
    // forever();
}

fn ret_unit_type() {
    let x = 1;
    let y = if x % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    println!("y: {}", y);
    let five = add_five(1);
    println!("five: {}", five);
}

fn add(i: i32, j: i32) -> i32 {
    i + j
}

// function with default value
fn another_function(i: i32, j:f32) -> i32 {
    i + j as i32
}

// expression can be used as statement
fn add_five(x: i32) -> i32 {
    x + 5
}

// return
fn add_three(x: i32) -> i32 {
    if x > 6 {
        return x + 3000
    } else {
        x + 4
    }
}
