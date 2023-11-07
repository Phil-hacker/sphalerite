#[macro_use]
mod transcode;

#[cfg(test)]
mod tests {
    use std::collections::{HashSet, HashMap};

    use super::transcode::Transcode;
    use rand::{distributions::Standard, distributions::Distribution, random, thread_rng, Rng};
    const TEST_SIZE: usize = 1 << 16;
    macro_rules! test_type {
        ($type: ty) => {{
            let mut values: Vec<$type> = Vec::with_capacity(TEST_SIZE);
            for _ in 0..TEST_SIZE {
                values.push(random());
            }
            let mut data: Vec<u8> = vec![];
            for value in values.iter() {
                (*value).to_bytes(&mut data).unwrap();
            }
            let mut new_values = Vec::with_capacity(TEST_SIZE);
            let mut slice = data.as_slice();
            for _ in 0..TEST_SIZE {
                new_values.push(<$type>::from_bytes(&mut slice).unwrap());
            }
            assert_eq!(values, new_values);
        }};
    }
    #[test]
    fn test_u8() {
        test_type!(u8);
    }
    #[test]
    fn test_u16() {
        test_type!(u16)
    }
    #[test]
    fn test_u32() {
        test_type!(u32)
    }
    #[test]
    fn test_u64() {
        test_type!(u64)
    }
    #[test]
    fn test_u128() {
        test_type!(u128)
    }
    #[test]
    fn test_i8() {
        test_type!(i8);
    }
    #[test]
    fn test_i16() {
        test_type!(i16)
    }
    #[test]
    fn test_i32() {
        test_type!(i32)
    }
    #[test]
    fn test_i64() {
        test_type!(i64)
    }
    #[test]
    fn test_i128() {
        test_type!(i128)
    }
    #[test]
    fn test_vec() {
        let mut values: Vec<Vec<u64>> = Vec::with_capacity(TEST_SIZE);
        let mut rng = thread_rng();
        for _ in 0..TEST_SIZE {
            let range = rng.gen_range(0..256);
            values.push(Standard.sample_iter(&mut rng).take(range).collect());
        }
        let mut data: Vec<u8> = vec![];
        for value in values.iter() {
            (*value).to_bytes(&mut data).unwrap();
        }
        let mut new_values = Vec::with_capacity(TEST_SIZE);
        let mut slice = data.as_slice();
        for _ in 0..TEST_SIZE {
            new_values.push(<Vec<u64>>::from_bytes(&mut slice).unwrap());
        }
        assert_eq!(values, new_values);
    }
    #[test]
    fn test_hashset() {
        let mut values: Vec<HashSet<u64>> = Vec::with_capacity(TEST_SIZE);
        let mut rng = thread_rng();
        for _ in 0..TEST_SIZE {
            let range = rng.gen_range(0..256);
            values.push(Standard.sample_iter(&mut rng).take(range).collect());
        }
        let mut data: Vec<u8> = vec![];
        for value in values.iter() {
            (*value).to_bytes(&mut data).unwrap();
        }
        let mut new_values = Vec::with_capacity(TEST_SIZE);
        let mut slice = data.as_slice();
        for _ in 0..TEST_SIZE {
            new_values.push(<HashSet<u64>>::from_bytes(&mut slice).unwrap());
        }
        assert_eq!(values, new_values);
    }
    #[test]
    fn test_hashmap() {
        let mut values: Vec<HashMap<u64,u64>> = Vec::with_capacity(TEST_SIZE);
        let mut rng = thread_rng();
        for _ in 0..TEST_SIZE {
            let range = rng.gen_range(0..256);
            values.push(Standard.sample_iter(&mut rng).take(range).collect());
        }
        let mut data: Vec<u8> = vec![];
        for value in values.iter() {
            (*value).to_bytes(&mut data).unwrap();
        }
        let mut new_values = Vec::with_capacity(TEST_SIZE);
        let mut slice = data.as_slice();
        for _ in 0..TEST_SIZE {
            new_values.push(<HashMap<u64,u64>>::from_bytes(&mut slice).unwrap());
        }
        assert_eq!(values, new_values);
    }
}
