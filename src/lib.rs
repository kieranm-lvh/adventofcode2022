#[cfg(test)]
pub mod tests {
    use nom::{
        character::complete::{digit1, newline},
        combinator::{map, opt},
        multi::many1,
        sequence::terminated,
        IResult,
    };

    macro_rules! problem {
        ($x:expr, $y: expr) => {
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/",
                $x,
                "_",
                $y,
                ".txt"
            ))
        };
    }

    #[test]
    fn day2_part1() {
        let res = problem!(2, 1).lines().fold(0, |acc, line| {
            let their = line.chars().nth(0).unwrap() as u32 - 65;
            let mine = line.chars().nth(2).unwrap() as u32 - 23 - 65;
            acc + 1
                + if their == mine {
                    mine + 3
                } else if ((their | 1 << 2) - (mine | 0 << 2)) % 3 != 0 {
                    mine
                } else {
                    mine + 6
                }
        });
        println!("{}", res);
    }

    #[test]
    fn day2_part2() {
        let res = problem!(2, 1).lines().fold(0, |acc, line| {
            let their = line.chars().nth(0).unwrap() as u32 - 65;
            let outcome = line.chars().nth(2).unwrap();
            acc + 1
                + match outcome {
                    'X' => {
                        if their == 0 {
                            2
                        } else {
                            their - 1
                        }
                    }
                    'Y' => their + 3,
                    'Z' => (their | 1 << 2) % 3 + 6,
                    _ => panic!(),
                }
        });
        println!("{}", res);
    }

    #[test]
    fn day1() {
        let s = problem!(1, 1);

        let x: IResult<&str, Vec<u32>> = many1(terminated(
            map(
                many1(terminated(
                    map(digit1, |s: &str| s.parse().unwrap()),
                    opt(newline),
                )),
                |v: Vec<u32>| v.iter().sum(),
            ),
            opt(newline),
        ))(s);

        let mut res = x.unwrap().1;
        res.sort();

        println!("{}", res.iter().max().unwrap());
        println!("{}", &res[res.len() - 3..].iter().sum::<u32>());
    }
}
