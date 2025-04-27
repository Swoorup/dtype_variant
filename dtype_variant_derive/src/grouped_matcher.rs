use darling::{Error, FromAttributes, FromMeta};
use proc_macro2::Span;
use syn::{Ident, Meta};

// Final structure to hold parsed data
#[derive(Debug, Clone)]
pub(crate) struct ParsedGroupedMatcher {
    pub macro_name: Ident,
    pub groups: Vec<(Ident, Vec<Ident>)>,
    pub _span: Span,
}

// Simplified wrapper for the groups
#[derive(Debug)]
pub(crate) struct ParsedGroups(pub Vec<(Ident, Vec<Ident>)>);

impl FromMeta for ParsedGroups {
    fn from_meta(item: &Meta) -> darling::Result<Self> {
        // Only handle name-value pairs with the name "grouping"
        let meta_list = match item {
            Meta::NameValue(nv) if nv.path.is_ident("grouping") => {
                match &nv.value {
                    syn::Expr::Array(array) => array,
                    _ => return Err(Error::custom(
                        "`grouping` value must be a list in brackets `[...]`",
                    )
                    .with_span(&nv.value)),
                }
            }
            Meta::Path(path) if path.is_ident("grouping") => {
                return Err(Error::custom(
                    "`grouping` requires a value: `grouping = [...]`",
                )
                .with_span(path));
            }
            _ => {
                return Err(Error::unexpected_type(
                    "Expected `grouping = [...]`",
                )
                .with_span(item));
            }
        };

        let mut groups = Vec::new();

        // Parse each group definition from the array
        for elem in &meta_list.elems {
            match elem {
                syn::Expr::Call(call) => {
                    // Extract group name
                    let group_name = match &*call.func {
                        syn::Expr::Path(path) => path.path.get_ident().cloned().ok_or_else(||
                            Error::custom("Expected group name identifier").with_span(&*call.func)
                        )?,
                        _ => return Err(Error::custom("Expected group name identifier").with_span(&*call.func))
                    };

                    // Ensure there's exactly one argument (the variant array)
                    if call.args.len() != 1 {
                        return Err(Error::custom(
                            format!("Group `{}` expects exactly one argument (a list of variants in brackets `[...]`)", group_name)
                        ).with_span(&call.args));
                    }

                    // Extract the variants array
                    let variants_array = match &call.args[0] {
                        syn::Expr::Array(array) => array,
                        _ => return Err(Error::custom(
                            "Expected variant list in brackets `[...]`"
                        ).with_span(&call.args))
                    };

                    // Extract each variant identifier
                    let mut variants = Vec::new();
                    for variant_expr in &variants_array.elems {
                        let variant = match variant_expr {
                            syn::Expr::Path(path) => path.path.get_ident().cloned().ok_or_else(||
                                Error::custom("Expected variant identifier").with_span(variant_expr)
                            )?,
                            _ => return Err(Error::unexpected_expr_type(variant_expr).with_span(variant_expr))
                        };
                        variants.push(variant);
                    }

                    // Ensure the variant list is not empty
                    if variants.is_empty() {
                        return Err(Error::custom(
                            "Group variant list cannot be empty"
                        ).with_span(variants_array));
                    }

                    groups.push((group_name, variants));
                },
                _ => return Err(Error::custom(
                    "Expected group definition in the format `GroupName([Variant, ...])`"
                ).with_span(elem))
            }
        }

        // Ensure at least one group was defined
        if groups.is_empty() {
            return Err(Error::custom(
                "`grouping` list must define at least one group",
            )
            .with_span(meta_list));
        }

        Ok(ParsedGroups(groups))
    }
}

// Struct for parsing the attribute arguments
#[derive(Debug, FromAttributes)]
#[darling(attributes(dtype_grouped_matcher))]
pub(crate) struct DTypeGroupedMatcherArgs {
    #[darling(rename = "name")]
    pub macro_name: Ident,
    pub grouping: ParsedGroups,
}
