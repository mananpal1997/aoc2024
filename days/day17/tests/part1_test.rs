use day17::part1;

#[test]
fn test_solve() {
    let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
    assert_eq!(part1::solve(&input), "4,6,3,5,6,3,5,2,1,0");
}