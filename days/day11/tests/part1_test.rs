use day11::part1;

#[test]
fn test_solve() {
    let input = "125 17";
    assert_eq!(part1::solve(&input), 55312);
}