use itertools::Itertools;

use crate::util::translate_radix;

const IN_MIN: i32 = 1;
const IN_MAX: i32 = 1000;
const IN_MARKER: i32 = IN_MAX + 1;
const IN_RADIX: u32 = IN_MAX as u32 - IN_MIN as u32 + 1;
fn in_from_user(i: i32) -> i32 {
    i - IN_MIN
}
fn in_to_user(i: i32) -> i32 {
    i + IN_MIN
}
const OUT_MIN: u8 = b' ';
const OUT_MAX: u8 = b'~';
const OUT_RADIX: u32 = OUT_MAX as u32 - OUT_MIN as u32 + 1;
fn out_from_user(i: u8) -> u8 {
    i - OUT_MIN
}
fn out_to_user(i: u8) -> u8 {
    i + OUT_MIN
}

fn encode_inner(input: &[i32]) -> Vec<(Vec<(i32, i32)>, usize)> {
    let mut input = input.iter().collect::<Vec<_>>();
    input.sort();
    let mut input = input
        .iter()
        .map(|it| (it, 1usize))
        .coalesce(|aa @ (a, acnt), bb @ (b, bcnt)| {
            if a == b {
                Ok((a, acnt + bcnt))
            } else {
                Err((aa, bb))
            }
        })
        .collect::<Vec<_>>();
    input.sort_by_key(|(_, cnt)| *cnt);
    let input = input
        .iter()
        .map(|(&n, c)| (vec![n], c))
        .coalesce(|(a, ac), (b, bc)| {
            if ac == bc {
                Ok((a.iter().cloned().chain(b.iter().cloned()).collect(), ac))
            } else {
                Err(((a, ac), (b, bc)))
            }
        })
        .collect::<Vec<_>>();

    let input = input
        .iter()
        .map(|(v, &cnt)| {
            (
                v.iter()
                    .map(|&&a| (a, 0))
                    .coalesce(|(a, ac), (b, bc)| {
                        if b == a + ac + 1 {
                            Ok((a, ac + bc + 1))
                        } else {
                            Err(((a, ac), (b, bc)))
                        }
                    })
                    .collect::<Vec<_>>(),
                cnt,
            )
        })
        .collect::<Vec<_>>();
    input
}

#[test]
fn test_encode_inner() {
    assert_eq!(encode_inner(&[1, 2, 3, 4, 5]), vec![(vec![(1, 4)], 1)]);
    assert_eq!(
        encode_inner(&[1, 2, 3, 3, 4, 5]),
        vec![(vec![(1, 1), (4, 1)], 1), (vec![(3, 0)], 2)]
    );

    println!("{:?}", encode_inner(&[1, 2, 3, 4, 5, 3]));
}

fn decode_inner(input: &[(Vec<(i32, i32)>, usize)]) -> Vec<i32> {
    let mut input = input
        .iter()
        .map(|(vec, sz)| {
            (
                vec.iter()
                    .flat_map(|&(n, cnt)| (n..).take(cnt as usize + 1))
                    .collect::<Vec<_>>(),
                sz,
            )
        })
        .flat_map(|(vec, &repeat)| vec![vec; repeat])
        .flatten()
        .collect::<Vec<_>>();
    input.sort();
    input
}

#[test]
fn test_decode_inner() {
    assert_eq!(
        vec![1, 2, 3, 3, 4, 5],
        decode_inner(&[(vec![(1, 1), (4, 1)], 1), (vec![(3, 0)], 2)])
    );
}

fn encode_after(input: &[(Vec<(i32, i32)>, usize)]) -> Vec<u8> {
    let mut output: Vec<i32> = Vec::new();
    for (vec, cnt) in input {
        for &(n, repeat) in vec {
            output.push(n);
            output.push(repeat);
        }
        output.push(in_from_user(IN_MARKER));
        output.push(*cnt as _)
    }
    translate_radix(&output, IN_RADIX + 1, OUT_RADIX)
        .iter()
        .map(|&x| out_to_user(x as u8))
        .collect()
}

fn decode_after(input: &[u8]) -> Vec<(Vec<(i32, i32)>, usize)> {
    let input = input
        .iter()
        .map(|&x| out_from_user(x) as i32)
        .collect::<Vec<_>>();
    let input = translate_radix(&input, OUT_RADIX, IN_RADIX + 1);
    let mut result: Vec<(Vec<(i32, i32)>, usize)> = Vec::new();
    let mut coll: Vec<(i32, i32)> = Vec::new();
    {
        let mut input = input.iter();
        while let Some(&i) = input.next() {
            if i == in_from_user(IN_MARKER) {
                result.push((coll.clone(), *input.next().unwrap() as _));
                coll.clear();
            } else {
                coll.push((i, *input.next().unwrap()));
            }
        }
    }

    result
}

pub fn encode(input: &[i32]) -> Vec<u8> {
    let input = input.iter().cloned().map(in_from_user).collect::<Vec<_>>();
    encode_after(&encode_inner(&input))
}

pub fn decode(input: &[u8]) -> Vec<i32> {
    let input = decode_inner(&decode_after(input));
    input.iter().cloned().map(in_to_user).collect::<Vec<_>>()
}

#[test]
fn encode_decode() {
    let t1 = vec![1, 2, 3, 4, 5];
    let t2 = vec![1, 2, 3, 3, 4, 5];
    let t3 = vec![1, 1, 1, 2, 2, 3, 3, 4, 5, 6, 7, 7];
    let t4 = vec![IN_MIN, IN_MIN, 10, 10, 10, IN_MAX, IN_MAX];
    assert_eq!(&decode(&encode(&t1)), &t1);
    assert_eq!(&decode(&encode(&t2)), &t2);
    assert_eq!(&decode(&encode(&t3)), &t3);
    assert_eq!(&decode(&encode(&t4)), &t4);
}
