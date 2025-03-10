// Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
// 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
// 当所有者（变量）离开作用域范围时，这个值将被丢弃(drop)
fn main() {
    // let s = "hello"; // s can not be changed
    // let mut ss = String::from("hello"); // ss can be changed
    // ss.push_str(", 111world");
    // println!("{}", s);
    // println!("{}", ss);
    // // 基础的数据类型是存在栈里面的，
    // let a = 1;
    // let b = a; // 这里a和b的值都是1，但是a和b的内存地址是不同的
    // // 1的归属权不一样， 所以a和b的内存地址是不同的
    // //string

    // let s1 = String::from("hello");
    // // let s2 = s1; // 这里s1和s2的值都是hello，但是s1和s2的内存地址是不同的
    // // 归属权，s1 被释放了， 所以s2 拥有hello的归属权
    // println!("{}", s1);
    // let s2 = s1.clone(); // 这里s1和s2的值都是hello，但是s1和s2的内存地址是不同的
    // println!("{}", s2);
    // // 基本类型在编译时是已知大小的，所以可以存储在栈中, 不需要 clone。

    let s3 = String::from("hello_3");
    takes_ownership(s3);// s3 的归属权转移给 some_string
    // s3 不能使用了
    // println!("{}", s3);
    let x = 5;
    makes_copy(x); // x 是基本类型，所以可以复制
    println!("{}", x);
    //brorrow
    let mut m = String::from("hello");
    {
        let r1 = &m;
    }
    let r2 = &mut m;
    println!("{}", r2);
    // 可变引用和不可变引用不能同时存在
    let r3 = &m;
    // let r4 = &mut m; // error occurs
    println!("{}", r3);
    // println!("{}", r4);
    // dangling reference
    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> String {
    let s = String::from("hello");
    // &s
    s
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
