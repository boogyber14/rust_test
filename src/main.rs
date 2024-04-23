/// Calculates the factorial of a given number.
///
/// # Examples
///
/// ```
/// assert_eq!(factorial(0), 1);
/// assert_eq!(factorial(1), 1);
/// assert_eq!(factorial(5), 120);
/// ```
///
/// # Panics
///
/// Panics if `n` is negative.
///
/// # Errors
///
/// Returns an error if the factorial of `n` cannot be calculated (e.g., overflow).
pub fn factorial(n: u32) -> Result<u32, &'static str> {
    if n == 0 {
        return Ok(1);
    }

    let mut result: u32 = 1; // Specify the type as u32
    let mut current: u32 = 1; // Specify the type as u32

    while current <= n {
        result = result.checked_mul(current)
            .ok_or("Factorial overflow")?;
        current += 1;
    }

    Ok(result)
}

fn main() {
    match factorial(10) {
        Ok(result) => println!("Factorial: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
