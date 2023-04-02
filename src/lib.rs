use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

pub trait ConstZero {
    const ZERO: Self;
}
pub trait ConstOne {
    const ONE: Self;
}

macro_rules! const_zero_one_impl {
    ($($type:ty = ($zero:expr, $one:expr),)*) => {
        $(
            impl ConstZero for $type {
                const ZERO: Self = $zero;
            }
            impl ConstOne for $type {
                const ONE: Self = $one;
            }
        )*
    };
}

const_zero_one_impl! {
    u8 = (0, 1), u16 = (0, 1), u32 = (0, 1), u64 = (0, 1), usize = (0, 1),
    i8 = (0, 1), i16 = (0, 1), i32 = (0, 1), i64 = (0, 1), isize = (0, 1),
    f64 = (0.0, 1.0), f32 = (0.0, 1.0),
}

pub mod rect2;
pub mod vec2;
