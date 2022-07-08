use proc_macro::TokenStream;

#[cfg(feature = "binary")]
const CRATE_NAME: &str = "crate";
#[cfg(not(feature = "binary"))]
const CRATE_NAME: &str = "r4d";

/// Create function macro signature
#[proc_macro]
pub fn function_template(item: TokenStream) -> TokenStream {
    format!(
        "|args : &str, processor : &mut Processor| -> RadResult<Option<String>> {{ 
        {}
}}",
        item
    )
    .parse()
    .unwrap()
}

/// Create deterred macro signature
#[proc_macro]
pub fn deterred_template(item: TokenStream) -> TokenStream {
    format!(
        "|args : &str, level: usize, processor : &mut Processor| -> RadResult<Option<String>> {{ 
        {}
}}",
        item
    )
    .parse()
    .unwrap()
}

/// Deterred macro's argument expansion and strip
///
/// # Usage
///
/// expand_args!(&args[0])
#[proc_macro]
pub fn expand_args(item: TokenStream) -> TokenStream {
    format!("processor.expand(level,{},true)", item)
        .parse()
        .unwrap()
}

/// Generic expansion without strip
///
/// # Usage
///
/// expand_expr!("Expresion to expand")
#[proc_macro]
pub fn expand_expr(item: TokenStream) -> TokenStream {
    format!("processor.expand(level,{},false)", item)
        .parse()
        .unwrap()
}

/// Split arguments
///
/// # Usage
///
/// split_args!(len,args,bool)
#[proc_macro]
pub fn split_args(item: TokenStream) -> TokenStream {
    format!("processor.split_arguments(args,{})", item)
        .parse()
        .unwrap()
}

/// Audith authentication
///
/// # Usage
///
/// audit_auth("macro_name", AuthType::CMD)
#[proc_macro]
pub fn audit_auth(item: TokenStream) -> TokenStream {
    let token_string = item.to_string();
    let tokens = token_string.split(',').collect::<Vec<_>>();
    let (name, atype) = (tokens[0], tokens[1]);
    if tokens.len() < 2 {
        panic!("audit_auth macro needs two arguments of AuthType and macro name");
    } else {
        format!(
        "if !processor.check_auth({1})? {{ return Err({2}::RadError::PermissionDenied({0}.to_string(),{1})) }}",
        name, atype,CRATE_NAME
    )
        .parse()
        .unwrap()
    }
}
