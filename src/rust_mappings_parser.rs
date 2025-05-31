use std::fs;
use syn::{Expr, ExprArray, ExprReference, ExprTuple, File, Ident, Item, Lit, LitStr};

#[derive(Debug)]
pub enum ParseError {
    FileReadError(String),
    SynParseError(String),
    MappingsNotFound,
    InvalidMappingsFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::FileReadError(s) => write!(f, "File read error: {}", s),
            ParseError::SynParseError(s) => write!(f, "Rust code parse error: {}", s),
            ParseError::MappingsNotFound => write!(f, "'MAPPINGS' constant not found"),
            ParseError::InvalidMappingsFormat(s) => write!(f, "Invalid 'MAPPINGS' format: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}

pub fn parse_mappings_from_rust_file(file_path: &str) -> Result<Vec<(String, String)>, ParseError> {
    let content = fs::read_to_string(file_path).map_err(|e| {
        ParseError::FileReadError(format!("Failed to read file {}: {}", file_path, e))
    })?;

    let ast: File = syn::parse_file(&content).map_err(|e| {
        ParseError::SynParseError(format!("Failed to parse Rust file {}: {}", file_path, e))
    })?;

    for item in ast.items {
        if let Item::Const(item_const) = item {
            if item_const.ident == Ident::new("MAPPINGS", item_const.ident.span()) {
                if let Expr::Reference(ExprReference { expr: ref_expr, .. }) = *item_const.expr {
                    if let Expr::Array(ExprArray { elems, .. }) = &*ref_expr {
                        let mut parse_mappings = Vec::new();
                        for elem_expl in elems {
                            if let Expr::Tuple(ExprTuple {
                                elems: tuple_elems, ..
                            }) = elem_expl
                            {
                                if tuple_elems.len() == 2 {
                                    let first = tuple_elems.first().unwrap();
                                    let second = tuple_elems.last().unwrap();

                                    let s1 = match first {
                                        Expr::Lit(expr_lit) => match &expr_lit.lit {
                                            Lit::Str(lit_str) => Ok(lit_str.value()),
                                            _ => Err(ParseError::InvalidMappingsFormat(
                                                "Tuple element not a string literal".into(),
                                            )),
                                        },
                                        _ => Err(ParseError::InvalidMappingsFormat(
                                            "Tuple element not a literal".into(),
                                        )),
                                    }?;
                                    let s2 = match second {
                                        Expr::Lit(expr_lit) => match &expr_lit.lit {
                                            Lit::Str(lit_str) => Ok(lit_str.value()),
                                            _ => Err(ParseError::InvalidMappingsFormat(
                                                "Tuple element not a string literal".into(),
                                            )),
                                        },
                                        _ => Err(ParseError::InvalidMappingsFormat(
                                            "Tuple element not a literal".into(),
                                        )),
                                    }?;
                                    parse_mappings.push((s1, s2));
                                } else {
                                    return Err(ParseError::InvalidMappingsFormat(
                                        "Tuple does not have 2 elements".into(),
                                    ));
                                }
                            } else {
                                return Err(ParseError::InvalidMappingsFormat(
                                    "Array element in not a tuple".into(),
                                ));
                            }
                        }
                        return Ok(parse_mappings);
                    }
                }
                return Err(ParseError::InvalidMappingsFormat(
                    "MAPPINGS contant expression in not an array reference `&[...]`".into(),
                ));
            }
        }
    }
    Err(ParseError::MappingsNotFound)
}
