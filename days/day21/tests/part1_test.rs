use day21::part1;

#[test]
fn test_solve() {
    let input = "029A
980A
179A
456A
379A";
    assert_eq!(part1::solve(&input), 126384);
}