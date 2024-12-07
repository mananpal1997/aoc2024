use day6::part1;

#[test]
fn test_solve() {
    let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    assert_eq!(part1::solve(&input), 41);
}