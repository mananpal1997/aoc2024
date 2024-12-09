use day9::part2;

#[test]
fn test_solve() {
    let input = "2333133121414131402";
    assert_eq!(part2::solve(&input), 2858);
}