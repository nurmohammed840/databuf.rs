pub mod decode;
pub mod encode;

use proc_macro2::*;
use quote::{quote, quote_each_token, quote_token, ToTokens, TokenStreamExt};
use syn::{__private::parse_quote, spanned::Spanned, *};

#[macro_export]
macro_rules! group {
    ($out: expr, $delim: ident, $s: ident, $body: block) => {
        $out.append(Group::new(Delimiter::$delim, {
            let mut $s = TokenStream::new();
            $body
            $s
        }))
    };
}
