use day10::part2;

#[test]
fn test_solve() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part2::solve(&input), 81);
}