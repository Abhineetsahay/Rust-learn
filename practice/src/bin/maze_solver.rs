fn dfs(
    maze: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    path: &mut Vec<(usize, usize)>,
    x: isize,
    y: isize,
    end: (usize, usize),
) -> bool {
    let rows = maze.len() as isize;
    let cols = maze[0].len() as isize;

    if x < 0 || y < 0 || x >= rows || y >= cols {
        return false;
    }
    let xi = x as usize;
    let yi = y as usize;

    if maze[xi][yi] == '#' || visited[xi][yi] {
        return false;
    }

    visited[xi][yi] = true;

    path.push((xi, yi));

    if (xi, yi) == end {
        return true;
    }

    let dir_rows = [-1, 0, 1, 0];
    let dir_cols = [0, 1, 0, -1];

    for i in 0..4 {
        if dfs(maze, visited, path, x + dir_rows[i], y + dir_cols[i], end) {
            return true;
        }
    }
    path.pop();
    return false;
}

fn main() {
    let maze = vec![
        vec!['S', '.', '.', '#'],
        vec!['#', '.', '#', '.'],
        vec!['.', '.', 'E', '.'],
    ];

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            if maze[i][j] == 'S' {
                start = (i, j);
            } else if maze[i][j] == 'E' {
                end = (i, j);
            }
        }
    }
    let mut visited = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = Vec::new();

    if dfs(
        &maze,
        &mut visited,
        &mut path,
        start.0 as isize,
        start.1 as isize,
        end,
    ) {
        println!("Path found:");
        for (x, y) in path {
            println!("({}, {})", x, y);
        }
    } else {
        println!("No path found");
    }
}
