use num_traits::int::PrimInt;
use std::{collections::HashSet, hash::Hash};

fn posbit<T: PrimInt>(number: T, checker: T) -> T {
    if number | checker == number {
        number
    } else {
        !number
    }
}
fn and_all_posbit<'a, T: PrimInt + 'a>(numbers: impl Iterator<Item = &'a T>, checker: T) -> T {
    numbers.fold(!T::zero(), |flag, &number| flag & posbit(number, checker))
}
fn exist_posbit<T: PrimInt>(flag: T, target: T, checker: T) -> bool {
    !(flag & posbit(target, checker)) & flag != T::zero()
}
pub fn is_makable<T: PrimInt>(target: T, factors: &HashSet<T>) -> bool {
    let mut checker: T = T::one();
    for _ in 0..(size_of::<T>() * 8) {
        if exist_posbit(and_all_posbit(factors.iter(), checker), target, checker) {
            return false;
        }
        checker = checker << 1;
    }
    true
}
pub fn is_makable_detail<T: PrimInt + Hash>(target: T, factors: &HashSet<T>) -> HashSet<T> {
    let mut detail: HashSet<T> = HashSet::new();
    let mut checker: T = T::one();
    for _ in 0..(size_of::<T>() * 8) {
        let flag: T = and_all_posbit(factors.iter(), checker);
        if exist_posbit(flag, target, checker) {
            detail.insert(flag);
        }
        checker = checker << 1;
    }
    return detail;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn posbit_test() {
        assert_eq!(posbit::<u8>(0b00101001, 0b00100000), 0b00101001);
        assert_eq!(posbit::<u8>(0b00101001, 0b00010000), 0b11010110);
        assert_eq!(posbit::<u8>(0b00101001, 0b00001000), 0b00101001);
        assert_eq!(posbit::<u8>(0b00101001, 0b00000100), 0b11010110);
        assert_eq!(posbit::<u8>(0b00101001, 0b00000001), 0b00101001);
    }

    #[test]
    fn and_all_posbit_test() {
        assert_eq!(
            and_all_posbit::<u8>(vec![0b10101110, 0b01010101, 0b11111111,].iter(), 0b00000100),
            0b00000100
        );
    }

	#[test]
	fn exist_posbit_test() {
		assert!(exist_posbit::<u8>(0b00110011, 0b00001111, 0b00000001));
		assert!(!exist_posbit::<u8>(0b00000011, 0b00001111, 0b00000001));
	}

	#[test]
	fn is_makable_test() {
		assert!(!is_makable(0b101, &HashSet::from([0b100, 0b011, 0b000, 0b111])));
		assert!(is_makable(0b101, &HashSet::from([0b100, 0b001])));
		assert!(is_makable(0b101, &HashSet::from([0b101])));
	}

	#[test]
	fn is_makable_detail_test() {
		assert_eq!(is_makable_detail(0b101, &HashSet::from([0b100, 0b011, 0b000, 0b111])), HashSet::from([0b011]));
		assert_eq!(is_makable_detail(0b10111, &HashSet::from([0b10101, 0b01010])), HashSet::from([0b01010]));
	}
}
