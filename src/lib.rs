#![no_std]

/*

t = current time
b = start value
c = change in value
d = duration

*/

use core::ops::{Mul,Div,Add,Sub,Neg};

pub fn linear_tween<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy {
    c * t / d + b
}

pub fn in_quad<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Copy {
      let x = t / d;
      c * x * x + b  
}

pub fn out_quad<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> {
    let x = t / d;
    -c * x * (x - 2.0.into()) + b
}

pub fn in_out_quad<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + PartialOrd {
    let x = t / (d / 2.0.into());

    if x < 1.0.into() { 
        return c / 2.0.into() * x * x + b; 
    }

    let x = x - 1.0.into();
    -c / 2.0.into() * (x * (x - 2.0.into()) - 1.0.into()) + b
}

pub fn in_cubic<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let x = t / d;
    c * x * x * x + b
}

pub fn out_cubic<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + PartialOrd {
    let x = t / d - 1.0.into();
    c * (x * x * x + 1.0.into()) + b
}

pub fn in_out_cubic<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let x = t / (d / 2.0.into());

    if x < 1.0.into() {
        return c / 2.0.into() * x * x * x + b;
    }

    let x = x - 2.0.into();
    c / 2.0.into() * (x * x * x + 2.0.into()) + b
}

pub fn in_quart<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let x = t / d;
    c * x * x * x * x + b
}

pub fn out_quart<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let x = t / d - 1.0.into();
    -c * (x * x * x * x - 1.0.into()) + b
}

pub fn in_out_quart<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let x = t / (d / 2.0.into());

    if x < 1.0.into() {
        return c / 2.0.into() * x * x * x * x + b;
    }

    let x = x - 2.0.into();
    -c / 2.0.into() * (x * x * x * x - 2.0.into()) + b
}

pub fn in_quint<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let t = t / d;
    c * t * t * t * t * t + b
}

pub fn out_quint<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let t = t / d - 1.0.into();
    c * (t * t * t * t * t + 1.0.into()) + b
}

pub fn in_out_quint<T>(t: T, b: T, c: T, d: T) -> T where T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Neg<Output = T> + Copy + From<f32> + Ord {
    let t = t / (d / 2.0.into());

    if t < 1.0.into() {
        return c / 2.0.into() * t * t * t * t * t + b;
    }

    let t = t - 2.0.into();
    c / 2.0.into() * (t * t * t * t * t + 2.0.into()) + b
}
  
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear() {
        let result = linear_tween(0.5, 0.0, 1.0, 1.0);
        assert_eq!(result, 0.5);
    }

    #[test]
    fn test_linear_u8() {
        let result = linear_tween(0x02_u8, 0, 0x10, 0x10);
        assert_eq!(result, 0x02);

        let result = linear_tween(0x05_u8, 0, 0x10, 0x10);
        assert_eq!(result, 0x05);

        let result = linear_tween(0x08_u8, 0, 0x10, 0x10);
        assert_eq!(result, 0x08);
    }

    #[test]
    fn test_in_out_quad() {
        let result = in_out_quad(80.0, 0.0, 100.0, 100.0);
        assert_eq!(result, 92.0);
    }
}
