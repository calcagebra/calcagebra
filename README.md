# Calcagebra

Programming language for evaluating mathematical expressions.

```rust
let a = [1, 2, -3; 0, -5, 1]
let b = [3; 5; 6]
let c = 3 + 4i

print(69*420) // 28980
print(|c|) // 5
print(a*b)
// ┌     ┐
// │ -5  │
// │ -19 │
// └     ┘
```

## Installation

### Topocatber (recommended)

[Topocatber](https://github.com/calcagebra/topocatber) is the official manager for calcagebra installations.

1. Install topocatber: `curl --proto '=https' --tlsv1.2 -sLSf https://raw.githubusercontent.com/calcagebra/topocatber/refs/heads/main/install.sh | sh`
2. View avaliable versions for calcagebra: `topocatber list`
3. Install your preferred version: `topocatber install vx.y.z`

### Manual

Download the binary for your OS from the [releases page.](https://github.com/calcagebra/calcagebra/releases/latest)

## Commands

* `calcagebra repl`: Open the REPL, use CTRL+C/CTRL+D to exit the repl.
* `calcagebra run INPUT`: Run the contents of the file, if a directory is provided or file is not present then an error is thrown.
* `calcagebra --version`: Print the verson.

## Documentation

Documentation can be found in the [docs](/docs) directory.

## Contributing

All contributions are welcome as long as they are formatted and linted by cargo and clippy.
