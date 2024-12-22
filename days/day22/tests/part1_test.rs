use day22::part1;

#[test]
fn test_solve() {
    let input = "1
10
100
2024";
    assert_eq!(part1::solve(&input), 37327623);
}