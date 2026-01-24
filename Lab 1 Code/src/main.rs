mod math;
use math::*;


fn main() {
    // Allocate space for the taylor series terms' values.
    let mut series_a_terms: Vec<f64> = Vec::new();
    let mut series_b_terms: Vec<f64> = Vec::new();

    // Determine the minimum number of terms required to reach err < 1e-10 for both series.
    let required_terms_a: i32 = get_required_terms(&mut series_a_terms, gen_series_a_term, SERIES_A_SCALAR);
    let retuired_terms_b: i32 = get_required_terms(&mut series_b_terms, gen_series_b_term, SERIES_B_SCALAR);

    println!("To reach err < 1e-10...");
    println!("Series A took {} terms.", required_terms_a);
    println!("Series B took {} terms.", retuired_terms_b);
    println!();


    // From here on, we will be working with a user-specified number of terms, n.
    let n: i32 = get_n();
    println!();

    // Clears the vectors without freeing memory so they can be re-used in the next step.
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
    let series_a_sum: f64 = -series_a_sum;
    let series_b_sum: f64 = series_b_sum * 2.0;

    // Calculate the error. TODO!

    println!("With {} terms, these are the results:", n);
    println!("ln(1.9) = {}", 1.9_f64.ln());
    println!("Series A: {}", series_a_sum);
    println!("Series B: {}", series_b_sum);
    println!();
}




/// Gets an integer n from the user. Will panic if the input cannot be parsed.
fn get_n() -> i32 {
    println!("n: ");
    let mut input: String = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    input.trim().parse().unwrap()
}
