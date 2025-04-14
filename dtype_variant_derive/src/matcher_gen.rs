use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Ident;

use crate::derive::ParsedVariantInfo;

pub struct MatchArmParam {
    pub enum_name: Ident, // Needed for context if type paths are relative? Maybe not.
    // --- Flags ---
    pub all_unit_variants: bool, // Optimization for simpler type declarations
    pub include_src_ty: bool,    // Should $src_type be defined?
    pub include_inner: bool,     // Should $src_type be defined?
    pub src_type_generic: bool,  // Is $src_type generic?
    pub include_dest: bool,      // Should $dest_type be defined?
    pub dest_type_generic: bool, // Is $dest_type generic?
    pub dest_constraint: bool,   // Should $dest_constraint be defined?
    pub dest_constraint_generic: bool, // Is $dest_constraint generic?
    // --- Identifiers used in the macro pattern ---
    pub inner_ident: TokenStream2, // The ident captured for the inner value (e.g., `inner`, `payload`)
    pub token_type_ident: TokenStream2, // The ident captured for the token type (e.g., `Token`, `TType`)
    pub src_type_ident: TokenStream2, // The ident captured for the src type (e.g., `Src`)
    pub src_type_generic_ident: TokenStream2, // The ident captured for src type generic (e.g., `G`)
    pub dest_enum_ident: TokenStream2, // The ident captured for the dest enum (e.g., `DestEnum`)
    pub dest_type_ident: TokenStream2, // The ident captured for the dest type (e.g., `Dest`)
    pub dest_type_generic_ident: TokenStream2, // The ident captured for dest type generic (e.g., `DG`)
    pub dest_constraint_ident: TokenStream2, // The ident captured for dest constraint (e.g., `Constraint`)
    pub dest_constraint_generic_ident: TokenStream2, // The ident captured for dest constraint generic (e.g., `CG`)

    // --- Path Generation ---
    pub token_path: TokenStream2, // Closure to get `crate::tokens`
    pub dtype_variant_path: TokenStream2, // Closure to get `crate::dtype_variant_path`
    // --- Final User Code ---
    pub user_body_code: TokenStream2, // The actual code block provided by the user (`$body`)
}

