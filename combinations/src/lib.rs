#![forbid(unsafe_code)]

pub fn combinations(arr: &[i32], k: usize) -> Vec<Vec<i32>> {
    let n = arr.len();
    if k > n {
        return Vec::new();
    }
    if k == 0 {
        return vec![Vec::new()];
    }

    let mut result = Vec::new();
    for (i, &val) in arr.iter().enumerate().take(n - k + 1) {
        let sub = combinations(&arr[i + 1..], k - 1);
        result.extend(sub.into_iter().map(|mut comb| {
            comb.insert(0, val);
            comb
        }));
    }

    result
}