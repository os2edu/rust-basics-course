
use std::env;

// run: ./calc 1+3*2
// C代码参考 https://github.com/limingth/NCCL.codes/blob/master/Lesson-35/stack.c
fn prio(op: char) -> i32 {
	match op {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        '/' => 2,
        _ => 0,
    }
}

fn do_calc(op: char, a: i32, b: i32) -> i32 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => 0,
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let buf = args[1].to_string();
    println!("buf is {}", buf);

    // number stack like 1,2,3
    let mut operand_stack: Vec<i32> = Vec::new();
    // operator stack like +,-,*,/
    let mut operator_stack: Vec<char> = Vec::new();

    // convert String buf to Vector<char> v
    let v: Vec<char> = buf.chars().collect();
    for i in &v {
        // println!("{}", i);
        let c = *i;

        if c >= '0' && c <= '9' {
            println!("get a operand {}, push to operand_stack", c);
            let u = c.to_digit(10).unwrap() as i32;
            operand_stack.push(u);
        } else {
            if c == '+' || c == '-' || c == '*' || c == '/' || c == '=' {
                println!("get a operator {}", i);

                // let top_operator = operator_stack.pop();
                let top_operator = operator_stack.pop();
                if top_operator == None {
                    println!("top operator is None, push {} to operator_stack", c);
                    operator_stack.push(c);
                } else {
                    let t = top_operator.unwrap();
                    println!("top operator is Not None, compare prio of {} vs {}", t, c);
                    if prio(t) >= prio(c) {
                        println!("{} >= {}, pop and pop and calc, then push", t, c);

                        let top_operand2 = operand_stack.pop().unwrap();
                        let top_operand1 = operand_stack.pop().unwrap();
                        println!("pop and pop => {} and {}", top_operand1, top_operand2);

                        let result = do_calc(t, top_operand1, top_operand2);
                        println!("calc result = {}, push it to operand_stack", result);
                        operand_stack.push(result);

                        if c != '=' {
                            println!("push {} to operator_stack", c);
                            operator_stack.push(c);
                        } else {
                            println!("Last Step: ");
                            let top_operator = operator_stack.pop();
                            if top_operator != None {
                                let top_operand2 = operand_stack.pop().unwrap();
                                let top_operand1 = operand_stack.pop().unwrap();
                                println!("pop and pop => {} and {}", top_operand1, top_operand2);
                                let t = top_operator.unwrap();
                                println!("t = {}", t);
                                let result = do_calc(t, top_operand1, top_operand2);
                                println!("calc result = {}, push it to operand_stack", result);
                                operand_stack.push(result);
                            }
                        }
                    } else {
                        println!("{} < {}, push {} back and push {} and wait for next operand ", t, c, t, c);
                        operator_stack.push(t);
                        operator_stack.push(c);
                    }
                    
                }
            }
        }
    }

    while let Some(top) = operand_stack.pop() {
        // Prints all numbers like 3, 2, 1
        println!("num: {}", top);
    }

    while let Some(top) = operator_stack.pop() {
        // Prints all operator like *, +
        println!("operator: {}", top);
    }
}
