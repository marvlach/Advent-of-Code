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
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
