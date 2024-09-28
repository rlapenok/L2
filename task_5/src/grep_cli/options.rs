pub enum GrepOption {
    After(usize),
    Before(usize),
    Context(usize),
    Count,
    LineNum,
    Current,
}

#[derive(Debug)]
pub enum FilterOptions {
    Ignore,
    Invert,
    Fixed,
    NotFixed,
}
