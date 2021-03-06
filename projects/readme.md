
## 约瑟夫环问题
据说著名犹太历史学家Josephus有过以下的故事：在罗马人占领乔塔帕特后，39 个犹太人与Josephus及他的朋友躲到一个洞中，39个犹太人决定宁愿死也不要被敌人抓到，于是决定了一个自杀方式，41个人排成一个圆圈，由第1个人开始报数，每报数到第3人该人就必须自杀，然后再由下一个重新报数，直到所有人都自杀身亡为止。然而Josephus 和他的朋友并不想遵从。首先从一个人开始，越过k-2个人（因为第一个人已经被越过），并杀掉第k个人。接着，再越过k-1个人，并杀掉第k个人。这个过程沿着圆圈一直进行，直到最终只剩下一个人留下，这个人就可以继续活着。问题是，给定了和，一开始要站在什么地方才能避免被处决。Josephus要他的朋友先假装遵从，他将朋友与自己安排在第16个与第31个位置，于是逃过了这场死亡游戏。

```
iMac:projects liming$ rustc ring.rs 
iMac:projects liming$ ./ring 
当前位置 1, 报数 j = 1
当前位置 2, 报数 j = 2
当前位置 3, 报数 j = 3
3 --> out
当前位置 4, 报数 j = 1
当前位置 5, 报数 j = 2
当前位置 6, 报数 j = 3
6 --> out
当前位置 7, 报数 j = 1
当前位置 8, 报数 j = 2
当前位置 9, 报数 j = 3
9 --> out
当前位置 10, 报数 j = 1
当前位置 11, 报数 j = 2
当前位置 12, 报数 j = 3
12 --> out
当前位置 13, 报数 j = 1
当前位置 14, 报数 j = 2
当前位置 15, 报数 j = 3
15 --> out
当前位置 16, 报数 j = 1
当前位置 17, 报数 j = 2
当前位置 18, 报数 j = 3
18 --> out
当前位置 19, 报数 j = 1
当前位置 20, 报数 j = 2
当前位置 21, 报数 j = 3
21 --> out
当前位置 22, 报数 j = 1
当前位置 23, 报数 j = 2
当前位置 24, 报数 j = 3
24 --> out
当前位置 25, 报数 j = 1
当前位置 26, 报数 j = 2
当前位置 27, 报数 j = 3
27 --> out
当前位置 28, 报数 j = 1
当前位置 29, 报数 j = 2
当前位置 30, 报数 j = 3
30 --> out
当前位置 31, 报数 j = 1
当前位置 32, 报数 j = 2
当前位置 33, 报数 j = 3
33 --> out
当前位置 34, 报数 j = 1
当前位置 35, 报数 j = 2
当前位置 36, 报数 j = 3
36 --> out
当前位置 37, 报数 j = 1
当前位置 38, 报数 j = 2
当前位置 39, 报数 j = 3
39 --> out
当前位置 40, 报数 j = 1
当前位置 41, 报数 j = 2
当前位置 1, 报数 j = 3
1 --> out
当前位置 2, 报数 j = 1
当前位置 4, 报数 j = 2
当前位置 5, 报数 j = 3
5 --> out
当前位置 7, 报数 j = 1
当前位置 8, 报数 j = 2
当前位置 10, 报数 j = 3
10 --> out
当前位置 11, 报数 j = 1
当前位置 13, 报数 j = 2
当前位置 14, 报数 j = 3
14 --> out
当前位置 16, 报数 j = 1
当前位置 17, 报数 j = 2
当前位置 19, 报数 j = 3
19 --> out
当前位置 20, 报数 j = 1
当前位置 22, 报数 j = 2
当前位置 23, 报数 j = 3
23 --> out
当前位置 25, 报数 j = 1
当前位置 26, 报数 j = 2
当前位置 28, 报数 j = 3
28 --> out
当前位置 29, 报数 j = 1
当前位置 31, 报数 j = 2
当前位置 32, 报数 j = 3
32 --> out
当前位置 34, 报数 j = 1
当前位置 35, 报数 j = 2
当前位置 37, 报数 j = 3
37 --> out
当前位置 38, 报数 j = 1
当前位置 40, 报数 j = 2
当前位置 41, 报数 j = 3
41 --> out
当前位置 2, 报数 j = 1
当前位置 4, 报数 j = 2
当前位置 7, 报数 j = 3
7 --> out
当前位置 8, 报数 j = 1
当前位置 11, 报数 j = 2
当前位置 13, 报数 j = 3
13 --> out
当前位置 16, 报数 j = 1
当前位置 17, 报数 j = 2
当前位置 20, 报数 j = 3
20 --> out
当前位置 22, 报数 j = 1
当前位置 25, 报数 j = 2
当前位置 26, 报数 j = 3
26 --> out
当前位置 29, 报数 j = 1
当前位置 31, 报数 j = 2
当前位置 34, 报数 j = 3
34 --> out
当前位置 35, 报数 j = 1
当前位置 38, 报数 j = 2
当前位置 40, 报数 j = 3
40 --> out
当前位置 2, 报数 j = 1
当前位置 4, 报数 j = 2
当前位置 8, 报数 j = 3
8 --> out
当前位置 11, 报数 j = 1
当前位置 16, 报数 j = 2
当前位置 17, 报数 j = 3
17 --> out
当前位置 22, 报数 j = 1
当前位置 25, 报数 j = 2
当前位置 29, 报数 j = 3
29 --> out
当前位置 31, 报数 j = 1
当前位置 35, 报数 j = 2
当前位置 38, 报数 j = 3
38 --> out
当前位置 2, 报数 j = 1
当前位置 4, 报数 j = 2
当前位置 11, 报数 j = 3
11 --> out
当前位置 16, 报数 j = 1
当前位置 22, 报数 j = 2
当前位置 25, 报数 j = 3
25 --> out
当前位置 31, 报数 j = 1
当前位置 35, 报数 j = 2
当前位置 2, 报数 j = 3
2 --> out
当前位置 4, 报数 j = 1
当前位置 16, 报数 j = 2
当前位置 22, 报数 j = 3
22 --> out
当前位置 31, 报数 j = 1
当前位置 35, 报数 j = 2
当前位置 4, 报数 j = 3
4 --> out
当前位置 16, 报数 j = 1
当前位置 31, 报数 j = 2
当前位置 35, 报数 j = 3
35 --> out
当前位置 16, 报数 j = 1
当前位置 31, 报数 j = 2
当前位置 16, 报数 j = 3
16 --> out
当前位置 31, 报数 j = 1
当前位置 31, 报数 j = 2
当前位置 31, 报数 j = 3
31 --> out
Josephus ring game is over!
```

