#[cfg(test)]
pub mod tests {
    use nom::{
        character::complete::{digit1, newline},
        combinator::{map, opt},
        multi::many1,
        sequence::terminated,
        IResult,
    };

    fn problem(x: usize, y: usize) -> String {
        std::fs::read_to_string(format!("{}/{}_{}.txt", env!("CARGO_MANIFEST_DIR"), x, y))
            .expect("No file")
    }

    #[test]
    fn day1() {
        let s = problem(1, 0);
        let val= || {
            map(
                many1(terminated(
                    map(digit1, |s: &str| s.parse::<u32>().unwrap()),
                    opt(newline),
                )),
                |v: Vec<u32>| v.iter().sum(),
            )
        };
        let res = {
            let x: IResult<&str, Vec<u32>> =
                many1(terminated(val(), opt(newline)))(&*s);

            let mut y = x.unwrap().1;
            y.sort();
            y
        };

        println!("{}", res.iter().max().unwrap());
        println!("{}", &res[res.len() - 3..].iter().sum::<u32>());
    }
}
