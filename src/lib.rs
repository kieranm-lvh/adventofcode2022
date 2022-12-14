#![allow(dead_code, unused_imports, unused_macros)]

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_till, take_till1, take_until},
    character::{
        complete::{alpha0, alpha1, alphanumeric0, digit1, line_ending, newline, space0, space1},
        is_digit, is_newline,
    },
    combinator::{eof, map, map_res, not, opt},
    multi::{many0, many1},
    number::complete::u32,
    sequence::{preceded, terminated, tuple},
    IResult,
};
use std::{
    collections::{HashMap, HashSet},
    iter,
    marker::PhantomData,
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
fn day11_part1() {
    macro_rules! alias_types{
        () => {};
        ($root: ident) => {};
        ($root: ident, $x: ident) => {
            type $x = $root;
        };
        ($root: ident, $x: ident,$($more:tt)*) => {
            type $x = $root;
            alias_types!( $root,$($more)*)
        };
    }

    alias_types!(u32, MonkeyID, TestDivisor, ItemWorry);

    type Chunk<'a> = (
        MonkeyID,
        Vec<ItemWorry>,
        &'a str,
        TestDivisor,
        MonkeyID,
        MonkeyID,
    );

    macro_rules! to_u32 {
        ( $x:expr) => {
            map($x, |y| str::parse::<u32>(y).unwrap())
        };
    }

    let take_n = take::<usize, _, ()>;
    let digit1_at = |offset| preceded(take_n(offset), digit1);

    let chunk = tuple((
        to_u32!(terminated(
            preceded(take_n(7), digit1),
            tuple((tag(":"), newline))
        )), //Monkey 0:
        preceded(
            take_n(18),
            many1(to_u32!(terminated(digit1, opt(tag(", "))))),
        ), //  Starting items: 79, 98
        preceded(take_n(24), take_until("\n")), //   Operation: new = old * 19
        to_u32!(digit1_at(22)),                 //  Test: divisible by 23
        to_u32!(digit1_at(30)),                 //  If true: throw to monkey 1
        to_u32!(digit1_at(31)),                 //  If false: throw to monkey 3
    ));
    let parse_res: Vec<Chunk> = many1(terminated(chunk, opt(tag("\n\n"))))(problem!(11, 1))
        .unwrap()
        .1;

    #[derive(Debug)]
    struct State {
        monkey_has_items: Vec<Vec<ItemWorry>>,
        monkey_inspect_counts: Vec<usize>,
    }
    impl Default for State {
        fn default() -> Self {
            State {
                monkey_has_items: vec![vec![]; 10],
                monkey_inspect_counts: vec![0; 10],
            }
        }
    }

    let mut state = State::default();
    for (monkey_id, starting_items, ..) in &parse_res {
        state.monkey_has_items[*monkey_id as usize] = starting_items.clone();
    }
    for _round in 0..20 {
        for (monkey_id, _, operation, test_divisor, if_true_monkey, if_false_monkey) in &parse_res {
            let items: Vec<u32> = state.monkey_has_items[*monkey_id as usize].clone();
            for item in items {
                let x = operation[2..].parse::<u32>().unwrap_or(item);

                let inspect_worry = match operation.chars().nth(0).unwrap() {
                    '*' => item * x,
                    '-' => item - x,
                    '+' => item + x,
                    '/' => item / x,
                    _ => panic!(),
                };

                let post_inspect_worry = f32::floor(inspect_worry as f32 / 3f32) as u32;

                let test = post_inspect_worry % test_divisor == 0;
                let target_monkey_id = match test {
                    true => if_true_monkey,
                    false => if_false_monkey,
                };
                state.monkey_has_items[*target_monkey_id as usize].push(post_inspect_worry);
                state.monkey_inspect_counts[*monkey_id as usize] += 1;
            }
            state.monkey_has_items[*monkey_id as usize].clear();
        }
    }
    let mut sort_list = state.monkey_inspect_counts;
    sort_list.sort();
    println!("{:?}", sort_list.pop().unwrap() * sort_list.pop().unwrap());
}

// use paste::paste;
// #[test]
// fn day10_part1() {
//     struct State {
//         counter: u32,
//         register: i32,
//         buf: u8,
//     }

//     let input = problem!(10, 0);

//     input
//         .lines()
//         .fold((0, 1), |(c, x), v| match v.chars().nth(0) {
//             Some('n') => (c + 1, x),
//             Some('a') => (c + 2, x + str::parse::<i32>(&v[5..]).unwrap()),
//             None | Some(_) => panic!(),
//         });
// }

#[test]
fn day3_part2() {
    let total = problem!(3, 1)
        .lines()
        .chunks(3)
        .into_iter()
        .fold(0, |total, chunk| {
            (match chunk
                .map(|line| {
                    line.chars()
                        .fold(0_u64, |acc, c| acc | 1_u64 << (c as u8 - b'A'))
                })
                .fold(u64::MAX, |acc, next| acc & next)
                .trailing_zeros() as u8
                + b'A'
            {
                v @ b'a'..=b'z' => v - b'a',
                v @ b'A'..=b'Z' => v - b'A' + 26,
                _ => panic!(),
            } as u64)
                + 1
                + total
        });
    println!("{}", total);
}

#[test]
fn day3_part1() {
    let res: u32 = problem!(3, 1).lines().fold(0_u32, |acc, line| {
        let mut compartments = [0_u64, 1];
        for comp in compartments.iter_mut() {
            let line_half_start = *comp as usize * line.len() / 2;
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
