pub fn gcd<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq + std::ops::Rem<Output = T> + Default + Copy,
{
    if b == T::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: std::cmp::PartialEq
        + std::ops::Rem<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + Default
        + Copy,
{
    a * b / gcd(a, b)
}

pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

pub fn mirror_rows<T>(v: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    for row in v.iter_mut() {
        row.reverse();
    }
}

pub fn mirror_cols<T>(v: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    let row_len = v.len();
    let col_len = v[0].len();
    for col in 0..col_len {
        for row in 0..(row_len / 2) {
            let temp = v[row][col];
            v[row][col] = v[row_len - 1 - row][col];
            v[row_len - 1 - row][col] = temp;
        }
    }
}

pub fn rotate_clockwise<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    let mut vt = transpose(v);
    mirror_rows(&mut vt);
    vt
}

pub fn rotate_counter_clockwise<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    assert!(!v.is_empty());
    let mut vt = transpose(v);
    mirror_cols(&mut vt);
    vt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let vt = transpose(v);
        assert_eq!(vt, vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]]);
    }

    #[test]
    fn test_mirror_rows() {
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        mirror_rows(&mut v);
        assert_eq!(v, vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]]);
    }

    #[test]
    fn test_mirror_cols() {
        let mut v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        mirror_cols(&mut v);
        assert_eq!(v, vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
    }

    #[test]
    fn test_rotate_clockwise() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let vt = rotate_clockwise(v);
        assert_eq!(vt, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }

    #[test]
    fn test_rotate_counter_clockwise() {
        let v = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let vt = rotate_counter_clockwise(v);
        assert_eq!(vt, vec![vec![3, 6, 9], vec![2, 5, 8], vec![1, 4, 7]]);
    }
}
