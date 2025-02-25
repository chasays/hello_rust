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
    let (a,b,c,d,e);
    (a,b) = (1,2);
    [c,..,d, _] = [1,2,3,4,5];
    User {e,..} = User {e:5};
    assert_eq!([1,2,1,4,5], [a,b,c,d,e]);
    let f = 1;
    println!("f: {}", f);
    // const MAX_NUM:i32 = 100_000;
    define_constant();
    let aa = 1;
    let aa = aa +1;
    {
        let aa = aa *2;
        println!("aa: {}", aa);
    }
    println!("newaa: {}", aa);
    let mut space = "    ";
    println!("space: {}", space.len());
    // 覆盖只能 同一个数据类型的；
    space = "123";

}
