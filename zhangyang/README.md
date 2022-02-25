<h1>传值与传引用在rust中的体现</h1>
<h2>1、Java中的参数</h2>
首先，我们来做一个测试：
我们首先创建一个对象A:
    
    public class A {
    public int a;
    }

接着我们在主函数中尝试对该对象进行传参：

    
    public class HelloWorld {
    public static void main(String[] args){

        A a1=new A();
        a1.a=1;
        int b=1;
        test1(b);
        test2(a1);
        System.out.println(b);
        System.out.println(a1.a);
        }
    public static void test1(int a1){a1=2;}
    public static void test2(A a){a.a=2;}

    }
    //output 1 2
在我们运行该段程序后，我们会发现输出了1和2，那么为什么会产生不同的结果，原因是因为Java将所有数据类型分为两类，其中一类是基本值类型，包括所有在编译期可确定大小的类型，该类型在运行时直接保存在栈上，在进行参数传递时，会在栈中直接开辟内存对原值进行复制，因此对参数进行修改并不会改变原值。另一类是引用类型，即编译期不可确定大小，那么该数据的内存位置会保存在堆上，栈上只会存在一个指向对象的指针，在传参时，栈上会开辟一块内存来复制一个同样的指针指向堆，所以我们修改对象的值会影响原值，这是因为两个指针都指向同一个对象。但是，要注意，Java中不存在传递引用，因为引用传递并不新开辟内存，而是直接操作原值，因此也不存在引用悬挂这一说法。如何证明这一点呢？我们来看以下的代码：
    
    public class HelloWorld {
    public static void main(String[] args){

        A a1=new A();
        a1.a=1;
        int b=1;
        test1(b);
        test2(a1);
        System.out.println(b);
        System.out.println(a1.a);
    }
    public static void test1(int a1){a1=2;}
    public static void test2(A a){a=new A();a.a=2;}

    }
    //output 1 1
在这里我们让参数a指向了一个新创建的对象并把他初始化为2，但是我们会在输出中发现我们的对象还是1，这是为什么呢，因为在函数中我们实际更改的是一个指针，如下图：
![](https://zhangyangzy.oss-cn-beijing.aliyuncs.com/img/20220225145241.png)

所以我们说Java只存在传值而并没有传引用。
<h2>2、rust中的参数传递</h2>
在rust中情况变得更加复杂。
首先rust的大致情况和Java比较类似，基本类型的传递也是不区分深复制和浅复制的，一律都是copy语义。但是对于一些不在栈上的变量，
<h3>1)借用</h3>在借用时我们分为move语义和copy语义，其中的区别是move语义实际上是将原来变量的所有权转移到函数中，而copy是完整的复制了一个新的变量，从而不会对原来的变量产生影响。
<h3>2)引用</h3>引用不会取得所有权，但是依然会对原变量产生影响。在生命周期上也会和原变量保持一致，我们可以参照以下例子：

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
我们运行一下会发现产生了一个错误，
    
    16 | fn test2(mut a:&stu){
    |                - let's call the lifetime of this reference `'1`
    17 |     let b=stu{b:2};
    18 |     a=&b;
       |     --^^
       |     | |
       |     | borrowed value does not live long enough
       |     assignment requires that `b` is borrowed for `'1`
    19 | }
       | - `b` dropped here while still borrowed
我们对i32和struct做了同样的操作，但是会发现只有struct发生了错误，这就是因为引用实际上会和原变量绑定，也就是说函数消亡时引用依然存在，所以导致了引用的悬挂，这和我们在c++中所说的引用并不是一个概念，读者需要注意进行辨析。

参考：https://blog.csdn.net/quicmous/article/details/113916168
    



