use proc_macro::TokenStream;

#[cfg(feature = "binary")]
const CRATE_NAME: &str = "crate";
#[cfg(not(feature = "binary"))]
const CRATE_NAME: &str = "r4d";

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

#[proc_macro]
pub fn expand(item: TokenStream) -> TokenStream {
    format!("processor.expand(level,{})", item).parse().unwrap()
}

#[proc_macro]
pub fn split_args(item: TokenStream) -> TokenStream {
    format!("processor.get_split_arguments({}, args)", item)
        .parse()
        .unwrap()
}

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
