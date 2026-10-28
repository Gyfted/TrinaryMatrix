use std::process::ExitCode;
use trinarymatrix::Matrix;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if !args.is_empty() && args != ["--check"] {
        return Err("usage: trinarymatrix [--check]".into());
    }
    let matrix = Matrix::new(2, 3, vec![1, 0, -1, -1, 1, 1])?;
    let input = [4, -2, 3];
    let output = matrix.infer(&input)?;
    let reference = matrix.reference_infer(&input)?;
    if output != reference || output != [1, -3] {
        return Err("synthetic fixture disagrees with its reference or expected output".into());
    }
    println!("input: {input:?}");
    println!("add/sub/skip output: {output:?}");
    println!("multiply reference: {reference:?}");
    println!("check: passed (host simulation; no physical performance claim)");
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
