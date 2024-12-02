pub fn run(input: &String) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let mut line_data = line.split_whitespace().map(|num| num.parse::<i32>().unwrap());
        left_list.push(line_data.next().unwrap());
        right_list.push(line_data.next().unwrap());
    }

    let mut sum = 0i32;

    for num in left_list.iter() {
        sum += num * right_list.iter().filter(|&x| x == num).count() as i32;
    }

    println!("{}", sum);
}