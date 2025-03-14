#![forbid(unsafe_code)]
extern crate proc_macro;
#[allow(unused_imports)]
use proc_macro::TokenStream;
use core::fmt::Write;

#[allow(dead_code)]
fn generic_type_param(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| { let _ = write!(s, "T{}, ", i); s })
}

#[allow(dead_code)]
fn from_tup_fn_ident(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| { let _ = write!(s, "t{}.into(), ", i); s })
}

#[allow(dead_code)]
fn from_tup_type_bound(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| { let _ = write!(s, "From<T{}> + ", i); s })
}

#[allow(dead_code)]
fn from_tup_value_ident(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| { let _ = write!(s, "t{}, ", i); s })
}

#[allow(dead_code)]
#[rustfmt::skip]
fn from_tup_trait_code(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(s,
"   #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    fn from_{i}<{type_param}>(tup: ( {type_param} ) ) -> [Self; {i}]
        where
             Self: {type_bound};

",
        type_param = generic_type_param(i),
        type_bound = from_tup_type_bound(i),
        type_doc = generic_type_param(i).trim_end(),
        i = i); s })
}

#[allow(dead_code)]
#[rustfmt::skip]
fn from_tup_impl_code(n: usize) -> String {
    (1..=n).fold("".to_string(), |mut s, i| {
        let _ = write!(s,
"   #[doc = \"Converts tuple ({type_doc}) to array [Self; {i}].\"]
    #[inline] 
    fn from_{i}<{type_param}>(tup: ( {type_param} ) ) -> [Self; {i}]
        where
             Self: {type_bound},
    {{
        let ( {value_ident} ) = tup;
        [ {fn_ident}]
    }}
",
        type_param = generic_type_param(i),
        fn_ident = from_tup_fn_ident(i),
        type_bound = from_tup_type_bound(i),
        value_ident = from_tup_value_ident(i),
        type_doc = generic_type_param(i).trim_end(),
        i = i,); s })
}

#[allow(unused_macros)]
macro_rules! from_tup_trait {
    ($to:expr) => {
        #[proc_macro]
        pub fn tup_from_trait(_item: TokenStream) -> TokenStream {
            from_tup_trait_code($to).parse().unwrap()
        }

        #[proc_macro]
        pub fn tup_from_impl(_item: TokenStream) -> TokenStream {
            from_tup_impl_code($to).parse().unwrap()
        }
    };
}

#[cfg(all(
    feature = "from_tup_8",
    not(feature = "from_tup_16"),
    not(feature = "from_tup_32"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(8);

#[cfg(all(
    feature = "from_tup_16",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_32"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(16);
#[cfg(all(
    feature = "from_tup_16",
    feature = "from_tup_8",
    not(feature = "from_tup_32"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(16);

#[cfg(all(
    feature = "from_tup_32",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_16"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(32);
#[cfg(all(
    feature = "from_tup_32",
    feature = "from_tup_8",
    not(feature = "from_tup_16"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(32);
#[cfg(all(
    feature = "from_tup_32",
    feature = "from_tup_16",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_64")
))]
from_tup_trait!(32);
#[cfg(all(
    feature = "from_tup_32",
    feature = "from_tup_8",
    feature = "from_tup_16",
    not(feature = "from_tup_64")
))]
from_tup_trait!(32);

#[cfg(all(
    feature = "from_tup_64",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_16"),
    not(feature = "from_tup_32")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_8",
    not(feature = "from_tup_16"),
    not(feature = "from_tup_32")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_16",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_32")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_32",
    not(feature = "from_tup_8"),
    not(feature = "from_tup_16")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_8",
    feature = "from_tup_16",
    not(feature = "from_tup_32")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_8",
    feature = "from_tup_32",
    not(feature = "from_tup_16")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_16",
    feature = "from_tup_32",
    not(feature = "from_tup_8")
))]
from_tup_trait!(64);
#[cfg(all(
    feature = "from_tup_64",
    feature = "from_tup_8",
    feature = "from_tup_16",
    feature = "from_tup_32"
))]
from_tup_trait!(64);
