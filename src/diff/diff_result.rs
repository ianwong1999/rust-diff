pub struct DiffResult {
    pub trace: Vec<(usize, usize)>,
}

impl DiffResult {
    pub fn new(trace: Vec<(usize, usize)>) -> DiffResult {
        DiffResult { trace }
    }
}
