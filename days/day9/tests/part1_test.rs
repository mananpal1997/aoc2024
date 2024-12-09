use day9::part1;

#[test]
fn test_solve() {
    let input = "2333133121414131402";
    assert_eq!(part1::solve(&input), 1928);
}