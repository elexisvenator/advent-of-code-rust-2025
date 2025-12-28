pub mod template;

// Use this file to add helper functions and additional modules.
pub fn get_factors_unsorted(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec![1];
    }

    let mut factors = Vec::new();
    let mut end = n;
    let mut i = 1;
    while i < end {
        if n % i == 0 {
            factors.push(i);
            let other = n / i;
            if other != i {
                factors.push(other);
            }
            end = other;
        }
        i += 1;
    }

    factors
}
