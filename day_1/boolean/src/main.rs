fn main() {
    let a = 'Z';
    let b = 'z';
    let c = '我';
    let cd = "d";
    println!("a: {}", std::mem::size_of_val(&a));
    println!("b: {}", std::mem::size_of_val(&b));
    println!("c: {}", std::mem::size_of_val(&c));
    println!("cd: {}", std::mem::size_of_val(&cd));
    println!("Hello, world!");
    let t = true;
    let f = false;
    let fa: bool = 1==1;
    println!("fa: {}", fa);
    println!();
    // println!("(): {}", );
}
