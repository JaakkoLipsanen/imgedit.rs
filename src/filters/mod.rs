pub mod parse;

/// Enum containing all the filters that the application supports
#[derive(Debug, PartialEq)]
pub enum Filter {
    Identity,
    InvertColor,
    Brighten { amount: f32 }
}