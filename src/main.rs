fn main() {
    let a = vec![
        vec![rand::random::<i32>() % 100, rand::random::<i32>() % 100],
        vec![rand::random::<i32>() % 100, rand::random::<i32>() % 100],
    ];
    let b = vec![
        vec![rand::random::<i32>() % 100, rand::random::<i32>() % 100],
        vec![rand::random::<i32>() % 100, rand::random::<i32>() % 100],
    ];
    for i in 1..8 {
        let count = 10i32.pow(i);
        let start = std::time::Instant::now();
        for _ in 0..count {
            naive_multiplication(&a, &b);
        }
        let duration = start.elapsed();
        println!(
            "Naive Multiplication: {} - {}ns - {}ns/kiloiteration",
            count,
            duration.as_nanos(),
            (duration.as_nanos() * 1000) / count as u128,
        );
    }

    for i in 1..8 {
        let count = 10i32.pow(i);
        let start = std::time::Instant::now();
        for _ in 0..count {
            loop_unroll(&a, &b);
        }
        let duration = start.elapsed();
        println!(
            "Loop Unroll Multiplication: {} - {}ns - {}ns/kiloiteration",
            count,
            duration.as_nanos(),
            (duration.as_nanos() * 1000) / count as u128,
        );
    }
}

fn loop_unroll(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let r1c1 = g(&a, 1, 1) * g(&b, 1, 1) + g(&a, 1, 2) * g(&b, 2, 1);
    let r1c2 = g(&a, 1, 1) * g(&b, 1, 2) + g(&a, 1, 2) * g(&b, 2, 2);
    let r2c1 = g(&a, 2, 1) * g(&b, 1, 1) + g(&a, 2, 2) * g(&b, 2, 1);
    let r2c2 = g(&a, 2, 1) * g(&b, 1, 2) + g(&a, 2, 2) * g(&b, 2, 2);

    vec![vec![r1c1, r1c2], vec![r2c1, r2c2]]
}

fn naive_multiplication(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    for i in 0..a.len() {
        let mut row = Vec::new();
        for j in 0..b[0].len() {
            let mut sum = 0;
            for k in 0..a[0].len() {
                sum += a[i][k] * b[k][j];
            }
            row.push(sum);
        }
        result.push(row);
    }
    result
}

fn g(matrix_one: &Vec<Vec<i32>>, row: usize, column: usize) -> i32 {
    matrix_one[row - 1][column - 1]
}

