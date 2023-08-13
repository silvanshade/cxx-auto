use proc_macro2::Span;
use syn::punctuated::Punctuated;

#[cfg(feature = "alloc")]
pub struct CxxAutoArtifactInfo {
    pub path_components: ::alloc::vec::Vec<&'static str>,
    pub path_descendants: ::alloc::vec::Vec<&'static str>,
    pub cxx_include: &'static str,
    pub cxx_namespace: &'static str,
    pub cxx_name: &'static str,
    pub rust_name: &'static str,
    pub lifetimes: ::indexmap::IndexMap<&'static str, ::alloc::vec::Vec<&'static str>>,
    pub align: usize,
    pub size: usize,
    pub cxx_has_operator_equal: bool,
    pub cxx_has_operator_not_equal: bool,
    pub cxx_has_operator_less_than: bool,
    pub cxx_has_operator_less_than_or_equal: bool,
    pub cxx_has_operator_greater_than: bool,
    pub cxx_has_operator_greater_than_or_equal: bool,
    pub is_rust_cxx_extern_type_trivial: bool,
    pub is_rust_unpin: bool,
    pub is_rust_send: bool,
    pub is_rust_sync: bool,
    pub is_rust_copy: bool,
    pub is_rust_debug: bool,
    pub is_rust_default: bool,
    pub is_rust_display: bool,
    pub is_rust_drop: bool,
    pub is_rust_copy_new: bool,
    pub is_rust_move_new: bool,
    pub is_rust_eq: bool,
    pub is_rust_partial_eq: bool,
    pub is_rust_partial_ord: bool,
    pub is_rust_ord: bool,
    pub is_rust_hash: bool,
}

#[cfg(feature = "alloc")]
impl CxxAutoArtifactInfo {
    pub fn emit_file(&self, auto_out_dir: &::std::path::Path) -> syn::File {
        let span = Span::call_site();
        let ident: &syn::Ident = &syn::Ident::new(self.rust_name, Span::call_site());
        let align = &proc_macro2::Literal::usize_unsuffixed(self.align);
        let size = &proc_macro2::Literal::usize_unsuffixed(self.size);
        let (generics_binder, generics) = {
            let all_static = false;
            &emit_generics(self, all_static)
        };
        let items_path_descendants = self
            .path_descendants
            .iter()
            .map(|descendant| {
                let path = auto_out_dir.join(descendant).with_extension("rs");
                let path = path.to_string_lossy();
                let ident = syn::Ident::new(descendant, span);
                syn::parse_quote! {
                    #[path = #path]
                    pub(crate) mod #ident;
                }
            })
            .collect::<alloc::vec::Vec<syn::Item>>();
        let item_struct = emit_struct(self, align, size, ident, generics_binder, generics);
        let item_impl_cxx_extern_type = emit_impl_cxx_extern_type(self, ident, generics_binder, generics);
        let item_impl_drop = emit_impl_drop(self, ident, generics_binder, generics);
        let item_impl_debug = emit_impl_debug(self, ident, generics_binder, generics);
        let item_impl_default = emit_impl_default(self, ident, generics_binder, generics);
        let item_impl_display = emit_impl_display(self, ident, generics_binder, generics);
        let item_impl_moveit_copy_new = emit_impl_moveit_copy_new(self, ident, generics_binder, generics);
        let item_impl_moveit_move_new = emit_impl_moveit_move_new(self, ident, generics_binder, generics);
        let item_impl_partial_eq = emit_impl_partial_eq(self, ident, generics_binder, generics);
        let item_impl_eq = emit_impl_eq(self, ident, generics_binder, generics);
        let item_impl_partial_ord = emit_impl_partial_ord(self, ident, generics_binder, generics);
        let item_impl_ord = emit_impl_ord(self, ident, generics_binder, generics);
        let item_impl_hash = emit_impl_hash(self, ident, generics_binder, generics);
        let item_mod_cxx_bridge = emit_item_mod_cxx_bridge(self, ident, generics);
        let item_info_test_module = emit_info_test_module(self, ident, align, size);
        syn::parse_quote! {
            #(#items_path_descendants)*
            #item_struct
            #item_impl_cxx_extern_type
            #item_impl_drop
            #item_impl_default
            #item_impl_moveit_copy_new
            #item_impl_moveit_move_new
            #item_impl_partial_eq
            #item_impl_eq
            #item_impl_partial_ord
            #item_impl_ord
            #item_impl_hash
            #item_impl_debug
            #item_impl_display
            #item_mod_cxx_bridge
            #item_info_test_module
        }
    }

