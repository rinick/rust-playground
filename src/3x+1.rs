fn check64(n: i64, max_visit: &mut i64) -> i64 {
    let mut t = n;
    let mut i: i64 = 0;
    while true {
        if (t & 1) == 0 {
            t >>= 1;
        } else {
            t = (t * 3 + 1) >> 1;
        }
        if t < n {
            break;
        }
        i += 1;
    }
    if i > *max_visit {
        println!("max visit {} : {}  {}", i, n, t);
        *max_visit = i;
    }
    return i;
}


fn main() {
    let mut max_visit: i64 = 0;
    let mut a: i64 = 2;
    while a < 10000000000 {
        check64(a, &mut max_visit);
        a += 1;
    }
}



