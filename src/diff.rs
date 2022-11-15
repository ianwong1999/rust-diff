mod diff_result;
mod myers_diff;

pub fn diff(lhs: &Vec<String>, rhs: &Vec<String>) -> diff_result::DiffResult {
    myers_diff::diff(lhs, rhs)
}
