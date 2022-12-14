use core::ops::Add;

pub fn linear_interpolate<T>(arr: &[T], index: f32)->T
    where T: Copy + std::ops::Mul<f32> + Add<Output = T> + std::ops::Mul<f32, Output = T> + std::fmt::Display{
    let sum1: T = arr[index.floor() as usize] * (1.0 - index + index.floor());
    let sum2: T = arr[index.floor() as usize +1] * (1.0 - index.floor() - 1.0 + index);
    sum1 + sum2
}
