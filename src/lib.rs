#![allow(dead_code, unused_imports, unused_macros)]

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
fn day3_part2() {
    let mut lines = problem!(3, 1).lines().peekable();

    let mut total = 0;
    while lines.peek().is_some() {
        let mut three = [0_u64; 3];

        (&mut lines)
            .take(3)
            .zip(three.iter_mut())
            .for_each(|(line, val)| {
                for c in line.chars() {
                    *val |= 1_u64 << (c as u8 - b'A');
                }
            });

        let val = 1 + match (three[0] & three[1] & three[2]).trailing_zeros() as u8 + b'A' {
            v @ b'a'..=b'z' => v - b'a',
            v @ b'A'..=b'Z' => v - b'A' + 26,
            _ => panic!(),
        } as u64;
        total += val;
    }
    println!("{}", total);
}

#[test]
fn day3_part1() {
    let res: u32 = problem!(3, 1).lines().fold(0_u32, |acc, line| {
        let mut compartments = [0_u64; 2];
        for (index, comp) in compartments.iter_mut().enumerate() {
            let line_half_start = index * line.len() / 2;
            let line_half: &str = &line[line_half_start..line_half_start + line.len() / 2];
            for c in line_half.chars() {
                *comp |= 1_u64 << (c as u8 - b'A');
            }
        }

        1 + acc
            + match (compartments[0] & compartments[1]).trailing_zeros() as u8 + b'A' {
                v @ b'a'..=b'z' => v - b'a',
                v @ b'A'..=b'Z' => v - b'A' + 26,
                _ => panic!(),
            } as u32
    });
    println!("{}", res);
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
