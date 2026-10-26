// Rewrite the factorial function using a `while` loop.
pub fn factorial(n: u32) -> u32 {
    // The `todo!()` macro is a placeholder that the compiler
    // interprets as "I'll get back to this later", thus
    // suppressing type errors.
    // It panics at runtime.

    let mut n = n;
    let mut fact = 1;
    while n > 0 {
        fact *= n;
        n -= 1;
    }
    fact
}
