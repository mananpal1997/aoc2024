use day17::part2;

#[test]
fn test_solve() {
    let input = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
    assert_eq!(part2::solve(&input), 117440);
}