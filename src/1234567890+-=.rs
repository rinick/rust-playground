//  make a valid equation with 0123456789+-=  every character must be used only once

fn check_result(result: &[usize; 13]) -> bool {
    let mut opsum = 0;
    let mut nums: [i64; 4] = [0; 4];
    let mut num_count = 0;
    let mut current = 0;
    for i in 0..13 {
        if (result[i] < 10) {
            current = current * 10 + result[i];
        } else {
            nums[num_count] = current as i64;
            num_count += 1;
            current = 0;
            opsum = opsum * 100 + result[i];
        }
    }
    nums[3] = current as i64;
    if (opsum == 101112) {
        if (nums[0] == nums[1] + nums[2] - nums[3]) {
            // println!("{} = {} + {} - {}", nums[0], nums[1], nums[2], nums[3]);
            return true;
        }
    } else if (opsum == 101211) {
        if (nums[0] + nums[1] == nums[2] - nums[3]) {
            // println!("{} + {} = {} - {}", nums[0], nums[1], nums[2], nums[3]);
            return true;
        }
    }
    return false;
}


fn main() {
    let mut count: i64 = 0;
    let mut found: i64 = 0;
    let mut pos: [i32; 13] = [-1; 13];
    for A in 0..10
    {
        pos[A] = 0;
        let mut beginB = 0;
        let mut endB = 11;
        if (A == 0) {
            beginB = 10;
            endB = 11;
        }
        for B in beginB..endB {
            if (pos[B] == -1) {
                pos[B] = 1;
                let mut endC = 10;
                if (B < 10) {
                    if (pos[10] != -1) {
                        endC = 13;
                    } else {
                        endC = 11;
                    }
                }
                for C in 0..endC {
                    if (pos[C] == -1) {
                        pos[C] = 2;
                        let mut endD = 10;
                        if (C < 10) {
                            if (pos[10] != -1) {
                                endD = 13;
                            } else {
                                endD = 11;
                            }
                        }
                        for D in 0..endD {
                            if (pos[D] == -1) {
                                pos[D] = 3;
                                let mut endE = 10;
                                if (D < 10) {
                                    if (pos[10] != -1) {
                                        endE = 13;
                                    } else {
                                        endE = 11;
                                    }
                                }
                                for E in 0..endE {
                                    if (pos[E] == -1) {
                                        pos[E] = 4;
                                        let mut endF = 10;
                                        if (E < 10) {
                                            if (pos[10] != -1) {
                                                endF = 13;
                                            } else {
                                                endF = 11;
                                            }
                                        }
                                        for F in 0..endF {
                                            if (pos[F] == -1) {
                                                pos[F] = 5;
                                                let mut endG = 10;
                                                if (F < 10) {
                                                    if (pos[10] != -1) {
                                                        endG = 13;
                                                    } else {
                                                        endG = 11;
                                                    }
                                                }
                                                for G in 0..endG {
                                                    if (pos[G] == -1) {
                                                        pos[G] = 6;
                                                        let mut endH = 10;
                                                        if (G < 10) {
                                                            if (pos[10] != -1) {
                                                                endH = 13;
                                                            } else {
                                                                endH = 11;
                                                            }
                                                        }
                                                        for H in 0..endH {
                                                            if (pos[H] == -1) {
                                                                pos[H] = 7;
                                                                let mut endI = 10;
                                                                if (H < 10) {
                                                                    if (pos[10] != -1) {
                                                                        endI = 13;
                                                                    } else {
                                                                        endI = 11;
                                                                    }
                                                                }
                                                                for I in 0..endI {
                                                                    if (pos[I] == -1) {
                                                                        pos[I] = 8;
                                                                        let mut endJ = 10;
                                                                        if (I < 10) {
                                                                            if (pos[10] != -1) {
                                                                                endJ = 13;
                                                                            } else {
                                                                                endJ = 11;
                                                                            }
                                                                        }
                                                                        for J in 0..endJ {
                                                                            if (pos[J] == -1) {
                                                                                pos[J] = 9;
                                                                                let mut endK = 10;
                                                                                if (J < 10) {
                                                                                    if (pos[10] != -1) {
                                                                                        endK = 13;
                                                                                    } else {
                                                                                        endK = 11;
                                                                                    }
                                                                                }
                                                                                for K in 0..endK {
                                                                                    if (pos[K] == -1) {
                                                                                        pos[K] = 10;
                                                                                        let mut endL = 10;
                                                                                        if (K < 10) {
                                                                                            if (pos[10] != -1) {
                                                                                                endL = 13;
                                                                                            } else {
                                                                                                endL = 11;
                                                                                            }
                                                                                        }
                                                                                        for L in 0..endL {
                                                                                            if (pos[L] == -1) {
                                                                                                pos[L] = 11;
                                                                                                // last loop
                                                                                                let mut endM = 10;
                                                                                                for M in 0..endM {
                                                                                                    if (pos[M] == -1) {
                                                                                                        if (check_result(&[A, B, C, D, E, F, G, H, I, J, K, L, M])) {
                                                                                                            found += 1;
                                                                                                        }
                                                                                                        count += 1;
                                                                                                    }
                                                                                                }
                                                                                                pos[L] = -1
                                                                                            }
                                                                                        }
                                                                                        pos[K] = -1
                                                                                    }
                                                                                }
                                                                                pos[J] = -1
                                                                            }
                                                                        }
                                                                        pos[I] = -1
                                                                    }
                                                                }
                                                                pos[H] = -1
                                                            }
                                                        }
                                                        pos[G] = -1
                                                    }
                                                }
                                                pos[F] = -1
                                            }
                                        }
                                        pos[E] = -1
                                    }
                                }
                                pos[D] = -1
                            }
                        }
                        println!("{} {} {} {} / {} ", A, B, C, found, count);
                        pos[C] = -1
                    }
                }
                pos[B] = -1
            }
        }
        pos[A] = -1;
    }
    println!("{} / {} ", found, count);
}



