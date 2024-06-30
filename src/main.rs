fn main() {
    let mut rows = vec![vec![1, 2, 3],  vec![4, 5, 6], vec![ 7, 8, 9]];
    for row  in &rows {
        for num in row {
            print!("{} ", num);
        }
        println!();
    }

}
