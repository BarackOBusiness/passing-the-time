// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut newmatrix: [[i32; 3]; 3] = [[0; 3]; 3];

    for r in 0..matrix.len() {
        for c in 0..matrix[0].len() {
            newmatrix[r][c] = matrix[c][r];
        }
    }
    
    newmatrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    let mut bruh = String::from("");

    let end = matrix.len() - 1;
    let end_inner = matrix[0].len();

    for r in 0..matrix.len() {
        match r {
            0 => bruh.push_str("⎡"),
            a if a == end => bruh.push_str("⎣"),
            _ => bruh.push_str("⎥"),
        }

        for c in 0..matrix[r].len() {
            bruh.push_str(" ");
            bruh.push_str((matrix[r][c].to_string() + " ").as_str());
        }

        match r {
            0 => bruh.push_str("⎤\n"),
            a if a == end => bruh.push_str("⎦\n"),
            _ => bruh.push_str("⎥\n"),
        }
    }

    println!("{}", bruh);
}

pub fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}