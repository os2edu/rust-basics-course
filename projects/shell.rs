
/* 
// C代码，参考 https://github.com/limingth/NCCL/blob/master/Unit-1/Lesson-19.md
#include <stdio.h>
#include <string.h>

int shell_parse(char * buf, char * argv[])
{
	int argc = 0;
	int state = 0;

	while (*buf)
	{
		if (*buf != ' ' && state == 0)
		{
			argv[argc++] = buf;
			state = 1;
		}
		if ((*buf == ' ') && state == 1)
		{
			*buf = '\0';
			state = 0;
		}

		buf++;	
	}

	return argc;
}

int do_cmd(int argc, char ** argv)
{
	int i;

	printf("argc = %d\n", argc);

	for (i = 0; i < argc; i++)
		printf("argv[%d]: <%s>\n", i, argv[i]);	

	return 0;
}

int do_add(int argc, char ** argv)
{
	printf("help: this is a add function with %d args\n", argc - 1);

	return 0;
}

int do_sub(int argc, char ** argv)
{
	printf("ls: this is a sub function with %d args\n", argc - 1);

	return 0;
}

int (*pf)(int argc, char ** argv);

int main(void)
{
	char buf[64];
	int argc = 0;
	char * argv[10];

	printf("$ ");
	fgets(buf, 64, stdin);
	buf[strlen(buf)-1] = '\0';
	printf("<%s>\n", buf);

	argc = shell_parse(buf, argv);

	pf = do_cmd;	
	if (strcmp(argv[0], "add") == 0)					
		pf = do_add;	

	if (strcmp(argv[0], "sub") == 0)
		pf = do_sub;	

	pf(argc, argv);
	//callback(pf，argc, argv);

	return 0;
}
*/

// 函数指针用法参考 Rust学习笔记： https://skyao.io/learning-rust/docs/grammar/pointer/fn.html
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
    // 通过函数指针调用函数
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

// let a = 2;
// let b = 3;
// assert_eq!(math(sum, a, b), 5);
// assert_eq!(math(product, a, b), 6);

// let a = 2;
// let b = 3;
// assert_eq!(math(sum, a, b), 5);
// assert_eq!(math(product, a, b), 6);

// 结构体用法参考教材： https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html
struct Command {
    name: String,
    handler: fn(i32, i32) -> i32
}

fn do_cmd(name: String, argv1: String, argv2: String) -> i32 {
    let cmd1 = Command {
        name: "s".to_string(),
        handler: sum
    };
    let cmd2 = Command {
        name: "p".to_string(),
        handler: product
    };

    let v1 = argv1.parse::<i32>().unwrap();
    let v2 = argv2.parse::<i32>().unwrap();
    if name == cmd1.name {
        return (cmd1.handler)(v1, v2);
    }
    if name == cmd2.name {
        return (cmd2.handler)(v1, v2);
    }

    println!("unknown command: {}", name);
    return -1;
}

// use std::env;

// fn main()
// {
//     let args: Vec<String> = env::args().collect();
//     println!("{:?}", args);
// }

fn main() {
    let mut buf = String::new();
    println!("Please input your command:");
    let length = std::io::stdin().read_line(&mut buf).unwrap();
    println!("your input line is: {}", buf);
    println!("line's length: {}", length);

    // let words: Vec<&str> = buf.split(' ').collect();
    let argv: Vec<&str> = buf.split_whitespace().collect();
    println!("{:?}", argv);

    // let argc = argv.len();
    // println!("argc is {}", argc);

    // println!("sum 2 3 is {}", math(sum, 2, 3));
    // println!("product 2 3 is {}", math(product, 2, 3));
    
    // let v1 = argv[1].parse::<i32>().unwrap();
    // let v2 = argv[2].parse::<i32>().unwrap();
    // println!("sum a b is {}", math(sum, v1, v2));

    let result = do_cmd(argv[0].to_string(), argv[1].to_string(), argv[2].to_string());
    println!("result is {}", result);
 }