/// **NEW**: Generates the code block `{ ... }` for a single match arm.
/// This includes type declarations, inner binding logic (if applicable), and the final user code.
pub fn generate_match_arm_content(
    variant_info: &ParsedVariantInfo,
    param: &MatchArmParam,
) -> TokenStream2 {
    let MatchArmParam {
        all_unit_variants,
        include_inner,
        include_src_ty,
        src_type_generic,
        include_dest,
        dest_type_generic,
        dest_constraint,
        dest_constraint_generic,
        inner_ident,
        token_type_ident,
        src_type_ident,
        src_type_generic_ident,
        dest_enum_ident,
        dest_type_ident,
        dest_type_generic_ident,
        dest_constraint_ident,
        dest_constraint_generic_ident,
        token_path,
        dtype_variant_path,
        user_body_code,
        ..
    } = param;
    let token_ident = &variant_info.token_ident;
    let src_type = variant_info
        .inner_type
        .as_ref()
        .map(|ty| quote! { #ty })
        .unwrap_or(quote! { () });

    let token_type_path = quote!(#token_path :: #token_ident);

    // --- Type Declarations ---
    let type_declarations = if *all_unit_variants {
        quote! {
            #[allow(unused)] type #token_type_ident = #token_type_path;
        }
    } else {
        let src_generic = src_type_generic
            .then_some(quote! { < #src_type_generic_ident > })
            .unwrap_or_default();
        let inner_decl = include_src_ty
            .then_some(quote! {
                #[allow(unused)] type #src_type_ident #src_generic = #src_type;
            })
            .unwrap_or_default();

        quote! {
            #inner_decl
            #[allow(unused)] type #token_type_ident = #token_type_path;
        }
    };

    // --- Dest Type/Constraint Declarations ---
    let dest_generic = dest_type_generic
        .then_some(quote! { < #dest_type_generic_ident > })
        .unwrap_or_default();
    let dest_constr_generic = dest_constraint_generic
        .then_some(quote! { < #dest_constraint_generic_ident > })
        .unwrap_or_default(); // Separate generic possible

    let dest_type_decl = include_dest
        .then_some(quote! {
            #[allow(unused)]
             type #dest_type_ident #dest_generic = <#dest_enum_ident #dest_generic as #dtype_variant_path::EnumVariantDowncast<#token_type_path>>::Target;
        })
        .unwrap_or_default();

    let dest_constraint_decl = dest_constraint
        .then_some(quote! {
            #[allow(unused)]
             type #dest_constraint_ident #dest_constr_generic = <#dest_enum_ident #dest_constr_generic as #dtype_variant_path::EnumVariantConstraint<#token_type_path>>::Constraint;
        })
        .unwrap_or_default();

    // --- Inner Binding Logic (for unit variants when inner is requested) ---
    // Note: The actual binding `Variant(inner_ident)` happens in the *pattern*.
    // This only handles the case where the pattern expects `inner_ident`, but the variant is Unit.
    let inner_unit_binding = match (include_inner, variant_info.is_unit) {
        (true, true) => (!all_unit_variants)
            .then_some(quote! {
               #[allow(unused_variables, clippy::let_unit_value)]
               let #inner_ident = (); // Provide a unit binding for consistency if inner requested
            })
            .unwrap_or_default(),
        _ => quote! {},
    };

    // --- Combine into the final arm body ---
    quote! {
        { // Wrap in braces
            #inner_unit_binding
            #type_declarations
            #dest_type_decl
            #dest_constraint_decl

            #[allow(unused_braces)] // Allow {$body} even if it's just {}
            #user_body_code // <<< Execute the final user code
        }
    }
}

// --- Generate Macro Arms ---
// Helper *inside* generate_matcher_method to generate the Vec<TokenStream2> of match arms
// MyEnum2::A($inner) => {
//     #[allow(unused)]type$SrcTy = u32;
//     #[allow(unused)]type$TokenTy =  $crate::tokens::AVariant;
//     #[allow(unused)]type$DestTy =  < $DestEnum as dtype_variant::EnumVariantDowncast< $crate::tokens::AVariant>> ::Target;
//     #[allow(unused)]type$ConstraintTy =  < $DestEnum as dtype_variant::EnumVariantConstraint< $crate::tokens::AVariant>> ::Constraint;
//     #[allow(unused_braces)]$body
pub fn generate_match_arms_for_regular_matcher(
    param: &MatchArmParam,
    parsed_variants: &[ParsedVariantInfo],
) -> Vec<TokenStream2> {
    parsed_variants
        .iter()
        .map(|v| {
            let MatchArmParam {
                enum_name,
                include_inner,
                inner_ident,
                ..
            } = param;
            let variant_ident = &v.variant_ident;

            // 1. Generate the pattern
            let pattern = match (include_inner, v.is_unit) {
                (_, true) => quote! { #enum_name::#variant_ident },
                (false, false) => {
                    quote! { #enum_name::#variant_ident(_) }
                }
                (true, false) => {
                    quote! { #enum_name::#variant_ident(#inner_ident) }
                } // Use captured inner_ident
            };

            // 2. Generate the arm body content using the new helper
            let arm_body_content = generate_match_arm_content(v, param);

            // 3. Combine pattern and body
            quote! { #pattern => #arm_body_content }
        })
        .collect::<Vec<_>>() // Collect into Vec<TokenStream2>
}

pub struct MacroRuleArm {
    pub pattern_prefix_fragment: TokenStream2,
    pub pattern_suffix_fragment: TokenStream2,
    pub variant_bodies: TokenStream2,
}

// Helper *inside* generate_matcher_method specific to the regular matcher's macro patterns
//   ($value:expr, $enum_:ident< $SrcTy:ident, $TokenTy:ident>($inner:ident), $DestEnum:ident< $DestTy:ident, $ConstraintTy:ident>  =>  $body:block) => {
// match$value {
pub fn generate_macro_rule_arm(
    enum_name: &Ident,
    parsed_variants: &[ParsedVariantInfo],
    tokens_path: TokenStream2,
    dtype_variant_path: &TokenStream2,
    bindname_suffix: Option<u8>,
) -> impl Fn(bool, bool, bool, bool, bool, bool) -> MacroRuleArm {
    let all_unit_variants = parsed_variants.iter().all(|v| v.is_unit);

    move |include_src_ty: bool,
          include_inner: bool,
          src_type_generic: bool,
          include_dest: bool,
          dest_type_generic: bool,
          dest_constraint: bool| {
        // Define the idents used in this specific macro pattern with optional suffix
        let suffix = bindname_suffix
            .map(|n| format!("{}", n))
            .unwrap_or_default();

        let binding_ts = |name: &str| -> TokenStream2 {
            syn::parse_str::<TokenStream2>(&format!("{}{}", name, suffix))
                .unwrap()
        };

        let inner_ident = binding_ts("$inner");
        let token_type_ident = binding_ts("$TokenTy"); // Choose consistent internal names
        let src_type_ident = binding_ts("$SrcTy");
        let src_type_generic_ident = binding_ts("$SrcGen");
        let dest_enum_ident = binding_ts("$DestEnum");
        let dest_type_ident = binding_ts("$DestTy");
        let dest_type_generic_ident = binding_ts("$DestGen");
        let dest_constraint_ident = binding_ts("$ConstraintTy");
        let dest_constraint_generic_ident = binding_ts("$ConstraintGen");
        let body_ident = binding_ts("$body");
        let enum_ident = binding_ts("$enum_");

        let param = MatchArmParam {
            inner_ident: inner_ident.clone(),
            token_type_ident: token_type_ident.clone(),
            src_type_ident: src_type_ident.clone(),
            src_type_generic_ident: src_type_generic_ident.clone(),
            dest_enum_ident: dest_enum_ident.clone(),
            dest_type_ident: dest_type_ident.clone(),
            dest_type_generic_ident: dest_type_generic_ident.clone(),
            dest_constraint_ident: dest_constraint_ident.clone(),
            dest_constraint_generic_ident: dest_constraint_generic_ident
                .clone(),
            include_src_ty,
            include_inner,
            src_type_generic,
            include_dest,
            dest_constraint,
            dest_type_generic,
            user_body_code: body_ident.clone(),
            enum_name: enum_name.clone(),
            all_unit_variants,
            dest_constraint_generic: dest_type_generic,
            token_path: tokens_path.clone(),
            dtype_variant_path: dtype_variant_path.clone(),
        };

        // Generate the list of match arms using the helper above
        let match_arms =
            generate_match_arms_for_regular_matcher(&param, parsed_variants);

        // Define the outer macro rule pattern (same as before)
        let source_enum_type = if include_src_ty {
            let src_generic = src_type_generic
                .then_some(quote!(<#src_type_generic_ident:tt>))
                .unwrap_or_default();
            quote! { #enum_ident:ident<#src_type_ident:ident #src_generic, #token_type_ident:ident> }
        } else {
            quote! { #enum_ident:ident<#token_type_ident:ident> }
        };
        let macro_arm_inner = include_inner
            .then_some(quote! { (#inner_ident:ident) })
            .unwrap_or_default(); // Use fixed inner_ident
        let (dest_generic, dest_constr_generic) = match dest_type_generic {
            true => (
                quote!(<#dest_type_generic_ident:tt>),
                quote!(<#dest_constraint_generic_ident:tt>),
            ),
            false => (quote!(), quote!()),
        };
        let dest_enum_type = match (include_dest, dest_constraint) {
            (true, true) => {
                quote! { , #dest_enum_ident:ident <#dest_type_ident:ident #dest_generic, #dest_constraint_ident:ident #dest_constr_generic > }
            }
            (true, false) => {
                quote! { , #dest_enum_ident:ident <#dest_type_ident:ident #dest_generic> }
            }
            (false, true) => {
                quote! { , #dest_enum_ident:ident <#dest_constraint_ident:ident #dest_constr_generic> }
            }
            (false, false) => quote!(),
        };

        let pattern_prefix_fragment = quote! { #source_enum_type };
        let pattern_suffix_fragment =
            quote! { #macro_arm_inner #dest_enum_type => #body_ident:block };

        let variant_bodies = quote! {
            #(#match_arms)* // Expand the generated match arms here
        };

        MacroRuleArm {
            pattern_prefix_fragment,
            pattern_suffix_fragment,
            variant_bodies,
        }
    }
}
