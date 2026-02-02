const TOLERANCE: f64 = 1e-12;

fn main() {
    println!("{:10}", bisect(1.0, 2.0, 1));
}

/// Implementation of Bisection Method
/// Returns best approximation which is the midpoint between l and r when
/// r - l < TOLERANCE.
/// Note: Tolerance is a constant and thus does not need to be passed.
/// Note: The lab never specifies a maximum number of iterations. Since
/// it doesn't exist, it isn't getting passed.
/// Note: Depth parameter d added to count iterations. Iterations start at 1.
fn bisect(l: f64, r: f64, d: i32) -> f64 {
    let midpoint: f64 = (r+l) / 2.0;
    let range: f64 = r - l;

    println!("n: {},\t Estimate: {:.15},\t f(est): {:.15}", d, midpoint, f(midpoint));

    if range < TOLERANCE { return midpoint; }
    if d == 50 { return midpoint; }

    if f(midpoint).is_sign_positive() {
        return bisect(l, midpoint, d+1);
    }

    else {
        return bisect(midpoint, r, d+1);
    }
}

fn f(x: f64) -> f64 {
    x.powi(3) + 2.0 * x.powi(2) + 10.0 * x - 20.0
}

#[test]
fn f_test() {
    assert_eq!(f(0.0), -20.0);
    assert_eq!(f(1.0), -7.0);
    assert_eq!(f(2.0), 16.0);
}