fn main() {
    let min = 357253;
    let max = 892942;
    let mut count = 0;

    for dig1 in 3..=8 {
        for dig2 in dig1..=9 {
            for dig3 in dig2..=9 {
                for dig4 in dig3..=9 {
                    for dig5 in dig4..=9 {
                        for dig6 in dig5..=9 {
                            if dig1 != dig2
                                && dig2 != dig3
                                && dig3 != dig4
                                && dig4 != dig5
                                && dig5 != dig6
                            {
                                continue;
                            }
                            let num = ((((dig1 * 10 + dig2) * 10 + dig3) * 10 + dig4) * 10 + dig5)
                                * 10
                                + dig6;
                            if num < min || num > max {
                                continue;
                            }

                            count += 1;
                            println!("Found example {}", num);
                        }
                    }
                }
            }
        }
    }

    println!("Found {} examples", count);
}
