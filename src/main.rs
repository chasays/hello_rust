// fn multi_language() {
//     let janpan = "janpan";
//     let china = "我的中国";
//     let uk = "uk";
//     let names = [janpan, china, uk];
//     // for item in names.iter(){
//     //     println!("{}", &item);
//     // }
//     for item in names{
//         println!("{}", &item);
//     }
// }

// fn print_numbers() {
//     for number in 1..=5 { // The `..=` operator includes the upper bound
//         println!("{}", number);
//     }
// }

// fn add(i:i32, j:i32) -> i32 {
//     i + j // here is the return value， and can add ; to end the function
// }

fn define_constant() {
    const MAX_NUM:i32 = 100_000;
    println!("MAX_NUM: {}", MAX_NUM);
}

struct User {
    e: i32
}
fn main() {
    // multi_language();
    // print_numbers();
    // let a = 10;
    // let b:i32 = 40;
    // println!("Hello, world!");
    // println!("b: {}", b);
    // let c = add(a,b);
    // println!("a+b={}", add(a,b));
    // let mut x = 6;
    // println!("x: {}", x);
    // x = 77;
    // println!("x: {}", x);
    // let _y = 88;
    // // println!("y: {}", y);
    // let (a, mut b): (bool, bool) = (true, false);
    // println!("a: {}, b: {}", a, b);
    // b = true;
    // println!("a: {}, b: {}", a, b);
    // assert_eq!(a, b);
    // day 2
    // let (a,b,c,d,e);
    // (a,b) = (1,2);
    // [c,..,d, _] = [1,2,3,4,5];
    // User {e,..} = User {e:5};
    // assert_eq!([1,2,1,4,5], [a,b,c,d,e]);
    // let f = 1;
    // println!("f: {}", f);
    // // const MAX_NUM:i32 = 100_000;
    // define_constant();
    // let aa = 1;
    // let aa = aa +1;
    // {
    //     let aa = aa *2;
    //     println!("aa: {}", aa);
    // }
    // println!("newaa: {}", aa);
    // let mut space = "    ";
    // println!("space: {}", space.len());
    // // 覆盖只能 同一个数据类型的；
    // space = "123";
    // day 3
    let guess: i32 = "42".parse().expect("not a numer:");
    let num:i8 = 127;
    println!("num: {}", num);
    let a :u8 = 255;
    let b = a.wrapping_add(1);
    println!("b: {}", b);
    let x = 2.0;
    let y:f32 = 3.0;
    println!("x: {}", x);
    println!("y: {}", y);
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    // ieee754
    let abc:(f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz:(f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc:{:x}", (abc.0+abc.1).to_bits());
    println!("abc:{:x}", abc.2.to_bits());
    println!("xyz:{:x}", (xyz.0+xyz.1).to_bits());
    println!("xyz:{:x}", xyz.2.to_bits());

    // NaN
    let x = (-42.0_f32).sqrt();
    println!("x: {}", x);
    if x.is_nan() {
        println!("x is nan not a number");
    }

    // math operation
    let sum = 2+5;
    let diff = 234.3-22.1;
    let product = 4*30;
    let quotient = 56.7/3.0;
    let remainder = 43%5;
    println!("sum: {}", sum);
    println!("diff: {}", diff);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    let x = 20;
    let y:i32 = 30;
    let z = 30_i32;
    let zz = 33i32;
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    println!("zz: {}", zz);
    // define a vector
    let list = [2.2, 3.3, 4.4, 5.5123123];
    println!("list: {:.3}", list[3]);
    let a: u8 = 2;
    let b: u8 = 3;
    println!("a: {:08b}", a);
    println!("b: {:08b}", b);
    println!("a&b: {:08b}", a&b);
    println!("a|b: {:08b}", a|b);
    println!("a^b: {:08b}", a^b);
    println!("!a: {:08b}", !a);
    //for range
    for i in 1..=5 {
        println!("i: {}", i);
    }
    for i in 1..5 {
        println!("i: {}", i);
    }
    for i in 'a'..='z' {
        println!("i: {}", i);
    }
    
}