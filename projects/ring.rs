const NUM: usize = 41;
const COUNT: usize = 3;

fn main() {
    let mut ring = [0; NUM];
    for index in 0..NUM {
        ring[index] = index + 1;
        // println!("ring[{}], {}", index, ring[index])
    }
    // return ();
    let mut i = 0;
    let mut j = 0;
    let mut out = 0;
    while out < NUM {
        // println!("当前位置 {}, ring[i]={}", i+1, ring[i]);

        if ring[i] != 0 {
            j = j + 1;
            println!("当前位置 {}, 报数 j = {}", i + 1, j);

            if j == COUNT {
                println!("{} --> out", i + 1);
                out = out + 1;
                ring[i] = 0;
                j = 0;
                if NUM - out < COUNT {
                    break;
                }
            }
        }

        i = (i + 1) % NUM;
        // i = i + 1;
        // if i == NUM {
        //   i = 0;
        // }
    }

    println!("Josephus ring game is over!");
}
