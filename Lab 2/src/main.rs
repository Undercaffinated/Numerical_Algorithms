const TOLERANCE: f64 = 1e-12;
const MAXIMUM_ITERATIONS: i32 = 50;

fn main() {
    println!("{:.10}", bisect(1.0, 2.0, 1, f));
    println!("{:.10}", newton(2.0, f, f_prime, 1));
}

/// Implementation of Bisection Method
/// Returns best approximation which is the midpoint between l and r when
/// r - l < TOLERANCE.
/// Note: Tolerance is a constant and thus does not need to be passed.
/// Note: The lab never specifies a maximum number of iterations. Since
/// it doesn't exist, it isn't getting passed. That said, even if it were
/// specified, it would be a constant and not needed to be passed.
/// Note: Depth parameter d added to count iterations. Iterations start at 1.
/// Note: Adding function parameter to allow inserting different functions,
/// improving modularity and testability.
fn bisect(l: f64, r: f64, d: i32, function: fn(f64) -> f64) -> f64 {
    let midpoint: f64 = (r+l) / 2.0;
    let range: f64 = r - l;

    // println!("n: {},\t Estimate: {:.15},\t f(est): {:.15}", d, midpoint, function(midpoint));

    if range < TOLERANCE { return midpoint; }
    if d == MAXIMUM_ITERATIONS { return midpoint; }

    if f(midpoint).is_sign_positive() {
        return bisect(l, midpoint, d+1, function);
    }

    else {
        return bisect(midpoint, r, d+1, function);
    }
}

/// Given a starting point x, approximate the nearest zero using the Newton method
/// on function ```f``` and that function's derivative ```d```.
fn newton(x: f64, f: fn(f64) -> f64, d: fn(f64) -> f64, iterations: i32) -> f64 {
    // Evaluate f(x).
    let fx: f64 = f(x);
    
    // If f(x) is close enough to 0, call it a zero.
    if fx.abs() < TOLERANCE { return x; }
    if iterations > MAXIMUM_ITERATIONS { return x; }

    // Else, evaluate the derivative and get a new test point. Then pass to recursive case.
    let dx: f64 = d(x);

    // Use the slope to calculate a new x-intercept. Derived from point-slope form.
    let new_intercept: f64 = x - fx / dx;

    // Recurse
    return newton(new_intercept, f, d, iterations+1);
}

fn f(x: f64) -> f64 {
    x.powi(3) + 2.0 * x.powi(2) + 10.0 * x - 20.0
}

fn f_prime(x: f64) -> f64 {
    3.0 * x.powi(2) + 4.0 * x + 10.0
}

#[test]
fn f_test() {
    assert_eq!(f(0.0), -20.0);
    assert_eq!(f(1.0), -7.0);
    assert_eq!(f(2.0), 16.0);
}
