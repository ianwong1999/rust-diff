use super::diff_result::*;
use std::cmp;

/* Myers Difference is an O(ND) (D = number of differences) difference algorithm
 * based on the assumption that, D is small in many cases. Basically, when D is
 * small, the optimal path on an edit graph would be very close to the main diagonal
 * (0, 0), (1, 1) .... (N, M). So we try to iterate the answer D from 0 to see if
 * we can reach to (N, M) using at most D vertical/horizontal edge. Notice that for
 * every D, on each diagonal, we want to reach to the furthest from origin. And such
 * furthest coordinate is can also be computed based on the furthest of D-1. So we can
 * greedily compute furthest reaching point of each diagonal on each increment of D.
 */

pub fn diff(lhs: &Vec<String>, rhs: &Vec<String>) -> DiffResult {
    let n = lhs.len();
    let m = rhs.len();

    let max_edit = n + m;

    let mut furthest_x: Vec<Vec<usize>> = vec![];

    let get_furthest_x = |furthest_x: &Vec<Vec<usize>>, d: usize, k: isize| -> usize {
        let offset = d as isize;
        furthest_x[d][(k + offset) as usize]
    };

    let update_furthest_x = |furthest_x: &mut Vec<Vec<usize>>, d: usize, k: isize, val: usize| {
        let offset = d as isize;
        furthest_x[d][(k + offset) as usize] = val;
    };

    let mut min_diff = max_edit;

    // for coordinate (x, y) they are on diagonal k=x-y
    // -m <= k <= n
    'main: for d in 0..=max_edit {
        let left_bound = -(cmp::min(m, d) as isize);
        let right_bound = cmp::min(n, d) as isize;

        furthest_x.push(vec![0; d * 2 + 1]);

        for k in left_bound..=right_bound {
            if k.abs() % 2 != (d % 2) as isize {
                continue;
            }

            let left_diagonal_x = if k == left_bound {
                0
            } else {
                get_furthest_x(&furthest_x, d - 1, k - 1) + 1
            };

            let right_diagonal_x = if k == right_bound {
                0
            } else {
                get_furthest_x(&furthest_x, d - 1, k + 1)
            };

            let mut x = cmp::max(left_diagonal_x, right_diagonal_x);
            let mut y = (x as isize - k) as usize;

            while x < n && y < m && lhs[x] == rhs[y] {
                x += 1;
                y += 1;
            }

            update_furthest_x(&mut furthest_x, d, k, x);

            if x >= n && y >= m {
                min_diff = d;
                break 'main;
            }
        }
    }

    let mut trace: Vec<(usize, usize)> = vec![];

    // backtrack
    {
        let mut cur_x = n;
        let mut cur_y = m;

        for d in (0..=min_diff).rev() {
            let left_bound = -(cmp::min(m, d) as isize);
            let right_bound = cmp::min(n, d) as isize;

            let k = cur_x as isize - cur_y as isize;

            let prev_left_diagonal_x = if k == left_bound {
                0
            } else {
                get_furthest_x(&furthest_x, d - 1, k - 1) + 1
            };

            let prev_right_diagonal_x = if k == right_bound {
                0
            } else {
                get_furthest_x(&furthest_x, d - 1, k + 1)
            };

            let prev_x = cmp::max(prev_left_diagonal_x, prev_right_diagonal_x);

            while cur_x > prev_x {
                cur_x -= 1;
                cur_y -= 1;

                trace.push((cur_x, cur_y));
            }

            if d == 0 {
                break;
            }

            if k != left_bound && prev_x == prev_left_diagonal_x {
                cur_x -= 1;
            } else {
                cur_y -= 1;
            }
        }
    }

    trace.reverse();

    DiffResult::new(trace)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_trace() {
        let lhs = vec!["a", "b", "c"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let rhs = vec!["d", "b", "e"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();

        let res = diff(&lhs, &rhs);

        assert_eq!(vec![(1, 1)], res.trace);
    }

    #[test]
    fn both_input_empty() {
        let lhs = vec![];
        let rhs = vec![];

        let res = diff(&lhs, &rhs);

        assert_eq!(true, res.trace.len() == 0);
    }

    #[test]
    fn lhs_input_empty() {
        let lhs = vec![];
        let rhs = vec!["a"].iter().map(|s| (*s).to_owned()).collect();

        let res = diff(&lhs, &rhs);

        assert_eq!(true, res.trace.len() == 0);
    }

    #[test]
    fn rhs_input_empty() {
        let lhs = vec!["a"].iter().map(|s| (*s).to_owned()).collect();
        let rhs = vec![];

        let res = diff(&lhs, &rhs);

        assert_eq!(true, res.trace.len() == 0);
    }

    #[test]
    fn same_input() {
        let lhs = vec!["aa", "bb", "cc"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();
        let rhs = vec!["aa", "bb", "cc"]
            .iter()
            .map(|s| (*s).to_owned())
            .collect();

        let res = diff(&lhs, &rhs);

        assert_eq!(vec![(0, 0), (1, 1), (2, 2)], res.trace);
    }
}
