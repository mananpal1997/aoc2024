use day3::part2;

#[test]
fn test_solve() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2::solve(&input), 48);
}