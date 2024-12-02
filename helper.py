import sys
import toml


def main(day):
    with open("./Cargo.toml") as f:
        cargo_toml = toml.load(f)
    cargo_toml["dependencies"][f"day{day}"] = {"path": f"days/day{day}"}
    with open("./Cargo.toml", "w") as f:
        f.write(toml.dumps(cargo_toml))
    
    with open("./src/main.rs") as f:
        lines = f.read().strip().split("\n")
    
    idx = next(i for i, line in enumerate(lines) if "// Add more match arms for other days and parts" in line)
    to_insert = [
        f'        ({day}, 1) => println!("{{}}", day{day}::part1::solve(&input)),',
        f'        ({day}, 2) => println!("{{}}", day{day}::part2::solve(&input)),',
    ]
    lines = lines[:idx] + to_insert + lines[idx:]
    with open("./src/main.rs", "w") as f:
        f.write("\n".join(lines))


if __name__ == "__main__":
    day = sys.argv[1]
    main(day)