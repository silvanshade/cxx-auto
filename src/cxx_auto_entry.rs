use serde::Deserialize;

#[cfg(feature = "alloc")]
#[derive(Deserialize)]
pub struct CxxAutoEntry<'ctx> {
    cxx_include: &'ctx str,
    cxx_proxy_include: Option<&'ctx str>,
    cxx_namespace: &'ctx str,
    cxx_proxy_namespace: Option<&'ctx str>,
    cxx_name: Option<&'ctx str>,
    rust_name: &'ctx str,
    #[serde(default)]
    rust_lifetimes: ::indexmap::IndexMap<&'ctx str, ::alloc::vec::Vec<&'ctx str>>,
}

#[cfg(feature = "alloc")]
impl<'ctx> CxxAutoEntry<'ctx> {
    #[must_use]
    pub fn cxx_name(&self) -> &str {
        self.cxx_name.unwrap_or(self.rust_name)
    }

    pub(crate) fn emit_items_write_module_for_file<'a, 'b>(
        &self,
        path_components: impl Iterator<Item = &'a ::alloc::string::String>,
        path_descendants: impl Iterator<Item = &'b ::alloc::string::String>,
    ) -> ::alloc::vec::Vec<syn::ItemFn> {
        let cxx_include = self.cxx_include;
        let cxx_namespace = self.cxx_namespace;
        let cxx_name = self.cxx_name();
        let rust_name = self.rust_name;
        let lifetimes = {
            let mut exprs = ::alloc::vec::Vec::<syn::Expr>::new();
            for (lifetime, bounds) in &self.rust_lifetimes {
                exprs.push(syn::parse_quote!((#lifetime, vec![#(#bounds),*])));
            }
            exprs
        };
        ::alloc::vec![
            syn::parse_quote! {
                fn artifact_info() -> ::cxx_auto::CxxAutoArtifactInfo {
                    let path_components = vec![#(#path_components),*];
                    let path_descendants = vec![#(#path_descendants),*];
                    let cxx_include = #cxx_include;
                    let cxx_namespace = #cxx_namespace;
                    let cxx_name = #cxx_name;
                    let rust_name = #rust_name;
                    let lifetimes = ::cxx_auto::indexmap::IndexMap::from_iter([#(#lifetimes),*].into_iter());
                    let align = self::ffi::cxx_abi_align();
                    let size = self::ffi::cxx_abi_size();
                    let cxx_has_operator_equal = self::ffi::cxx_has_operator_equal();
                    let cxx_has_operator_not_equal = self::ffi::cxx_has_operator_not_equal();
                    let cxx_has_operator_less_than = self::ffi::cxx_has_operator_less_than();
                    let cxx_has_operator_less_than_or_equal = self::ffi::cxx_has_operator_less_than_or_equal();
                    let cxx_has_operator_greater_than = self::ffi::cxx_has_operator_greater_than();
                    let cxx_has_operator_greater_than_or_equal = self::ffi::cxx_has_operator_greater_than_or_equal();
                    let is_rust_cxx_extern_type_trivial = {
                        let cxx_is_trivially_movable = self::ffi::cxx_is_trivially_movable();
                        let rust_should_impl_cxx_extern_type_trivial = self::ffi::rust_should_impl_cxx_extern_type_trivial();
                        if cxx_is_trivially_movable == rust_should_impl_cxx_extern_type_trivial {
                            cxx_is_trivially_movable
                        } else {
                            rust_should_impl_cxx_extern_type_trivial
                        }
                    };
                    let is_rust_unpin = self::ffi::rust_should_impl_unpin();
                    let is_rust_send = self::ffi::rust_should_impl_send();
                    let is_rust_sync = self::ffi::rust_should_impl_sync();
                    let is_rust_copy = self::ffi::rust_should_impl_copy();
                    let is_rust_drop = self::ffi::rust_should_impl_drop();
                    let is_rust_debug = self::ffi::rust_should_impl_debug();
                    let is_rust_default = self::ffi::rust_should_impl_default();
                    let is_rust_display = self::ffi::rust_should_impl_display();
                    let is_rust_copy_new = self::ffi::rust_should_impl_moveref_copy_new();
                    let is_rust_move_new = self::ffi::rust_should_impl_moveref_move_new();
                    let is_rust_eq = self::ffi::rust_should_impl_eq();
                    let is_rust_partial_eq = self::ffi::rust_should_impl_partial_eq();
                    let is_rust_partial_ord = self::ffi::rust_should_impl_partial_ord();
                    let is_rust_ord = self::ffi::rust_should_impl_ord();
                    let is_rust_hash = self::ffi::rust_should_impl_hash();
                    ::cxx_auto::CxxAutoArtifactInfo {
                        path_components,
                        path_descendants,
                        cxx_include,
                        cxx_namespace,
                        cxx_name,
                        rust_name,
                        lifetimes,
                        align,
                        size,
                        cxx_has_operator_equal,
                        cxx_has_operator_not_equal,
                        cxx_has_operator_less_than,
                        cxx_has_operator_less_than_or_equal,
                        cxx_has_operator_greater_than,
                        cxx_has_operator_greater_than_or_equal,
                        is_rust_cxx_extern_type_trivial,
                        is_rust_unpin,
                        is_rust_send,
                        is_rust_sync,
                        is_rust_copy,
                        is_rust_debug,
                        is_rust_default,
                        is_rust_display,
                        is_rust_drop,
                        is_rust_copy_new,
                        is_rust_move_new,
                        is_rust_eq,
                        is_rust_partial_eq,
                        is_rust_partial_ord,
                        is_rust_ord,
                        is_rust_hash,
                    }
                }
            },
            syn::parse_quote! {
                pub(crate) fn write_module(auto_out_dir_root: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
                    self::artifact_info().write_module_for_file(auto_out_dir_root)
                }
            },
        ]
    }

    pub(crate) fn emit_item_mod_cxx_bridge(&self) -> [syn::Item; 2] {
        let namespace: syn::Attribute = {
            let namespace = self.cxx_proxy_namespace.unwrap_or(self.cxx_namespace);
            syn::parse_quote!(#[namespace = #namespace])
        };
        let include = self.cxx_proxy_include.unwrap_or(self.cxx_include);
        [
            syn::parse_quote! {
                #[cxx::bridge]
                mod ffi {
                    #namespace
                    unsafe extern "C++" {
                        include!(#include);
                        fn cxx_abi_align() -> usize;
                        fn cxx_abi_size() -> usize;
                        fn cxx_is_copy_constructible() -> bool;
                        fn cxx_is_move_constructible() -> bool;
                        fn cxx_is_default_constructible() -> bool;
                        fn cxx_is_destructible() -> bool;
                        fn cxx_is_trivially_copyable() -> bool;
                        fn cxx_is_trivially_movable() -> bool;
                        fn cxx_is_trivially_destructible() -> bool;
                        fn cxx_is_equality_comparable() -> bool;
                        fn cxx_has_operator_equal() -> bool;
                        fn cxx_has_operator_not_equal() -> bool;
                        fn cxx_has_operator_less_than() -> bool;
                        fn cxx_has_operator_less_than_or_equal() -> bool;
                        fn cxx_has_operator_greater_than() -> bool;
                        fn cxx_has_operator_greater_than_or_equal() -> bool;
                        fn cxx_is_partially_ordered() -> bool;
                        fn cxx_is_totally_ordered() -> bool;
                        fn cxx_is_hashable() -> bool;
                        fn rust_should_impl_cxx_extern_type_trivial() -> bool;
                        fn rust_should_impl_unpin() -> bool;
                        fn rust_should_impl_send() -> bool;
                        fn rust_should_impl_sync() -> bool;
                        fn rust_should_impl_copy() -> bool;
                        fn rust_should_impl_debug() -> bool;
                        fn rust_should_impl_default() -> bool;
                        fn rust_should_impl_display() -> bool;
                        fn rust_should_impl_drop() -> bool;
                        fn rust_should_impl_moveref_copy_new() -> bool;
                        fn rust_should_impl_moveref_move_new() -> bool;
                        fn rust_should_impl_eq() -> bool;
                        fn rust_should_impl_partial_eq() -> bool;
                        fn rust_should_impl_partial_ord() -> bool;
                        fn rust_should_impl_ord() -> bool;
                        fn rust_should_impl_hash() -> bool;
                    }
                }
            },
            syn::parse_quote! {
                pub use ffi::*;
            },
        ]
    }
}
