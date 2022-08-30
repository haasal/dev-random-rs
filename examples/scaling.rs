// def min_max_scale(x, a, b):
//     """
//     Min-Max scaling
//     """
//     return a + ((x - x.min()) * (b - a)) / (x.max() - x.min())

use random::Rng;

/// Scales a single value x into a range between a and b
///
/// # Arguments
/// * `x` - The value to scale
/// * `x_min` - The minimum value of the array
/// * `x_max` - The maximum value of the array
/// * `a` - The minimum value of the scaled value
/// * `b` - The maximum value of the scaled value
fn min_max_value(x: u8, x_min: u8, x_max: u8, a: u32, b: u32) -> u32 {
    let (x, x_min, x_max) = (x as u32, x_min as u32, x_max as u32);

    // a + ((x - x.min) * (b - a)) // (x.max() - x.min())
    a + ((x - x_min) * (b - a)) / (x_max - x_min)
}

fn min_max_array(x_arr: &[u8], a: u32, b: u32) -> Vec<u32> {
    let (x_min, x_max) = (*x_arr.iter().min().unwrap(), *x_arr.iter().max().unwrap());

    x_arr
        .iter()
        .map(|x| min_max_value(*x, x_min, x_max, a, b))
        .collect()
}

fn main() {
    let mut rng = Rng::with_cap(10);
    let output = rng.next_bytes();

    let scaled: Vec<u8> = min_max_array(output, 0x21, 0x7e)
        .into_iter()
        .map(|x| x as u8)
        .collect();

    let string = std::str::from_utf8(&scaled).expect("Invalid scaled array");

    println!(
        "Random output: {:?}\nScaled: {:?}\nAs String: {:?}",
        output, scaled, string
    )
}

#[cfg(test)]
mod tests {
    use crate::{min_max_array, min_max_value};

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
        let arr = [3, 42, 51, 32, 12, 73];
        let scaled = min_max_array(&arr, 6, 65);
        assert_eq!(scaled, vec![6, 38, 46, 30, 13, 65])
    }
}
