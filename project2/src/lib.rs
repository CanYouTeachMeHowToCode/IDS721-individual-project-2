/* A library for doing N-Queens Problem */

/* build a function that returns all the valid chess board of N-Queens Problem
where Queens being represented by 'Q' and empty tiles being represented by '.' and
the parameter passing in is 32-bit integer */

// reference: https://leetcode.com/problems/n-queens/solutions/1224550/rust-backtrack/?q=rust&orderBy=most_relevant

const QUEEN: char = 'Q';
const EMPTY: char = '.';
pub fn n_queens(input: i32) -> String {
    let n: usize = input as usize;
    let mut is_same_col: Vec<bool> = vec![false; n];
    let mut is_same_main_diagonal: Vec<bool> = vec![false; 2 * n - 1];
    let mut is_same_anti_diagonal: Vec<bool> = vec![false; 2 * n - 1];
    let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    let mut ans: Vec<Vec<String>> = Vec::new();

    backtrack(
        0,
        &mut is_same_col,
        &mut is_same_main_diagonal,
        &mut is_same_anti_diagonal,
        &mut board,
        n,
        &mut ans,
    );
    convert(&ans)
}

fn convert(vec: &[Vec<String>]) -> String {
    vec.iter()
        .map(|inner_vec| inner_vec.join(", "))
        .collect::<Vec<String>>()
        .join("\n")
}

fn backtrack(
    row: usize,
    is_same_col: &mut Vec<bool>,
    is_same_main_diagonal: &mut Vec<bool>,
    is_same_anti_diagonal: &mut Vec<bool>,
    board: &mut Vec<Vec<char>>,
    n: usize,
    res: &mut Vec<Vec<String>>,
) {
    if row == n {
        res.push(
            board
                .iter()
                .map(|cur_row| cur_row.iter().collect())
                .collect(),
        );
        return;
    }

    for col in 0..n {
        if is_same_col[col]
            || is_same_main_diagonal[row + n - col - 1]
            || is_same_anti_diagonal[row + col]
        {
            continue;
        }

        is_same_col[col] = true;
        is_same_main_diagonal[row + n - col - 1] = true;
        is_same_anti_diagonal[row + col] = true;
        board[row][col] = QUEEN;

        backtrack(
            row + 1,
            is_same_col,
            is_same_main_diagonal,
            is_same_anti_diagonal,
            board,
            n,
            res,
        );

        is_same_col[col] = false;
        is_same_main_diagonal[row + n - col - 1] = false;
        is_same_anti_diagonal[row + col] = false;
        board[row][col] = EMPTY;
    }
}
