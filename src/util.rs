use num_bigint::BigInt;
use num_traits::Zero;
use std::iter::from_fn;

pub fn translate_radix(input: &[i32], from: u32, to: u32) -> Vec<i32> {
    let mut num = BigInt::zero();
    for (i, &it) in input.iter().enumerate() {
        assert!(0 <= it && it < from as _);
        num += BigInt::from(it) * BigInt::from(from).pow(i as _);
    }
    from_fn(|| {
        if num.is_zero() {
            None
        } else {
            let rem = &num % to;
            num /= to;
            Some(if rem.is_zero() {
                0
            } else {
                rem.to_u32_digits().1[0] as _
            })
        }
    })
    .collect()
}

#[test]
fn test_translate_radix() {
    assert_eq!(translate_radix(&[], 10, 10), &[]);
    assert_eq!(translate_radix(&[1], 10, 10), &[1]);
    assert_eq!(
        translate_radix(&[1, 2, 3], 10, 2),
        &[1, 0, 0, 0, 0, 0, 1, 0, 1]
    );
    assert_eq!(
        translate_radix(&[1, 0, 0, 0, 0, 0, 1, 0, 1], 2, 10),
        &[1, 2, 3]
    );
    assert_eq!(
        translate_radix(&[28, 44, 30, 36], 95, 318),
        &[308, 299, 307]
    );
}
