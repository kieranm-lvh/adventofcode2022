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
            include_str!(concat!("../", $x, "_", $y, ".txt"))
        };
    }

    #[test]
    fn day1() {
        let s = problem!(1, 0);

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
