use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let t = vec![56.0, 97.0, 77.0, 93.0];
    let d = vec![499.0, 2210.0, 1097.0, 1440.0];
    solve(&t, &d);


    let t = vec![56977793.0];
    let d = vec![499221010971440.0];
    solve(&t, &d);

    eprintln!("{} ns", (Instant::now() - start_time).as_nanos());
}

/// The problem can be written in the form of a simple 2nd order polynom:
/// with :
///   T = time allowed
///   R = distance to beat
///   v = speed of the boat
/// we have the following equation system:
///   t = T - v = travel time
///   d = v * t
///   d > R
///
/// => -v² + T*v - R > 0
///
/// So all the valid values of the problem are the integer values between the two root of this polynom.
///
fn solve(t: &Vec<f64>, d: &Vec<f64>) {
    let mut result = 1;

    for i in 0..t.len() {
        let determinant_sqrt = (t[i].powi(2) - 4.0 * d[i]).sqrt();

        let v0 = (t[i] - determinant_sqrt) / 2.0;
        let v1 = (t[i] + determinant_sqrt) / 2.0;

        let t_min = v0.ceil() as i32;
        let t_max = v1.floor() as i32;

        let t_valid_count = t_max - t_min + 1;
        result *= t_valid_count;
    }

    println!("{}", result);
}