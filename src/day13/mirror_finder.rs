use itertools::Itertools;

pub fn find_almost_mirror_line(matrix: &Vec<Vec<char>>, imperfections: usize) -> Option<usize> {
    matrix
        .iter()
        .enumerate()
        .skip(1)
        .find(|&(line_index, _)| {
            verify_almost_mirror_line(matrix, line_index, imperfections).unwrap_or(false)
        })
        .map(|(line_index, _)| line_index)
}

pub fn verify_almost_mirror_line(
    matrix: &Vec<Vec<char>>,
    line: usize,
    imperfection: usize,
) -> Option<bool> {
    let mirrored_lines = usize::min(line, matrix.len() - line);

    let above_lines = matrix
        .get(line - mirrored_lines..line)?
        .iter()
        .collect_vec();
    let below_lines = matrix
        .get(line..line + mirrored_lines)?
        .iter()
        .rev()
        .collect_vec();
    let mut diff_counter = 0;
    for i in 0..mirrored_lines {
        let above_line = above_lines[i];
        let below_line = below_lines[i];
        for j in 0..above_line.len() {
            if above_line[j] != below_line[j] {
                diff_counter += 1;
            }
        }
    }

    Some(diff_counter == imperfection)
}