    #[cfg(feature = "std")]
    pub fn write_module_for_dir(
        auto_out_dir_root: &std::path::Path,
        path_components: &[&str],
        path_descendants: &[&str],
    ) -> crate::BoxResult<()> {
        use quote::ToTokens;
        use rust_format::Formatter;
        let auto_out_dir = auto_out_dir_root.join(std::path::PathBuf::from_iter(path_components));
        if let Some(parent) = auto_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = auto_out_dir.with_extension("rs");
        let file: syn::File = {
            let span = Span::call_site();
            let items = path_descendants
                .iter()
                .map(|descendant| {
                    let path = auto_out_dir.join(descendant).with_extension("rs");
                    let path = path.to_string_lossy();
                    let ident = syn::Ident::new(descendant, span);
                    syn::parse_quote! {
                        #[path = #path]
                        pub(crate) mod #ident;
                    }
                })
                .collect::<alloc::vec::Vec<syn::Item>>();
            syn::File {
                shebang: None,
                attrs: alloc::vec![],
                items,
            }
        };
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }

    #[cfg(feature = "std")]
    pub fn write_module_for_file(&self, auto_out_dir_root: &::std::path::Path) -> crate::BoxResult<()> {
        use quote::ToTokens;
        use rust_format::Formatter;
        let auto_out_dir = auto_out_dir_root.join(std::path::PathBuf::from_iter(&self.path_components));
        if let Some(parent) = auto_out_dir.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let path = auto_out_dir.with_extension("rs");
        let file = self.emit_file(&auto_out_dir);
        let tokens = file.to_token_stream();
        let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

#[cfg(feature = "alloc")]
fn emit_struct(
    info: &CxxAutoArtifactInfo,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemStruct {
    let attribute = emit_derive_attribute(info);
    let field_layout = field_layout(size);
    let field_neither_send_nor_sync = field_neither_send_nor_sync(info);
    let field_pinned = field_pinned(info);
    let field_lifetimes = field_lifetimes(generics);
    let fields = syn::FieldsNamed {
        brace_token: syn::token::Brace::default(),
        named: ::alloc::vec![
            Some(field_layout),
            field_neither_send_nor_sync,
            field_pinned,
            field_lifetimes,
        ]
        .into_iter()
        .flatten()
        .collect::<Punctuated<syn::Field, syn::Token![,]>>(),
    };
    syn::parse_quote! {
        #attribute
        #[repr(C, align(#align))]
        pub struct #ident #generics_binder #fields
    }
}

#[cfg(feature = "alloc")]
fn emit_derive_attribute(info: &CxxAutoArtifactInfo) -> Option<syn::Attribute> {
    if info.is_rust_copy {
        Some(syn::parse_quote!(#[derive(Clone, Copy)]))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_field(name: &str, ty: syn::Type) -> syn::Field {
    syn::Field {
        attrs: ::alloc::vec![],
        vis: syn::Visibility::Inherited,
        mutability: syn::FieldMutability::None,
        ident: Some(syn::Ident::new(name, Span::call_site())),
        colon_token: Some(syn::Token![:](Span::call_site())),
        ty,
    }
}

#[cfg(feature = "alloc")]
fn emit_generics(info: &CxxAutoArtifactInfo, all_static: bool) -> (syn::Generics, syn::Generics) {
    let span = Span::call_site();
    let mut binder_params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    let mut params = Punctuated::<syn::GenericParam, syn::Token![,]>::new();
    for (name, bounds) in &info.lifetimes {
        let name = if all_static { "static" } else { name };

        let lifetime = syn::Lifetime::new(&::alloc::format!("'{name}"), span);
        let lifetime_param = syn::LifetimeParam::new(lifetime);

        let mut lifetime_param_binder = lifetime_param.clone();
        for bound in bounds {
            let lifetime = syn::Lifetime::new(&::alloc::format!("'{bound}"), span);
            lifetime_param_binder.bounds.push_value(lifetime);
        }

        binder_params.push(syn::GenericParam::from(lifetime_param_binder));
        params.push(syn::GenericParam::from(lifetime_param))
    }
    let lt_token = if params.is_empty() {
        None
    } else {
        Some(syn::Token![<](span))
    };
    let gt_token = if params.is_empty() {
        None
    } else {
        Some(syn::Token![>](span))
    };
    let where_clause = None;
    (
        syn::Generics {
            lt_token,
            gt_token,
            params: binder_params,
            where_clause: where_clause.clone(),
        },
        syn::Generics {
            lt_token,
            gt_token,
            params,
            where_clause,
        },
    )
}

#[cfg(feature = "alloc")]
fn emit_impl_cxx_extern_type(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl {
    let type_id = ::alloc::format!("{}::{}", info.cxx_namespace, info.cxx_name);
    let kind: syn::Type = if info.is_rust_cxx_extern_type_trivial {
        syn::parse_quote!(::cxx::kind::Trivial)
    } else {
        syn::parse_quote!(::cxx::kind::Opaque)
    };
    syn::parse_quote! {
        unsafe impl #generics_binder ::cxx::ExternType for #ident #generics {
            type Id = ::cxx::type_id!(#type_id);
            type Kind = #kind;
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_drop(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_drop {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::ops::Drop for #ident #generics {
                #[cfg_attr(feature = "tracing", tracing::instrument)]
                #[inline]
                fn drop(&mut self) {
                    unsafe {
                        self::ffi::cxx_destruct(self);
                    }
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_debug(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> syn::ItemImpl {
    if info.is_rust_debug {
        syn::parse_quote! {
            impl #generics_binder ::core::fmt::Debug for #ident #generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let string = self::ffi::cxx_debug(self);
                    write!(f, "{string}")
                }
            }
        }
    } else {
        let name = info.rust_name;
        syn::parse_quote! {
            impl #generics_binder ::core::fmt::Debug for #ident #generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.debug_struct(#name).finish()
                }
            }
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_default(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_default {
        Some(syn::parse_quote! {
            impl #generics_binder #ident #generics {
                #[inline]
                pub(crate) fn default_new() -> impl ::moveref::New<Output = #ident #generics> {
                    unsafe {
                        ::moveref::new::by_raw(move |this| {
                            let this = this.get_unchecked_mut().as_mut_ptr();
                            self::ffi::cxx_default_new(this);
                        })
                    }
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_display(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_display {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::fmt::Display for #ident #generics {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    let string = self::ffi::cxx_display(self);
                    write!(f, "{string}")
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_moveit_copy_new(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_copy_new {
        Some(syn::parse_quote! {
            impl #generics_binder ::moveref::CopyNew for #ident #generics {
                #[inline]
                unsafe fn copy_new(that: &Self, this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>) {
                    let this = this.get_unchecked_mut().as_mut_ptr();
                    self::ffi::cxx_copy_new(this, that)
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_moveit_move_new(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_move_new {
        Some(syn::parse_quote! {
            impl #generics_binder ::moveref::MoveNew for #ident #generics {
                #[inline]
                unsafe fn move_new(
                    that: ::core::pin::Pin<::moveref::MoveRef<'_, Self>>,
                    this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<Self>>,
                ) {
                    let this = this.get_unchecked_mut().as_mut_ptr();
                    let that = &mut *::core::pin::Pin::into_inner_unchecked(that);
                    self::ffi::cxx_move_new(this, that)
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_partial_eq(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_partial_eq {
        let ne: Option<syn::ImplItemFn> = if info.cxx_has_operator_not_equal {
            Some(syn::parse_quote! {
                #[allow(clippy::partialeq_ne_impl)]
                #[inline]
                fn ne(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_not_equal(self, other)
                }
            })
        } else {
            None
        };
        Some(syn::parse_quote! {
            impl #generics_binder ::core::cmp::PartialEq for #ident #generics {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_equal(self, other)
                }
                #ne
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_eq(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_eq {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::cmp::Eq for #ident #generics {}
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_partial_ord(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_partial_ord {
        let lt: Option<syn::ImplItemFn> = if info.cxx_has_operator_less_than {
            Some(syn::parse_quote! {
                #[inline]
                fn lt(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_less_than(self, other)
                }
            })
        } else {
            None
        };
        let le: Option<syn::ImplItemFn> = if info.cxx_has_operator_less_than_or_equal {
            Some(syn::parse_quote! {
                #[inline]
                fn le(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_less_than_or_equal(self, other)
                }
            })
        } else {
            None
        };
        let gt: Option<syn::ImplItemFn> = if info.cxx_has_operator_greater_than {
            Some(syn::parse_quote! {
                #[inline]
                fn gt(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_greater_than(self, other)
                }
            })
        } else {
            None
        };
        let ge: Option<syn::ImplItemFn> = if info.cxx_has_operator_greater_than_or_equal {
            Some(syn::parse_quote! {
                #[inline]
                fn ge(&self, other: &Self) -> bool {
                    self::ffi::cxx_operator_greater_than_or_equal(self, other)
                }
            })
        } else {
            None
        };
        Some(syn::parse_quote! {
            impl #generics_binder ::core::cmp::PartialOrd for #ident #generics {
                #[inline]
                fn partial_cmp(&self, other: &Self) -> Option<::core::cmp::Ordering> {
                    let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                    if res == -1 {
                        Some(::core::cmp::Ordering::Less)
                    } else if res == 1 {
                        Some(::core::cmp::Ordering::Greater)
                    } else if res == 0 {
                        Some(::core::cmp::Ordering::Equal)
                    } else {
                        ::core::assert_eq!(res, ::core::primitive::i8::MAX);
                        None
                    }
                }
                #lt
                #le
                #gt
                #ge
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_ord(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_ord {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::cmp::Ord for #ident #generics {
                #[inline]
                fn cmp(&self, other: &Self) -> ::core::cmp::Ordering {
                    let res = self::ffi::cxx_operator_three_way_comparison(self, other);
                    res.cmp(&0)
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_impl_hash(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    generics_binder: &syn::Generics,
    generics: &syn::Generics,
) -> Option<syn::ItemImpl> {
    if info.is_rust_hash {
        Some(syn::parse_quote! {
            impl #generics_binder ::core::hash::Hash for #ident #generics {
                #[inline]
                fn hash<H>(&self, state: &mut H)
                where
                    H: ::core::hash::Hasher,
                {
                    let hash = self::ffi::cxx_hash(self);
                    state.write_usize(hash);
                }
            }
        })
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn emit_info_test_module(
    info: &CxxAutoArtifactInfo,
    ident: &syn::Ident,
    align: &proc_macro2::Literal,
    size: &proc_macro2::Literal,
) -> syn::ItemMod {
    let (_, generics) = {
        let all_static = true;
        &emit_generics(info, all_static)
    };
    let static_assert_is_copy: Option<syn::ItemMacro> = if info.is_rust_copy {
        Some(syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Copy);
        ))
    } else {
        None
    };
    let static_assert_is_unpin: Option<syn::ItemMacro> = if info.is_rust_unpin {
        Some(syn::parse_quote!(
            ::static_assertions::assert_impl_all!(#ident #generics: ::core::marker::Unpin);
        ))
    } else {
        None
    };
    syn::parse_quote! {
        #[cfg(test)]
        mod info {
            use super::*;
            mod test {
                use super::*;
                #[test]
                fn cxx_abi_align() {
                    ::core::assert_eq!(::core::mem::align_of::<#ident #generics>(), #align)
                }
                #[test]
                fn cxx_abi_size() {
                    ::core::assert_eq!(::core::mem::size_of::<#ident #generics>(), #size)
                }
                #static_assert_is_copy
                #static_assert_is_unpin
            }
        }
    }
}

#[cfg(feature = "alloc")]
fn emit_item_mod_cxx_bridge(info: &CxxAutoArtifactInfo, ident: &syn::Ident, generics: &syn::Generics) -> syn::ItemMod {
    let cxx_include = &info.cxx_include;
    let cxx_namespace = &info.cxx_namespace;
    let cxx_name = &info.cxx_name;
    let cxx_copy_new: Option<syn::ForeignItemFn> = if info.is_rust_copy_new {
        Some(syn::parse_quote! {
            unsafe fn cxx_copy_new #generics (This: *mut #ident #generics, that: &#ident #generics);
        })
    } else {
        None
    };
    let cxx_move_new: Option<syn::ForeignItemFn> = if info.is_rust_move_new {
        Some(syn::parse_quote! {
            unsafe fn cxx_move_new #generics (This: *mut #ident #generics, that: *mut #ident #generics);
        })
    } else {
        None
    };
    let cxx_default_new: Option<syn::ForeignItemFn> = if info.is_rust_default {
        Some(syn::parse_quote! {
            unsafe fn cxx_default_new #generics (This: *mut #ident #generics);
        })
    } else {
        None
    };
    let cxx_destruct: Option<syn::ForeignItemFn> = if info.is_rust_drop {
        Some(syn::parse_quote! {
            unsafe fn cxx_destruct #generics (This: *mut #ident #generics);
        })
    } else {
        None
    };
    let cxx_operator_equal: Option<syn::ForeignItemFn> = if info.is_rust_eq {
        Some(syn::parse_quote! {
            fn cxx_operator_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_not_equal: Option<syn::ForeignItemFn> = if info.is_rust_eq {
        Some(syn::parse_quote! {
            fn cxx_operator_not_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_less_than: Option<syn::ForeignItemFn> = if info.cxx_has_operator_less_than {
        Some(syn::parse_quote! {
            fn cxx_operator_less_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_less_than_or_equal: Option<syn::ForeignItemFn> = if info.cxx_has_operator_less_than_or_equal {
        Some(syn::parse_quote! {
            fn cxx_operator_less_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_greater_than: Option<syn::ForeignItemFn> = if info.cxx_has_operator_greater_than {
        Some(syn::parse_quote! {
            fn cxx_operator_greater_than #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_greater_than_or_equal: Option<syn::ForeignItemFn> = if info.cxx_has_operator_greater_than_or_equal
    {
        Some(syn::parse_quote! {
            fn cxx_operator_greater_than_or_equal #generics (This: & #ident #generics, That: & #ident #generics) -> bool;
        })
    } else {
        None
    };
    let cxx_operator_three_way_comparison: Option<syn::ForeignItemFn> = if info.is_rust_partial_ord {
        Some(syn::parse_quote! {
            fn cxx_operator_three_way_comparison #generics (This: & #ident #generics, That: & #ident #generics) -> i8;
        })
    } else {
        None
    };
    let cxx_hash: Option<syn::ForeignItemFn> = if info.is_rust_hash {
        Some(syn::parse_quote! {
            fn cxx_hash #generics (This: & #ident #generics) -> usize;
        })
    } else {
        None
    };
    let cxx_debug: Option<syn::ForeignItemFn> = if info.is_rust_debug {
        Some(syn::parse_quote! {
            fn cxx_debug #generics (This: & #ident #generics) -> String;
        })
    } else {
        None
    };
    let cxx_display: Option<syn::ForeignItemFn> = if info.is_rust_display {
        Some(syn::parse_quote! {
            fn cxx_display #generics (This: & #ident #generics) -> String;
        })
    } else {
        None
    };
    syn::parse_quote! {
        #[cxx::bridge]
        pub(crate) mod ffi {
            #![allow(clippy::needless_lifetimes)]
            #[namespace = #cxx_namespace]
            unsafe extern "C++" {
                include!(#cxx_include);

                #[cxx_name = #cxx_name]
                #[allow(unused)]
                type #ident #generics = super :: #ident #generics;
                #cxx_copy_new
                #cxx_move_new
                #cxx_default_new
                #cxx_destruct
                #cxx_operator_equal
                #cxx_operator_not_equal
                #cxx_operator_less_than
                #cxx_operator_less_than_or_equal
                #cxx_operator_greater_than
                #cxx_operator_greater_than_or_equal
                #cxx_operator_three_way_comparison
                #cxx_hash
                #cxx_debug
                #cxx_display
            }
        }
    }
}

fn emit_refs_from_lifetimes(generics: &syn::Generics) -> Punctuated<syn::Type, syn::Token![,]> {
    generics
        .params
        .iter()
        .filter_map(emit_ref_type_from_lifetime)
        .collect::<Punctuated<syn::Type, syn::Token![,]>>()
}

fn emit_ref_type_from_lifetime(generic_param: &syn::GenericParam) -> Option<syn::Type> {
    if let syn::GenericParam::Lifetime(lifetime_param) = generic_param {
        let lifetime = &lifetime_param.lifetime;
        Some(syn::parse_quote!(&#lifetime ()))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn field_layout(size: &proc_macro2::Literal) -> syn::Field {
    let name = "_layout";
    let ty = syn::parse_quote!([u8; #size]);
    emit_field(name, ty)
}

#[cfg(feature = "alloc")]
fn field_neither_send_nor_sync(info: &CxxAutoArtifactInfo) -> Option<syn::Field> {
    let is_neither_send_nor_sync = !info.is_rust_send && !info.is_rust_sync;
    if is_neither_send_nor_sync {
        let name = "_neither_send_nor_sync";
        let ty = syn::parse_quote!(::core::marker::PhantomData<[*const u8; 0]>);
        Some(emit_field(name, ty))
    } else {
        None
    }
}

#[cfg(feature = "alloc")]
fn field_pinned(info: &CxxAutoArtifactInfo) -> Option<syn::Field> {
    if info.is_rust_unpin {
        None
    } else {
        let name = "_pinned";
        let ty = syn::parse_quote!(::core::marker::PhantomPinned);
        Some(emit_field(name, ty))
    }
}

#[cfg(feature = "alloc")]
fn field_lifetimes(generics: &syn::Generics) -> Option<syn::Field> {
    let ref_types = emit_refs_from_lifetimes(generics);
    if ref_types.is_empty() {
        None
    } else {
        let name = "_lifetimes";
        let ty = syn::parse_quote!(::core::marker::PhantomData<(#ref_types,)>);
        Some(emit_field(name, ty))
    }
}
