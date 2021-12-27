const X_ADD: [i32; 14] = [10, 10, 14, 11, 14, -14, 0, 10, -10, 13, -12, -3, -11, -2];
const Z_DIV: [i32; 14] = [1, 1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 26, 26, 26];
const Z_ADD: [i32; 14] = [2, 4, 8, 7, 12, 7, 10, 14, 2, 6, 8, 11, 5, 11];

fn algo(w: &[i32; 14]) -> bool {
    let mut z = 0;

    for i in 0..14 {
        let x = (w[i] != (z % 26 + X_ADD[i])) as i32;
        if Z_DIV[i] == 26 && x == 1 {
                return false;
        }

        z /= Z_DIV[i];
        z *= 25 * x + 1;
        z += x * (w[i] + Z_ADD[i]);
    }

    z == 0
}

fn main() {
    for w0 in 1..=9 {
        for w1 in 1..=9 {
            for w2 in 1..=9 {
                for w3 in 1..=9 {
                    for w4 in 3..=9 {
                        let w5 = w4 - 2; {
                        // for w5 in 1..=9 {
                            for w6 in 8..=9 {
                                for w7 in 1..=5 {
                                    let w8 = w7 + 4; {
                                    // for w8 in 1..=9 {
                                        for w9 in 7..=9 {
                                            let w10 = w9 - 6; {
                                            // for w10 in 1..=9 {
                                                for w11 in 6..=9 {
                                                    for w12 in 1..=2 {
                                                        for w13 in 1..=9 {
                                                            if algo(&[
                                                                w0, w1, w2, w3, w4, w5, w6, w7, w8,
                                                                w9, w10, w11, w12, w13,
                                                            ]) {
                                                                println!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    w0,
                                                                    w1,
                                                                    w2,
                                                                    w3,
                                                                    w4,
                                                                    w5,
                                                                    w6,
                                                                    w7,
                                                                    w8,
                                                                    w9,
                                                                    w10,
                                                                    w11,
                                                                    w12,
                                                                    w13
                                                                );
                                                                return;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
