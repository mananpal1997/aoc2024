use day1::part2;

#[test]
fn test_solve() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(part2::solve(&input), 31);
}