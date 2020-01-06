fn main() {
    let min = 357_253;
    let max = 892_942;
    let mut count = 0;

    for dig1 in 3..=8 {
        for dig2 in dig1..=9 {
            for dig3 in dig2..=9 {
                for dig4 in dig3..=9 {
                    for dig5 in dig4..=9 {
                        for dig6 in dig5..=9 {
                            let mut sets = vec![(dig1, 1)];
                            for x in vec![dig2, dig3, dig4, dig5, dig6].into_iter() {
                                if x == sets.last().unwrap().0 {
                                    sets.last_mut().unwrap().1 += 1;
                                } else {
                                    sets.push((x, 1));
                                }
                            }

                            if !sets.into_iter().any(|(_, len)| len == 2) {
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
