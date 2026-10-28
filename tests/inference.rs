use trinarymatrix::{Error, InvalidWeight, Matrix, Weight};

#[test]
fn all_weight_values_are_checked() {
    for value in i8::MIN..=i8::MAX {
        match Weight::try_from(value) {
            Ok(weight) => {
                assert!((-1..=1).contains(&value));
                assert_eq!(weight.value(), value);
            }
            Err(error) => {
                assert!(!(-1..=1).contains(&value));
                assert_eq!(error, InvalidWeight(value));
            }
        }
    }
}

#[test]
fn constructor_rejects_invalid_shapes_and_values() {
    assert_eq!(Matrix::new(0, 3, vec![]), Err(Error::ZeroDimension));
    assert_eq!(Matrix::new(2, 0, vec![]), Err(Error::ZeroDimension));
    assert_eq!(
        Matrix::new(usize::MAX, 2, vec![]),
        Err(Error::ShapeOverflow)
    );
    assert_eq!(
        Matrix::new(2, 2, vec![0; 3]),
        Err(Error::WeightCount {
            expected: 4,
            actual: 3
        })
    );
    assert_eq!(
        Matrix::new(1, 2, vec![1, 2]),
        Err(Error::InvalidWeight { index: 1, value: 2 })
    );
}

#[test]
fn shape_mismatch_is_reported_before_execution() {
    let matrix = Matrix::new(1, 2, vec![1, -1]).unwrap();
    for input in [&[][..], &[1][..], &[1, 2, 3][..]] {
        let expected = Err(Error::InputLength {
            expected: 2,
            actual: input.len(),
        });
        assert_eq!(matrix.infer(input), expected);
        assert_eq!(matrix.reference_infer(input), expected);
    }
}

#[test]
fn synthetic_fixture_and_zero_weight() {
    let matrix = Matrix::new(2, 3, vec![1, 0, -1, -1, 1, 1]).unwrap();
    assert_eq!(matrix.infer(&[4, -2, 3]), Ok(vec![1, -3]));
    assert_eq!(matrix.reference_infer(&[4, -2, 3]), Ok(vec![1, -3]));
    let zeros = Matrix::new(1, 2, vec![0, 0]).unwrap();
    assert_eq!(zeros.infer(&[i64::MIN, i64::MAX]), Ok(vec![0]));
}

#[test]
fn exhaustive_small_matrices_match_independent_reference() {
    // Every 2x3 ternary matrix and every 3-vector over [-2, 2].
    for mut code in 0..3_usize.pow(6) {
        let mut weights = Vec::new();
        for _ in 0..6 {
            weights.push((code % 3) as i8 - 1);
            code /= 3;
        }
        let matrix = Matrix::new(2, 3, weights).unwrap();
        for a in -2..=2 {
            for b in -2..=2 {
                for c in -2..=2 {
                    let input = [a, b, c];
                    assert_eq!(matrix.infer(&input), matrix.reference_infer(&input));
                }
            }
        }
    }
}

#[test]
fn boundary_results_and_prefix_overflow_agree() {
    let inputs = [i64::MIN, i64::MIN + 1, -1, 0, 1, i64::MAX - 1, i64::MAX];
    for left in -1..=1 {
        for right in -1..=1 {
            let matrix = Matrix::new(1, 2, vec![left, right]).unwrap();
            for &a in &inputs {
                for &b in &inputs {
                    assert_eq!(matrix.infer(&[a, b]), matrix.reference_infer(&[a, b]));
                }
            }
        }
    }
    let negation = Matrix::new(1, 1, vec![-1]).unwrap();
    assert_eq!(
        negation.infer(&[i64::MIN]),
        Err(Error::ArithmeticOverflow { row: 0, column: 0 })
    );
    let cancellation = Matrix::new(1, 3, vec![1, 1, -1]).unwrap();
    assert_eq!(
        cancellation.infer(&[i64::MAX, 1, 1]),
        Err(Error::ArithmeticOverflow { row: 0, column: 1 })
    );
}

#[test]
fn failure_does_not_mutate_model_or_input() {
    let matrix = Matrix::new(2, 2, vec![0, 0, 1, 1]).unwrap();
    let saved = matrix.clone();
    let input = [i64::MAX, 1];
    assert_eq!(
        matrix.infer(&input),
        Err(Error::ArithmeticOverflow { row: 1, column: 1 })
    );
    assert_eq!(matrix, saved);
    assert_eq!(input, [i64::MAX, 1]);
    assert_eq!(matrix.infer(&[2, 3]), Ok(vec![0, 5]));
}
