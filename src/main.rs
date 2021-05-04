use multicover::facade;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
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

#[derive(Debug, StructOpt)]
/// Output the set to which each element belongs.
///
/// See: https://github.com/berquerant/multicover
#[structopt(name = "multicover")]
struct Opt {
    /// Header as decimal.
    #[structopt(short = "i", long = "use_decimal_index")]
    use_decimal_index: bool,

    // Separator when read values from stdin.
    #[structopt(short = "s", long = "separator", default_value = ",")]
    separator: char,

    /// Input files.
    files: Vec<PathBuf>,

    /// Min size of pairs of the combinations.
    #[structopt(short = "b", long = "depth_begin")]
    depth_begin: Option<usize>,

    /// Max size of pairs of the combinations.
    #[structopt(short = "d", long = "depth_end")]
    depth_end: Option<usize>,

    /// Accept empty string if true.
    #[structopt(short = "e", long = "use_empty")]
    use_empty: bool,

    /// Sort elements if true.
    #[structopt(long = "sort")]
    use_sort: bool,
}
