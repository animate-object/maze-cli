use exitfailure::ExitFailure;
use maze;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    pub enum AlgorithmArg {
        AB,
        SW,
        BT
    }
}

arg_enum! {
    #[derive(Debug)]
    pub enum OutputTypeArg {
        BIN, ASCII
    }

}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum Corner {
        NW, NE, SW, SE
    }
}

impl Corner {
    fn convert(&self) -> maze::Corner {
        match &self {
            Corner::NW => maze::Corner::NorthWest,
            Corner::NE => maze::Corner::NorthEast,
            Corner::SW => maze::Corner::SouthWest,
            Corner::SE => maze::Corner::SouthEast,
        }
    }
}

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum Dir {
        N, E, S, W
    }
}

impl Dir {
    fn convert(&self) -> maze::Direction {
        match &self {
            Dir::N => maze::Direction::North,
            Dir::E => maze::Direction::East,
            Dir::S => maze::Direction::South,
            Dir::W => maze::Direction::West,
        }
    }
}

#[derive(StructOpt, Debug)]
struct MazeParams {
    /// maze algorithm -- choose from the implemented maze generation algorithms -- required
    #[structopt(possible_values = &AlgorithmArg::variants(), case_insensitive = true)]
    alg: AlgorithmArg,
    /// height --  of the maze to be generated, in cells
    #[structopt(short, long, default_value = "10")]
    height: usize,
    /// width --  of the maze to be generated, in cells
    #[structopt(short, long, default_value = "10")]
    width: usize,
    /// output type --
    /// "ascii" produces a nice command line visual,
    /// "bin" produces the maze's underlying byte encoding
    #[structopt(short,long,  possible_values = &OutputTypeArg::variants(), case_insensitive = true , default_value = "ascii" )]
    output_type: OutputTypeArg,

    /// corner -- optionally specify a corner for algorithms that accept this argument, ignored otherwise
    #[structopt(long, possible_values = &Corner::variants(), case_insensitive = true)]
    corner: Option<Corner>,

    /// direction -- optionally specify a direction for algorithms that accept this argument, ignored otherwise
    #[structopt(long, possible_values = &Dir::variants(), case_insensitive = true)]
    direction: Option<Dir>,
}

impl MazeParams {
    fn derive_output_type(&self) -> maze::OutputType {
        match &self.output_type {
            OutputTypeArg::BIN => maze::OutputType::BIN,
            OutputTypeArg::ASCII => maze::OutputType::ASCII,
        }
    }

    fn derive_algorithm(&self) -> maze::Algorithm {
        match &self.alg {
            AlgorithmArg::AB => maze::Algorithm::AlduousBroder,
            AlgorithmArg::SW => maze::Algorithm::SideWinder {
                traversal_direction: self.derive_traversal_direction(),
            },
            AlgorithmArg::BT => maze::Algorithm::BinTree {
                start_corner: self.derive_start_corner(),
            },
        }
    }

    fn derive_dimensions(&self) -> maze::Dimensions {
        return maze::Dimensions {
            height: self.height,
            width: self.width,
        };
    }

    fn derive_start_corner(&self) -> maze::Corner {
        self.corner.as_ref().unwrap_or(&Corner::NW).convert()
    }

    fn derive_traversal_direction(&self) -> maze::Direction {
        self.direction.as_ref().unwrap_or(&Dir::N).convert()
    }
}

fn main() -> Result<(), ExitFailure> {
    let params: MazeParams = MazeParams::from_args();

    let output: maze::Output = maze::generate(maze::Args {
        dimensions: params.derive_dimensions(),
        output_type: params.derive_output_type(),
        algorigthm: params.derive_algorithm(),
    })
    .expect("Maze generation failed");
    match output {
        maze::Output::ASCII(text) => println!("{}", text),
        maze::Output::BIN(bytes) => {
            for byte in bytes {
                print!("{:#08b} ", byte)
            }
        }
    }
    Ok(())
}
