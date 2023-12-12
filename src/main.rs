/// Rust implementation of Shamir's Secret Sharing, Python version in GFG
/// Refer: https://www.geeksforgeeks.org/implementing-shamirs-secret-sharing-scheme-in-python/
use rand::Rng;
use std::env;

/// Combines individual shares (points on graph) using Lagranges interpolation.
///
/// `shares` is a list of points (x, y) on the curve that is our polynomial
/// The constant term of this polynomial is our secret key.
///
fn reconstruct_secret(shares: Vec<(i32, i32)>) -> i32 {
    let l = shares.len();
    let mut xi: i32;
    let mut yi: i32;
    let mut xj: i32;

    let mut product: f32;
    let mut sum = 0f32;

    for i in 0..l {
        (xi, yi) = shares[i];
        product = yi as f32;
        for j in 0..l {
            if i != j {
                (xj, _) = shares[j];
                product *= xj as f32 / ((xj - xi) as f32);
            }
        }
        sum += product;
    }

    sum as i32
}

/// Evaluates the polynomial we designed at a point
/// chosen randomly
/// n evaluations give n shares of our secret key
///
fn get_polynomial_value(x: i32, coeffs: &[i32]) -> i32 {
    let r = (coeffs.len() - 1) as usize;
    let mut poly = coeffs[r];
    for i in 0..r {
        let xpow: i32 = x.pow((r - i) as u32);
        poly += xpow * coeffs[i];
    }

    poly
}

/// Randomly generate a list of coefficients for a polynomial of degree `t` - 1,
/// where the constant term is our secret key.
/// For example with a set of coefficients (3,4,18,554), the polynomial is of
/// degree 3 and looks like this: 3x^3 + 4x^2 + 18x + 554
/// Here, 554 is our secret key.
fn get_coefficients(t: i32, s: i32) -> Vec<i32> {
    let mut coeffs: Vec<i32> = Vec::new();
    for _i in 0..t as usize {
        coeffs.push(rand::thread_rng().gen_range(0..100) as i32);
    }
    coeffs.push(s);

    coeffs
}

/// Split given `secret` into `n` shares with minimum threshold t
/// using Shamir's Secret Sharing algorithm.
///
fn get_shares(n: i32, t: i32, s: i32) -> Vec<(i32, i32)> {
    let coefficients: Vec<i32> = get_coefficients(t, s);
    let mut shares: Vec<(i32, i32)> = Vec::new();
    for _i in 1..(n + 1) as usize {
        let x = rand::thread_rng().gen_range(0..100) as i32;
        let y = get_polynomial_value(x, coefficients.as_slice());
        shares.push((x, y));
    }

    shares
}

/// Get input arguments as integers
fn get_int_args(args: Vec<String>) -> Vec<i32> {
    let mut args_int: Vec<i32> = Vec::new();

    for i in args {
        match i.trim().parse::<i32>() {
            Err(e) => panic!("Error in parsing input arguments: {}", e),
            Ok(v) => args_int.push(v),
        }
    }
    args_int
}

/// Driver code
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let args_int = get_int_args(args);
    let s = args_int[0];
    let n = args_int[1];
    let t = args_int[2];

    println!("Original Secret: {}", s);

    let shares = get_shares(n, t, s);
    println!("Shares: {:?}", shares);

    println!("Reconstructed secret: {}", reconstruct_secret(shares));
}
