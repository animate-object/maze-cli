# maze-cli
CLI tool for generating rectangular grid mazes. This tool was developed primarily for testing and bench marking the algorithms implemented in my [maze generation project](https://github.com/animate-object/mazes). 

# installation

```
$ cargo install https://github.com/animate-object/maze-clis
```

# usage
```
maze-cli [OPTIONS] <alg>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --corner <corner>              corner -- optionally specify a corner for algorithms that accept this argument,
                                       ignored otherwise [possible values: NW, NE, SW, SE]
        --direction <direction>        direction -- optionally specify a direction for algorithms that accept this
                                       argument, ignored otherwise [possible values: N, E, S, W]
    -h, --height <height>              height --  of the maze to be generated, in cells [default: 10]
    -o, --output-type <output-type>    output type -- "ascii" produces a nice command line visual, "bin" produces the
                                       maze's underlying byte encoding [default: ascii]  [possible values: BIN, ASCII]
    -w, --width <width>                width --  of the maze to be generated, in cells [default: 10]

ARGS:
    <alg>    maze algorithm -- choose from the implemented maze generation algorithms -- required [possible values:
             AB, SW, BT]
```
