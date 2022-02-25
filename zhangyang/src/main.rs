//#[derive(Clone, Copy)]
struct stu{
    b:i32
}
impl stu{
    fn new(&mut self,val:i32)
    {
        self.b=val;
    }
}
fn test1(mut a:&i32)
{
    let b=&2;    
    a=&b;
}
fn test2(mut a:&stu){
    let b=stu{b:2};
    a=&b;
}
fn main() {
    let m=1;
    test1(&m);
    print!("{}",m);
    let mut stu1=stu{b:1};
    test2(&stu1);
    //print!("{}",stu1.b)
}
