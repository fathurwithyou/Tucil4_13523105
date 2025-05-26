use std::io::{self, BufRead}; 
use solver::{solve_tsp_dynamic_programming, Graph, TspError};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("Input");
    let n: usize = match lines.next() {
        Some(Ok(line)) => match line.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(_) => {
                eprintln!("Error: Jumlah kota (N) harus lebih besar dari 0.");
                return;
            }
            Err(_) => {
                eprintln!("Error: Input N tidak valid (harus angka).");
                return;
            }
        },
        _ => {
            eprintln!("Error: Gagal membaca input N.");
            return;
        }
    };

    if n == 1 {
        println!("[0, 0]"); 
        return;
    }

    
    let mut graph: Graph = Vec::with_capacity(n);
    for i in 0..n {
        match lines.next() {
            Some(Ok(line)) => {
                let row_values: Result<Vec<i32>, _> = line
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect();

                match row_values {
                    Ok(row) => {
                        if row.len() == n {
                            graph.push(row);
                        } else {
                            eprintln!(
                                "Error: Baris {} matriks tidak memiliki {} angka (ditemukan {}).",
                                i + 1,
                                n,
                                row.len()
                            );
                            return;
                        }
                    }
                    Err(_) => {
                        eprintln!(
                            "Error: Baris {} matriks mengandung nilai non-numerik.",
                            i + 1
                        );
                        return;
                    }
                }
            }
            _ => {
                eprintln!("Error: Gagal membaca baris {} matriks.", i + 1);
                return;
            }
        }
    }

    
    let start_node: usize = match lines.next() {
        Some(Ok(line)) => match line.trim().parse() {
            Ok(num) if num < n => num,
            Ok(_) => {
                 eprintln!(
                    "Error: Node awal di luar batas (harus antara 0 dan {}).",
                    n - 1
                );
                return;
            }
            Err(_) => {
                eprintln!("Error: Input node awal tidak valid (harus angka).");
                return;
            }
        },
        _ => {
            eprintln!("Error: Gagal membaca input node awal.");
            return;
        }
    };

    println!("Output");
    match solve_tsp_dynamic_programming(graph, start_node) {
        Ok((_cost, path)) => { 
            println!("Total cost: {}", _cost);
            if path.is_empty() {
                eprintln!("Error: Jalur yang dikembalikan kosong.");
            } else {
                for (i, node) in path.iter().enumerate() {
                    if i == path.len() - 1 {
                        print!("{}", node); 
                    } else {
                        print!("{} -> ", node);
                    }
                }
                println!(); 
            }
        }
        Err(e) => {
            match e {
                TspError::NoPathFound => eprintln!("Error: Tidak ada jalur TSP yang valid ditemukan."),
                TspError::EmptyGraph => eprintln!("Error: Graf tidak boleh kosong (seharusnya sudah ditangani)."),
                TspError::NonSquareGraph { .. } => {
                    eprintln!("Error: Graf tidak persegi (seharusnya sudah ditangani).");
                }
                TspError::StartNodeOutOfBounds { .. } => {
                    eprintln!("Error: Node awal di luar batas (seharusnya sudah ditangani).");
                }
                TspError::InternalSolverError(msg) => {
                    eprintln!("Error internal solver: {}", msg);
                }
            }
        }
    }
}
