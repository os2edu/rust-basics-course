fn main() {
    println!("Hello, world!");

    // 概念理解 -- 借用之后无法执行move语义的赋值的理解

    // eg: 
    struct A { name: String }

    let a = A { name: String::from("aaa")};

    let b = &a; // 正确
    let c = b; // 正确

    // let d = a; 此时赋值语句会报错 无法移动已经被借用的变量

}