## 命令解释器
实现一个命令行下的计算器，可以实现加减乘除四则运算。

```
iMac:rust-basics-course liming$ rustc shell.rs 
iMac:rust-basics-course liming$ ./shell 
Please input your command:
s 2 3
your input line is: s 2 3

line's length: 6
["s", "2", "3"]
result is 5
iMac:rust-basics-course liming$ ./shell 
Please input your command:
p 2 3
your input line is: p 2 3

line's length: 6
["p", "2", "3"]
result is 6
iMac:rust-basics-course liming$ ./shell 
Please input your command:
add 2 3
your input line is: add 2 3

line's length: 8
["add", "2", "3"]
unknown command: add
result is -1
iMac:rust-basics-course liming$ 
```

## 表达式求值问题
计算表达式 1-2+3*4-5= 的值，需要考虑乘除法的优先级结合比加减法高。
```
iMac:projects liming$ ./calc 1-2+3*4-5=
["./calc", "1-2+3*4-5="]
buf is 1-2+3*4-5=
get a operand 1, push to operand_stack
get a operator -
top operator is None, push - to operator_stack
get a operand 2, push to operand_stack
get a operator +
top operator is Not None, compare prio of - vs +
- >= +, pop and pop and calc, then push
pop and pop => 1 and 2
calc result = -1, push it to operand_stack
push + to operator_stack
get a operand 3, push to operand_stack
get a operator *
top operator is Not None, compare prio of + vs *
+ < *, push + back and push * and wait for next operand 
get a operand 4, push to operand_stack
get a operator -
top operator is Not None, compare prio of * vs -
* >= -, pop and pop and calc, then push
pop and pop => 3 and 4
calc result = 12, push it to operand_stack
push - to operator_stack
get a operand 5, push to operand_stack
get a operator =
top operator is Not None, compare prio of - vs =
- >= =, pop and pop and calc, then push
pop and pop => 12 and 5
calc result = 7, push it to operand_stack
Last Step: 
pop and pop => -1 and 7
t = +
calc result = 6, push it to operand_stack
num: 6
```