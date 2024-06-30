struct Coordinate {
    row: usize,
    col: usize,
}
fn main() {
    let mut matrix: Vec<Vec<f32>> = vec![
        vec![1.0, 2.0, -1.0, -4.0],
        vec![2.0, 3.0, -1.0, -11.0],
        vec![-2.0, 0.0, -3.0, 22.0],
    ];
    println!("Original Matrix:");
    print_matrix(&matrix);
    
    rref(&mut matrix);
    sanitize_matrix(&mut matrix);
    println!("\nRREF:");
    print_matrix(&matrix);
}
fn print_matrix(matrix: &Vec<Vec<f32>>) {
    for row in matrix {
        for &val in row {
            // Format each element with a fixed width for better alignment
            print!("{:>8.2} ", val);
        }
        println!(); // Newline after each row
    }
}
fn rref(matrix: &mut Vec<Vec<f32>>) {
    let mut pivot = Coordinate { row: 0, col: 0 };
    for col in 0..matrix[0].len() {
        if pivot.row == matrix.len() {
            return;
        }
        if matrix[pivot.row][col] == 0.0 {
            pivot.col += 1;
            break;
        }
        let scale_number = 1.0 / matrix[pivot.row][col];
        //set the pivot element to the 1
        scale_row(matrix, pivot.row, scale_number);
        //reduce pivot columns
        reduce_pivot_col(matrix, &pivot);
        pivot.col += 1;
        pivot.row += 1;
    }
}
fn swap_rows(matrix: &mut Vec<Vec<f32>>, row1: usize, row2: usize) {
    matrix.swap(row1, row2);
}
fn scale_row(matrix: &mut Vec<Vec<f32>>, row: usize, scalar: f32) {
    for col in 0..matrix[row].len() {
        matrix[row][col] *= scalar;
    }
}
fn scale_vector(vector: &mut Vec<f32>, scalar: f32) {
    for col in 0..vector.len() {
        vector[col] *= scalar;
    }
}
fn reduce_pivot_col(matrix: &mut Vec<Vec<f32>>, pivot: &Coordinate) {
    for row in 0..matrix.len() {
        if row != pivot.row {
            //extract pivot row from matrix
            let mut add_vactor = matrix[pivot.row].clone();
            scale_vector(&mut add_vactor, -1.0 * matrix[row][pivot.col]);
            // set zero the elements of the above and below of the pivot elemet
            add_vactors(&mut matrix[row], &add_vactor);
        }
    }
}
fn add_vactors(first_vector: &mut Vec<f32>, second_vector: &Vec<f32>) {
    for i in 0..first_vector.len() {
        first_vector[i] += second_vector[i];
    }
}

fn sanitize_matrix(matrix: &mut Vec<Vec<f32>>) {
    for row in matrix.iter_mut() {
        for element in row.iter_mut() {
            if *element == -0.0 {
                *element = 0.0;
            }
        }
    }
}
