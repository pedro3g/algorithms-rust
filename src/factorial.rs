pub fn factorial(input: usize) -> u32 {
    let mut arr = vec![1; input];

    for i in 1..input {
        arr[i] = (i as u32) + 1;
    }

    return arr.iter().fold(1, |acc, x| acc * x);
}
