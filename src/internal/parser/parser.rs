enum SlrAction {
    Shift(usize),
    Reduce(usize),
    Go(usize),
    Accept,
}
