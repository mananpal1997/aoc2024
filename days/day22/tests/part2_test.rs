use day22::part2;

#[test]
fn test_solve() {
    let input = "1
2
3
2024";
    assert_eq!(part2::solve(&input), 23);
}