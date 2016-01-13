// p14: longest collatz sequence
//
// the idea  is to cache the computed result.

pub fn main() {
    let mut cache = vec![-1; 1000000];
    cache[1] = 0;

    for start in 2..1000000 {
        let mut running = start;
        let mut seq = 1;

        while running != 1 {
            if running < 1000000 && cache[running] != -1 {
                seq += cache[running];
                break;
            }

            if running % 2 == 0 {
                running = running/2;
            } else {
                running = 3 * running + 1;
            }
            seq += 1;
        }

        cache[start] = seq;
    }

    let mut max = -1;
    let mut ret = 1;
    for i in 1..1000000 {
        if cache[i] > max {
            max = cache[i];
            ret = i;
        }
    }

    println!("ret = {}, max = {}", ret, max);
}
