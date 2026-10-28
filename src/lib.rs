//! Host-side mathematical simulator, with no guest ISA or hardware implementation.
//!
//! A row computes `sum(weight[column] * input[column])` in column order.
//! Activations and partial sums use checked host `i64` arithmetic. A prefix
//! overflow is an error even if a later term could cancel it.

use std::fmt;

/// Mathematical ternary weights. This enum has no target storage encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Weight {
    Negative,
    Zero,
    Positive,
}

impl Weight {
    pub const fn value(self) -> i8 {
        match self {
            Self::Negative => -1,
            Self::Zero => 0,
            Self::Positive => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidWeight(pub i8);

impl fmt::Display for InvalidWeight {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "weight {} is outside -1, 0, +1", self.0)
    }
}

impl std::error::Error for InvalidWeight {}

impl TryFrom<i8> for Weight {
    type Error = InvalidWeight;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::Negative),
            0 => Ok(Self::Zero),
            1 => Ok(Self::Positive),
            other => Err(InvalidWeight(other)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    ZeroDimension,
    ShapeOverflow,
    WeightCount { expected: usize, actual: usize },
    InvalidWeight { index: usize, value: i8 },
    InputLength { expected: usize, actual: usize },
    ArithmeticOverflow { row: usize, column: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroDimension => write!(f, "matrix dimensions must be positive"),
            Self::ShapeOverflow => write!(f, "matrix dimensions overflow the host index range"),
            Self::WeightCount { expected, actual } => {
                write!(f, "expected {expected} row-major weights, got {actual}")
            }
            Self::InvalidWeight { index, value } => {
                write!(f, "weight {index} has invalid value {value}")
            }
            Self::InputLength { expected, actual } => {
                write!(f, "expected {expected} activations, got {actual}")
            }
            Self::ArithmeticOverflow { row, column } => {
                write!(f, "partial sum overflow at row {row}, column {column}")
            }
        }
    }
}

impl std::error::Error for Error {}

/// A dense row-major matrix with validated ternary weights.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    weights: Vec<Weight>,
}

impl Matrix {
    /// Validate dimensions, count, then each weight in row-major order.
    ///
    /// Allocations use Rust's host allocator; allocation failure is not modeled
    /// as a recoverable simulator error.
    pub fn new(rows: usize, columns: usize, weights: Vec<i8>) -> Result<Self, Error> {
        if rows == 0 || columns == 0 {
            return Err(Error::ZeroDimension);
        }
        let expected = rows.checked_mul(columns).ok_or(Error::ShapeOverflow)?;
        if weights.len() != expected {
            return Err(Error::WeightCount {
                expected,
                actual: weights.len(),
            });
        }
        let weights = weights
            .into_iter()
            .enumerate()
            .map(|(index, value)| {
                Weight::try_from(value).map_err(|_| Error::InvalidWeight { index, value })
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            rows,
            columns,
            weights,
        })
    }

    /// Evaluate by subtracting, skipping, or adding each input activation.
    ///
    /// No partial output is returned on failure. Inputs and matrix are immutable.
    pub fn infer(&self, input: &[i64]) -> Result<Vec<i64>, Error> {
        self.validate_input(input)?;
        let mut output = Vec::with_capacity(self.rows);
        for (row, weights) in self.weights.chunks_exact(self.columns).enumerate() {
            let mut sum = 0_i64;
            for (column, (&weight, &activation)) in weights.iter().zip(input).enumerate() {
                sum = match weight {
                    Weight::Negative => sum.checked_sub(activation),
                    Weight::Zero => Some(sum),
                    Weight::Positive => sum.checked_add(activation),
                }
                .ok_or(Error::ArithmeticOverflow { row, column })?;
            }
            output.push(sum);
        }
        Ok(output)
    }

    /// Independent multiply-based host reference with the same prefix limits.
    ///
    /// Each product and sum is computed in `i128`, then the prefix is checked
    /// against `i64`. This deliberately does not call the add/sub/skip kernel.
    pub fn reference_infer(&self, input: &[i64]) -> Result<Vec<i64>, Error> {
        self.validate_input(input)?;
        let mut output = Vec::with_capacity(self.rows);
        for row in 0..self.rows {
            let mut sum = 0_i128;
            for (column, &activation) in input.iter().enumerate() {
                let weight = self.weights[row * self.columns + column].value();
                sum += i128::from(weight) * i128::from(activation);
                if i64::try_from(sum).is_err() {
                    return Err(Error::ArithmeticOverflow { row, column });
                }
            }
            output.push(i64::try_from(sum).expect("every prefix was checked"));
        }
        Ok(output)
    }

    fn validate_input(&self, input: &[i64]) -> Result<(), Error> {
        if input.len() != self.columns {
            return Err(Error::InputLength {
                expected: self.columns,
                actual: input.len(),
            });
        }
        Ok(())
    }
}
