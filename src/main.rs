use clap::Parser;
use multicover::facade;
use std::path::PathBuf;

fn main() {
    let opt = Opt::parse();
    let e = facade::Executor::new(
        opt.files,
        opt.use_decimal_index,
        opt.depth_begin,
        opt.depth_end,
        opt.separator,
        opt.use_empty,
        opt.use_sort,
    );
    e.execute();
}

/// Output the set to which each element belongs.
///
/// See: https://github.com/berquerant/multicover
#[derive(Parser, Debug)]
#[command(version, about)]
struct Opt {
    /// Header as decimal.
    #[arg(short = 'i', long = "use_decimal_index")]
    use_decimal_index: bool,

    // Separator when read values from stdin.
    #[arg(short = 's', long = "separator", default_value = ",")]
    separator: char,

    /// Input files.
    files: Vec<PathBuf>,

    /// Min size of pairs of the combinations.
    #[arg(short = 'b', long = "depth_begin")]
    depth_begin: Option<usize>,

    /// Max size of pairs of the combinations.
    #[arg(short = 'd', long = "depth_end")]
    depth_end: Option<usize>,

    /// Accept empty string if true.
    #[arg(short = 'e', long = "use_empty")]
    use_empty: bool,

    /// Sort elements if true.
    #[arg(long = "sort")]
    use_sort: bool,
}
