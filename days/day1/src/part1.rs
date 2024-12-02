pub fn solve(_input: &str) -> i32 {
    let mut ret: i32 = 0;

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in _input.lines() {
        let mut numbers = line.split_whitespace();
        if let(Some(num1), Some(num2)) = (numbers.next(), numbers.next()) {
            if let(Ok(n1), Ok(n2)) = (num1.parse::<i32>(), num2.parse::<i32>()) {
                list1.push(n1);
                list2.push(n2);
            }
        }
    }

    list1.sort();
    list2.sort();

    for (n1, n2) in list1.iter().zip(list2.iter()) {
        ret += (n1 - n2).abs();
    }

    ret
}