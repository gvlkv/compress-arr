use crate::util::*;
use itertools::Itertools;

const IN_MIN: i32 = 1;
const IN_MAX: i32 = 300;
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

fn leb_encode(mut input: u32) -> Vec<u8> {
    let rad = OUT_RADIX / 2;
    let mut result = vec![];
    loop {
        let rem = (input % rad) as u8;
        input /= rad;
        if input == 0 {
            result.push(OUT_MIN + rem);
            break;
        } else {
            result.push(OUT_MIN + rad as u8 + rem);
        }
    }
    result
}

#[test]
fn test_leb_encode() {
    assert_eq!(leb_encode(0), vec![b' ']);
    assert_eq!(leb_encode(1), vec![b'!']);
    assert_eq!(leb_encode(1234), vec![b'[', b':']);
}

/// returns decoded number and payload position
fn leb_decode(input: &[u8]) -> (u32, usize) {
    let rad = OUT_RADIX / 2;
    let mut result = 0;
    for (i, &it) in input.iter().enumerate() {
        let it = it - OUT_MIN;
        result += (it as u32 % rad) * rad.pow(i as _);
        if it == it % rad as u8 {
            return (result, i + 1);
        }
    }
    panic!("no sentinel")
}

#[test]
fn test_leb_decode() {
    assert_eq!(leb_decode(&[b' ']), (0, 1));
    assert_eq!(leb_decode(&[b'!']), (1, 1));
    assert_eq!(leb_decode(&[b'[', b':']), (1234, 2));
}

/// `limit` is max repeat count
fn count_with_limit(input: &[i32], limit: i32) -> Vec<i32> {
    input
        .iter()
        .map(|&it| (it, 1))
        .coalesce(|aa @ (a, cnt_a), bb @ (b, cnt_b)| {
            if a == b && cnt_a + cnt_b <= limit {
                Ok((a, cnt_a + cnt_b))
            } else {
                Err((aa, bb))
            }
        })
        .flat_map(|(it, cnt)| {
            if cnt < 3 {
                // no reason to fold short sequence
                vec![it; cnt as _]
            } else {
                // optimization: 3 from the start
                vec![IN_MAX + cnt - 2, it]
            }
        })
        .collect()
}

fn uncount_with_limit(input: &[i32], limit: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut repeat_next: Option<i32> = None;
    for &it in input {
        if it > IN_MAX {
            repeat_next = Some(it - IN_MAX + 2);
        } else if let Some(repeat) = repeat_next {
            assert!(repeat <= limit);
            result.append(&mut vec![it; repeat as _]);
            repeat_next = None
        } else {
            result.push(it);
        }
    }
    result
}

#[test]
fn test_count_with_limit_uncount_with_limit() {
    let in1 = vec![IN_MAX; 2];
    let out1 = vec![IN_MAX; 2];
    let in2 = vec![IN_MAX; 3];
    let out2 = vec![IN_MAX + 1, IN_MAX];
    let in3 = vec![IN_MAX; 4];
    let out3 = vec![IN_MAX + 2, IN_MAX];
    let in4 = vec![IN_MAX; 11];
    let lim4 = 4;
    let out4 = vec![IN_MAX + 2, IN_MAX, IN_MAX + 2, IN_MAX, IN_MAX + 1, IN_MAX];
    let in5 = vec![
        IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX, IN_MAX,
        IN_MIN, IN_MIN, IN_MIN,
    ];
    let lim5 = 4;
    let out5 = vec![
        IN_MAX + 2,
        IN_MAX,
        IN_MAX + 2,
        IN_MAX,
        IN_MAX + 1,
        IN_MAX,
        IN_MAX + 1,
        IN_MIN,
    ];
    assert_eq!(&count_with_limit(&in1, 100), &out1);
    assert_eq!(&count_with_limit(&in2, 100), &out2);
    assert_eq!(&count_with_limit(&in3, 100), &out3);
    assert_eq!(&count_with_limit(&in4, lim4), &out4);
    assert_eq!(&count_with_limit(&in5, lim5), &out5);

    assert_eq!(&uncount_with_limit(&out1, 100), &in1);
    assert_eq!(&uncount_with_limit(&out2, 100), &in2);
    assert_eq!(&uncount_with_limit(&out3, 100), &in3);
    assert_eq!(&uncount_with_limit(&out4, lim4), &in4);
    assert_eq!(&uncount_with_limit(&out5, lim5), &in5);
}

fn encode_with_limit(input: &[i32], limit: i32) -> Vec<u8> {
    let limit = if limit < 3 { 2 } else { limit };
    let input = count_with_limit(input, limit)
        .iter()
        .copied()
        .map(in_from_user)
        .chain(std::iter::once(1)) // needed in case of ending in zeros
        .collect::<Vec<_>>();
    let header = leb_encode(limit as u32 - 2); // optimization
    let payload = translate_radix(&input, IN_RADIX + limit as u32 - 2, OUT_RADIX)
        .iter()
        .map(|&it| out_to_user(it as u8))
        .collect::<Vec<_>>();
    header.iter().chain(payload.iter()).copied().collect()
}

#[allow(unused)]
pub fn encode(input: &[i32]) -> Vec<u8> {
    let (_, max) = input
        .iter()
        .map(|x| (x, 1))
        .coalesce(|(a, cnta), (b, cntb)| {
            if a == b {
                Ok((a, cnta + cntb))
            } else {
                Err(((a, cnta), (b, cntb)))
            }
        })
        .max_by_key(|&(_, cnt)| cnt)
        .expect("empty");
    encode_with_limit(input, max)
}

#[allow(unused)]
pub fn decode(input: &[u8]) -> Vec<i32> {
    let (limit, start) = leb_decode(input);
    let limit = limit + 2;

    let input: Vec<i32> = input
        .iter()
        .skip(start)
        .map(|&it| out_from_user(it) as _)
        .collect();
    let input = translate_radix(&input, OUT_RADIX, IN_RADIX + limit - 2);
    assert_eq!(input.last().unwrap(), &1);
    let input = &input[..input.len() - 1];
    let input: Vec<i32> = input.iter().map(|&it| in_to_user(it)).collect();

    uncount_with_limit(&input, limit as _)
}

#[test]
fn test_encode_decode() {
    let data = vec![
        300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 300, 1, 1, 1,
    ];

    let a = encode(&data);
    assert_eq!(decode(&a), data);
}
