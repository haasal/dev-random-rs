/// Scales a single value x into a range between a and b
///
/// # Arguments
/// * `x` - The value to scale
/// * `x_min` - The minimum value of the array
/// * `x_max` - The maximum value of the array
/// * `a` - The minimum value of the scaled value
/// * `b` - The maximum value of the scaled value
fn min_max_value(x: u8, x_min: u8, x_max: u8, a: u8, b: u8) -> u8 {
    a + (((x - x_min) as u16 * (b - a) as u16) / (x_max - x_min) as u16) as u8
}

/// Scales a whole array to a range between a and b
///
/// # Arguments
/// * `x_arr` - The array to scale
/// * `a` - Lower
/// * `b` - Upper
fn min_max_array(x_arr: Vec<u8>, a: u8, b: u8) -> Vec<u8> {
    let (x_min, x_max) = (*x_arr.iter().min().unwrap(), *x_arr.iter().max().unwrap());

    x_arr
        .into_iter()
        .map(|x| min_max_value(x, x_min, x_max, a, b))
        .collect()
}

pub fn scale_to_utf8(arr: Vec<u8>) -> Vec<u8> {
    min_max_array(arr, 0x21, 0x7e)
}

#[cfg(test)]
mod tests {
    use super::{min_max_array, min_max_value};

    #[test]
    fn test_int_divide() {
        assert_eq!(42 / 7, 6);
    }

    #[test]
    fn test_min_max_value() {
        let scaled_inside = min_max_value(38, 3, 51, 6, 42);
        let scaled_outside = min_max_value(39, 3, 41, 6, 32);

        assert_eq!(scaled_inside, 32);
        assert_eq!(scaled_outside, 30);
    }

    #[test]
    fn test_min_max_array() {
        let arr = vec![3u8, 42, 51, 32, 12, 73];
        let scaled = min_max_array(arr, 6, 65);
        assert_eq!(scaled, vec![6, 38, 46, 30, 13, 65])
    }
}
