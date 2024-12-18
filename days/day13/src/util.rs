use regex::Regex;

/*
X = a*x1 + b*x2
Y = a*y1 + b*y2

b = (X - a*x1) / x2

Y = a*y1 + (X*y2 - a*x1*y2) / x2
Y*x2 = a*y1*x2 + X*y2 - a*x1*y2
Y*x2 - X*y2 = a (y1*x2 - x1*y2)
a = (Y*x2 - X*y2) / (y1*x2 - x1*y2)
 */

pub fn get_min_tokens(_input: &str, max_presses_allowed: i64, prize_offset: i64) -> i64 {
    let machine_pattern = Regex::new(r"Button [A-Z]: X\+(?<x1>\d+), Y\+(?<y1>\d+)\nButton [A-Z]: X\+(?<x2>\d+), Y\+(?<y2>\d+)\nPrize: X=(?<X>\d+), Y=(?<Y>\d+)").unwrap();

    machine_pattern.captures_iter(_input).map(|c| {
        let array: Vec<_> = c.iter().skip(1).map(|g| g.unwrap().as_str().parse::<i64>().unwrap()).collect();
        let [x1, y1, x2, y2, px, py] = array.try_into().unwrap();

        let n1 = (py + prize_offset) * x2 - (px + prize_offset) * y2;
        let d1 = y1 * x2 - x1 * y2;
        if n1 % d1 != 0 {
            return i64::MAX;
        }
        let a_presses = n1 / d1;
        if a_presses > max_presses_allowed {
            return i64::MAX;
        }
        
        let n2 = (px + prize_offset) - a_presses * x1;
        let d2 = x2;
        if n2 % d2 != 0 {
            return i64::MAX;
        }
        let b_presses = n2 / d2;
        if b_presses > max_presses_allowed {
            return i64::MAX;
        }
        a_presses * 3 + b_presses
    }).filter(|&tokens| tokens != i64::MAX).sum()
}