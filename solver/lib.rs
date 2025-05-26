mod solver;

use solver::INFINITY as INTERNAL_INFINITY;

pub type Graph = Vec<Vec<i32>>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TspError {
    EmptyGraph,
    NonSquareGraph { rows: usize, cols: usize },
    StartNodeOutOfBounds { num_nodes: usize, start_node: usize },
    NoPathFound,
    InternalSolverError(String),
}

impl std::fmt::Display for TspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TspError::EmptyGraph => write!(f, "Graph tidak boleh kosong."),
            TspError::NonSquareGraph { rows, cols } => {
                write!(
                    f,
                    "Graph harus berupa matriks persegi. Diberikan: {} baris, {} kolom.",
                    rows, cols
                )
            }
            TspError::StartNodeOutOfBounds { num_nodes, start_node } => {
                write!(
                    f,
                    "Node awal {} berada di luar batas. Jumlah node: {}.",
                    start_node, num_nodes
                )
            }
            TspError::NoPathFound => write!(f, "Tidak ada jalur TSP yang valid ditemukan."),
            TspError::InternalSolverError(msg) => write!(f, "Error internal solver: {}", msg),
        }
    }
}

impl std::error::Error for TspError {}

pub fn solve_tsp_dynamic_programming(
    graph: Graph,
    start_node: usize,
) -> Result<(i32, Vec<usize>), TspError> {
    let n = graph.len();

    if n == 0 {
        return Err(TspError::EmptyGraph);
    }

    for row in graph.iter() {
        if row.len() != n {
            return Err(TspError::NonSquareGraph { rows: n, cols: row.len() });
        }
    }

    if start_node >= n {
        return Err(TspError::StartNodeOutOfBounds { num_nodes: n, start_node });
    }

    let mut tsp_solver = solver::TspSolver::new(graph, start_node);

    match tsp_solver.solve() {
        (cost, _path) if cost == INTERNAL_INFINITY => Err(TspError::NoPathFound),
        (_cost, ref path_ref) if path_ref.is_empty() && n > 1 => Err(TspError::NoPathFound), 
        (_cost, path) if n > 1 && (path.len() != n + 1 || path.first() != Some(&start_node) || path.last() != Some(&start_node)) => {
             Err(TspError::NoPathFound)
        }
        (cost, path) => Ok((cost, path)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_graph_ok() {
        let graph = vec![
            vec![0, 10, 15, 20],
            vec![10, 0, 35, 25],
            vec![15, 35, 0, 30],
            vec![20, 25, 30, 0],
        ];
        let result = solve_tsp_dynamic_programming(graph.clone(), 0);
        assert!(result.is_ok());
        let (cost, path) = result.unwrap();
        assert_eq!(cost, 80);
        let valid_paths = vec![vec![0, 1, 3, 2, 0], vec![0, 2, 3, 1, 0]];
        assert!(valid_paths.contains(&path), "Jalur {:?} tidak ada di antara jalur optimal yang valid", path);

        let result_start1 = solve_tsp_dynamic_programming(graph, 1);
        assert!(result_start1.is_ok());
        let (cost_s1, path_s1) = result_start1.unwrap();
        assert_eq!(cost_s1, 80);
        let valid_paths_s1 = vec![vec![1, 0, 2, 3, 1], vec![1, 3, 2, 0, 1]];
        assert!(valid_paths_s1.contains(&path_s1), "Jalur {:?} tidak ada di antara jalur optimal yang valid untuk start 1", path_s1);
    }

    #[test]
    fn test_another_graph_ok() {
        let inf = i32::MAX / 2;
        let graph = vec![
            vec![0, 1, 5, inf],
            vec![1, 0, 2, 4],
            vec![5, 2, 0, 3],
            vec![inf, 4, 3, 0],
        ];
        let (cost, path) = solve_tsp_dynamic_programming(graph, 0).unwrap();
        assert_eq!(cost, 13);
        assert!(vec![vec![0, 1, 3, 2, 0], vec![0, 2, 3, 1, 0]].contains(&path), "Jalur {:?} tidak ada di antara jalur optimal yang valid", path);
    }

    #[test]
    fn test_single_node_ok() {
        let graph = vec![vec![0]];
        let (cost, path) = solve_tsp_dynamic_programming(graph, 0).unwrap();
        assert_eq!(cost, 0);
        assert_eq!(path, vec![0, 0]);
    }

    #[test]
    fn test_no_path_found_error() {
        let inf = i32::MAX / 2;
        let graph = vec![vec![0, inf], vec![inf, 0]];
        let result = solve_tsp_dynamic_programming(graph, 0);
        assert_eq!(result, Err(TspError::NoPathFound));
    }

    #[test]
    fn test_empty_graph_error() {
        let graph: Graph = Vec::new();
        let result = solve_tsp_dynamic_programming(graph, 0);
        assert_eq!(result, Err(TspError::EmptyGraph));
    }

    #[test]
    fn test_non_square_graph_error() {
        let graph = vec![vec![0, 1], vec![2]];
        let result = solve_tsp_dynamic_programming(graph, 0);
        assert_eq!(result, Err(TspError::NonSquareGraph { rows: 2, cols: 1 }));
    }

    #[test]
    fn test_start_node_out_of_bounds_error() {
        let graph = vec![vec![0]];
        let result = solve_tsp_dynamic_programming(graph, 1);
        assert_eq!(result, Err(TspError::StartNodeOutOfBounds { num_nodes: 1, start_node: 1 }));

        let graph2 = vec![vec![0, 1], vec![1, 0]];
        let result2 = solve_tsp_dynamic_programming(graph2, 2);
        assert_eq!(result2, Err(TspError::StartNodeOutOfBounds { num_nodes: 2, start_node: 2 }));
    }

     #[test]
    fn test_three_nodes_ok() { 
         let graph = vec![
            vec![0, 10, 20],
            vec![10, 0, 30],
            vec![20, 30, 0],
        ];
        let (cost, path) = solve_tsp_dynamic_programming(graph, 0).unwrap();
        assert_eq!(cost, 60);
        
        let valid_paths = vec![vec![0, 1, 2, 0], vec![0, 2, 1, 0]];
         assert!(valid_paths.contains(&path), "Jalur {:?} tidak ada di antara jalur optimal yang valid untuk graf 3 node simetris", path);
    }

    #[test]
    fn test_three_nodes_directed_ok() { 
        let graph = vec![
           
            vec![0,  10, 40], 
            vec![12, 0,  15], 
            vec![25, 18, 0 ], 
        ];
        
        let (cost, path) = solve_tsp_dynamic_programming(graph, 0).unwrap();
        assert_eq!(cost, 50);
        assert_eq!(path, vec![0, 1, 2, 0]);
    }


    #[test]
    fn test_two_nodes_ok() { 
        let graph = vec![
            vec![0, 10], 
            vec![12, 0], 
        ];
        
        let (cost, path) = solve_tsp_dynamic_programming(graph, 0).unwrap();
        assert_eq!(cost, 22);
        assert_eq!(path, vec![0, 1, 0]);
    }

    #[test]
    fn test_no_return_path_error() {
        let inf = i32::MAX / 2;
         let graph = vec![
            vec![0, 1, inf], 
            vec![1, 0, 1],   
            vec![inf, 1, 0], 
        ];
        
        let result = solve_tsp_dynamic_programming(graph, 0);
        assert_eq!(result, Err(TspError::NoPathFound));
    }
}