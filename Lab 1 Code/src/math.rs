// math.rs

// These values are pulled from eariler in the homework.
const SERIES_A_X: f64 = -0.9;
const SERIES_B_X: f64 = 0.9 / 2.9;

// Given by the approximation functions on the homework.
pub const SERIES_A_SCALAR: f64 = -1.0;
pub const SERIES_B_SCALAR: f64 = 2.0;

/// Finds the sum of all elements in a vector.
pub fn sum_vector_elements(vec: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;

    for each in vec {
        sum += each;
    }

    sum
}

/// Returns the number of terms required to calculate to a desired precision.
pub fn get_required_terms(series: &mut Vec<f64>, method: fn(i32) -> f64, scalar: f64) -> i32 {
    let ln_one_point_nine = 1.9_f64.ln();

    series.clear();

    // Use a loop to try increasingly many terms to find when error < 1e-10
    loop {
        series.push(method((series.len() + 1) as i32));

        if (ln_one_point_nine - scalar * sum_vector_elements(&series)).abs() < 1e-10 {
            break;
        }
    }

    return series.len() as i32;
}

/// Calculates the value of the nth term of Series A
pub fn gen_series_a_term(term_degree: i32) -> f64 {
    SERIES_A_X.powi(term_degree) / term_degree as f64
}

/// Calculates the value of the nth term of Series B
pub fn gen_series_b_term(term_degree: i32) -> f64 {
    SERIES_B_X.powi(2*term_degree - 1) / (2*term_degree - 1) as f64
}

