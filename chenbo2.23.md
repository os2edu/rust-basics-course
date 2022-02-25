### 递归数据结构的实现：AVL 树

> 百度曾经说过：
>
> 在[计算机科学](https://baike.baidu.com/item/计算机科学/9132)中，**AVL树**是最先发明的自平衡二叉查找树。在AVL树中任何节点的两个子树的高度最大差别为1，所以它也被称为**高度平衡树**。增加和删除可能需要通过一次或多次[树旋转](https://baike.baidu.com/item/树旋转)来重新平衡这个树。AVL树得名于它的发明者G. M. Adelson-Velsky和E. M. Landis，他们在1962年的论文《An algorithm for the organization of information》中发表了它。
>
> AVL树本质上还是一棵二叉搜索树，它的特点是： [1] 
>
> 1.本身首先是一棵二叉搜索树。
>
> 2.带有平衡条件：每个结点的左右子树的高度之差的绝对值（平衡因子）最多为1。
>
> 也就是说，AVL树，本质上是带了平衡功能的二叉查找树（二叉排序树，二叉搜索树）。

实现了一棵 AVL 树，还差一个 Iter 没有实现。

我觉得用 rust 实现递归数据结构一个比较复杂的方面就是你需要标明一个很难标注的生命周期，困难之处在于入参和出参可能并没有直接的生命周期关系。

```rust
// 这是一个 Tree<T>的方法,Tree<T> 是一个二叉平衡树
// 函数返回找到的子树的可变引用
fn recursion(&mut self, &t: T) -> &mut Tree<T> {
    match self.val.cmp(t) {
        case Less => {
            self.right_subtree.recursion()
        },
        case Equal => {
            self
        },
        case Greater => {
            self.left_subtree.recursion()
        }
    }
}
```

比我我们在树根去调用这个函数

`root.recursion(&t)` ，你会发现返回值的生命周期既和 `t` 没啥关系，和树根 `root` 也没啥关系，所以这件事就成了很难做到的事。

我的解决办法是，如果碰到了这种情况（AVL 的 `delete` 操作就会有这个情况），我就直接传入树的所有权，配合 `Option::take()` 方法实现操作，`take` 方法其实和 `mem::replace` 是一个东西，它可以把结构体的一个成员给偷走，然后补上一个其他值，`take` 方法把 `Option` 偷走后，放回一个 `None`。非常滴实用。

### rust 实用函数式编程

rust 的错误处理提供了好用的 `Option`, `Result` ，它们是两个类型构造器。

类型构造器就是泛型类型，比如 `Option` 可以接受一个 `Person` 类型变成一个 `Option<Persion>`, 当然也可以用 `Option<Option<Person>>` 这种东西，这种东西非常糟糕，因为你要访问里面的数据，你得拆两次，要是有人写出来套 10 层 `Option` 的东西给你让你处理，你肯定心里默默问候他家人了，你得套 10 层模式匹配才行解包这个玩意。

咱心里明白的很，这十层模式匹配基本都是废话，它们表达的都是一个意思，这十层模式匹配代码是这么相似，这是信息冗余！

错误处理本该没这么痛苦，rust 提供了函数式的 API，可以让咱避免搞出来叠好多层的 Option, 或者因为错误处理导致代码过度耦合。

#### Monad

<del>Monad 简单的来说就是自函子范畴上的幺半群</del>  

**Monad 就是用来粘函数的。**

比如咱的数学老师让你求一个 vector 的平均值的倒数开根。

比如说标准库里有一个写好的求 mean(平均值) 的函数，它的函数签名是这样的：

`mean: Vec<int> -> Option<float>` 这是一个可能失败的函数，因为你没办法给一个空数组定义一个平均值。

标准库里还有一个基于 `magic number` 实现的平均值倒数开根:

`kaigen: float -> Option<float> ` 这个函数显然也可能失败。

然后咱就开始组装两个函数了，标准实现是这样的

```rust
fn foo(v: Vec<Int>) -> Option<Float> {
    match mean(v) {
        case None => None,
        case Some(a) => kaigen(a),
    }
}
```

Monad 就是让我们避免写出这样的代码的。

因为组合 **任意** 两个签名是这样的函数(`f: a -> Option<b>`, `g: b -> Option<c>`) 的行为都是相似的，并且我们总在做相同的事情，它们无外乎是先把参数给 a，如果成功了就传给下一个，失败就把失败传递下去。

所以我们抽象了这个逻辑，带来了 `and_then` 方法，实现上面的逻辑看起来是这样的：

```rust
fn bar(v: Vec<Int>): Option<Float> {
    mean(v).and_then(|a_float| kaigen(a_float))
}
```

我们就简单的把两个可能失败的函数 **粘起来 **了。看起来代码没减少几行，但是如果你要粘很多函数，那它的强大就体现出来了，因为如果标准实现你可能会写一堆的 `match`, 而且更重要的是，这代码可读性提升太大了。

#### Functor

Functor 也是用来粘函数的

我们已经有能力粘两个可能失败的函数了，我们现在可能有需求粘一个可能失败和一个不会失败的函数，如果不会失败的函数在前，很好粘：

`may_fail(succ(a))`

可是调换一下顺序就不太好粘了

`f: a -> Option<b>` 和 `g: b -> c`

这两个函数可以用 `map` 粘起来，这两个函数粘起来啥意思是显然的

比如新函数输入了 `ImaA`(我是一个A) 那么输出：`f(ImaA).map(|ImaB| g(ImaB))`

相当于粘完后得到一个 `a -> Option<c>` 的函数

#### 我也不知道叫啥，反正很有用的高阶函数

`map_or` 方法接受一个 `self: Option<T>`,一个 `default` ，一个函数`f` ，如果是 None，就返回 default ,如果是 Some，就解包后传入f。

它也粘了这样两个函数:

`f: a -> Option<b>`

`g: b -> c`

不过粘出来的函数是 `a -> c`

#### 总结

我们就有了三个粘可能失败的函数的办法，这样我们就可以把几乎所有可以导致错误的函数粘到一起，然后最后搞出来一个 `a -> Opiton<b>` 来供下个调用者继续粘函数或者处理错误，或者我们也可以用黑科技 `map_or` 直接做错误处理。（这个方法的行为就有点像错误处理，不是吗？）最终写出这样的东西。

```rust
fn foobar(a) -> Option<z> {
    may_fail(succ1(a))
    .and_then(|dd| may_fail1(dd))
    .and_then(|ee| may_fail2(ee))
    .map(|ff| succ2(ff))
}
```

虽然不是很好看，不过它基本是我们能掌握的最好的武器了，这种逻辑复杂的代码，反正咋写都不会好看的。。。

最后如果一个类型构造器 F 实现了粘两个函数(`a -> F<b>`, `b -> F<c>`) 得到一个`a -> F<c>`的函数的方法，那么它就是一个 Monad，显然 `Result` 也是一个 Monad，我们可以把它看成第二个泛型（err）固定的 Monad，因为它实现了`ok_or`方法，这东西基本和 `Option`  构造是相同的，就不说了（或者第一个泛型固定的Monad，不过可能一般不这么用），最后，咱有时间可以试试用`Option`的`and_then` 去把 `map` 构造出来，这两个函数的签名其实蛮像的，所以一个 Monad 一定是一个 Functor，同样的 Result 也是一个 Functor，所以标准库肯定也有类似 `Option` 的 `map` 的方法。

