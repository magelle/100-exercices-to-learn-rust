// Define a trait named `IsEven` that has a method `is_even` that returns a `true` if `self` is
// even, otherwise `false`.
//
// Then implement the trait for `u32` and `i32`.

pub trait IsEvent {
    fn is_even(&self) -> bool;
}

impl IsEvent for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

impl IsEvent for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}