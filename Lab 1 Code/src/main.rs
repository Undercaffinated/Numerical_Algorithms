


// These values are pulled from eariler in the homework.
const SERIES_A_X: f64 = -0.9;
const SERIES_B_X: f64 = 0.9 / 2.9;



fn main() {
    let ln_one_point_nine = 1.9_f64.ln();

    // Get a value for n. We will assume the user knows how to properly use the code.
    // The number of Taylor Polynomial Terms we will be using.
    let n: i32 = get_n();
    println!();

    // Allocate space for the resutls
    let mut series_a_terms: Vec<f64> = Vec::new();
    let mut series_b_terms: Vec<f64> = Vec::new();

    // Use a loop to try different values of n to find when error < 1e-10
    // In theory, we could use a binary search, not a linear search, but there aren't bonus
    // points for optimization on this assignment.
    let mut a: i32 = 1;
    loop {
        series_a_terms.push(gen_series_a_term(a));

        if (ln_one_point_nine + sum_vector_elements(&series_a_terms)).abs() < 1e-10 {
            break;
        }

        else {
            a += 1;
            continue;
         }
    }

    let mut b: i32 = 1;
    loop {
        series_b_terms.push(gen_series_b_term(b));

        if (ln_one_point_nine - 2.0*sum_vector_elements(&series_b_terms)).abs() < 1e-10 {
            break;
        }

        else {
            b += 1;
            continue;
         }
    }

    println!("To reach err < 1e-10...");
    println!("Series A took {} terms.", a);
    println!("Series B took {} terms.", b);
    println!();

    series_a_terms.clear();
    series_b_terms.clear();

    // Calculate the terms
    for i in 1..=n {
        series_a_terms.push(gen_series_a_term(i));
        series_b_terms.push(gen_series_b_term(i));
    }

    // Calculate the sums
    let series_a_sum: f64 = sum_vector_elements(&series_a_terms);
    let series_b_sum: f64 = sum_vector_elements(&series_b_terms);

    // Apply scalars
    let series_a_sum: f64 = series_a_sum * -1.0;
    let series_b_sum: f64 = series_b_sum * 2.0;

    // Calculate the error. TODO!

    println!("With {} terms, these are the results:", n);
    println!("ln(1.9) = {}", 1.9_f64.ln());
    println!("Series A: {}", series_a_sum);
    println!("Series B: {}", series_b_sum);
    println!();


    
}




/// Finds the sum of all elements in a vector.
fn sum_vector_elements(vec: &Vec<f64>) -> f64 {
    let mut sum: f64 = 0.0;

    for each in vec {
        sum += each;
    }

    sum
}

/// Calculates the value of the nth term of Series A
fn gen_series_a_term(term_degree: i32) -> f64 {
    SERIES_A_X.powi(term_degree) / term_degree as f64
}

/// Calculates the value of the nth term of Series B
fn gen_series_b_term(term_degree: i32) -> f64 {
    SERIES_B_X.powi(2*term_degree - 1) / (2*term_degree - 1) as f64
}

/// Gets an integer n from the user. Will panic if the input cannot be parsed.
fn get_n() -> i32 {
    println!("n: ");
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}




#[test]
fn gen_series_a_term_test() {
    println!("{}", gen_series_a_term(4));
}

#[test]
fn push_to_vec() {
    let mut v: Vec<f64> = Vec::new();
    for each in 0..10 {
        v.push(each as f64);
    }
}