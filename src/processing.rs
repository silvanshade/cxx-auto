#[cfg(feature = "alloc")]
use crate::BoxResult;

use proc_macro2::Span;
use quote::ToTokens;
use rust_format::Formatter;
#[cfg(feature = "std")]
use std::collections::BTreeSet;

pub(crate) fn process_src_auto_module(
    project_dir: &std::path::Path,
    out_dir: &std::path::Path,
    cfg_dir: &std::path::Path,
) -> BoxResult<()> {
    let cfg_dir_walker = walkdir::WalkDir::new(cfg_dir).min_depth(1);
    let skip_paths = std::collections::BTreeSet::new();
    let mut walked_path_components = ::alloc::vec::Vec::new();

    process_src_auto_sub_module(
        project_dir,
        out_dir,
        cfg_dir_walker.into_iter(),
        skip_paths,
        &mut walked_path_components,
    )?;

    walked_path_components.sort();

    let contents = {
        let mut path_descendants = BTreeSet::new();
        let mut item_mods = ::alloc::vec![];
        emit_item_mods_for_path_descendants(
            project_dir,
            out_dir,
            cfg_dir,
            &mut std::collections::BTreeSet::new(),
            &mut path_descendants,
            &mut item_mods,
        )?;
        let item_write_module = emit_item_write_module_for_dir(&::alloc::vec![], &path_descendants);
        let item_fn_process_artifact_infos =
            emit_item_fn_process_artifact_infos(([::alloc::vec![]]).iter().chain(walked_path_components.iter()));
        let file: syn::File = syn::parse_quote! {
            #(#item_mods)*
            #item_write_module
            #item_fn_process_artifact_infos
        };
        let tokens = file.to_token_stream();
        rust_format::RustFmt::default().format_tokens(tokens)?
    };
    std::fs::write(out_dir.join("auto.rs"), contents)?;

    Ok(())
}

#[cfg(feature = "std")]
fn process_src_auto_sub_module(
    project_dir: &std::path::Path,
    out_dir: &std::path::Path,
    mut cfg_dir_walker: impl Iterator<Item = walkdir::Result<walkdir::DirEntry>>,
    mut skip_paths: std::collections::BTreeSet<std::path::PathBuf>,
    walked_path_file_components: &mut ::alloc::vec::Vec<::alloc::vec::Vec<::alloc::string::String>>,
) -> BoxResult<()> {
    if let Some(entry) = cfg_dir_walker.next().transpose()? {
        let path = entry.path();

        if !skip_paths.contains(path) {
            let path_dir = Some(path.to_path_buf())
                .filter(|p| p.is_dir())
                .or_else(|| Some(path.with_extension("")).filter(|p| p.is_dir()));
            let path_file = Some(path.to_path_buf())
                .filter(|p| p.is_file())
                .or_else(|| Some(path.with_extension("json")).filter(|p| p.is_file()));

            let path_components = relativized_components_from_path(path)?;
            let mut path_descendants = BTreeSet::new();
            let mut item_mods = ::alloc::vec![];

            if let Some(path) = &path_dir {
                emit_item_mods_for_path_descendants(
                    project_dir,
                    out_dir,
                    path,
                    &mut skip_paths,
                    &mut path_descendants,
                    &mut item_mods,
                )?;
            }

            let mut items_write_module: ::alloc::vec::Vec<syn::ItemFn> = ::alloc::vec![];
            let mut item_mod_cxx_bridge: ::alloc::vec::Vec<syn::Item> = ::alloc::vec![];

            if let Some(path) = &path_file {
                skip_paths.insert(path.clone());
                let text = std::fs::read_to_string(path)?;
                let data = serde_json::from_str::<crate::CxxAutoEntry>(&text)?;
                items_write_module =
                    data.emit_items_write_module_for_file(path_components.iter(), path_descendants.iter());
                item_mod_cxx_bridge.extend(data.emit_item_mod_cxx_bridge());
            } else {
                items_write_module.push(emit_item_write_module_for_dir(&path_components, &path_descendants));
            }

            let auto_sub_module_path = out_dir.join(
                [::alloc::string::String::from("auto")]
                    .iter()
                    .chain(&path_components)
                    .collect::<std::path::PathBuf>(),
            );

            walked_path_file_components.push(path_components);

            if let Some(parent) = auto_sub_module_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            write_auto_sub_module(
                &auto_sub_module_path.with_extension("rs"),
                &item_mods,
                &item_mod_cxx_bridge,
                &items_write_module,
            )?;
        }
        process_src_auto_sub_module(
            project_dir,
            out_dir,
            cfg_dir_walker,
            skip_paths,
            walked_path_file_components,
        )?;
    }
    Ok(())
}

fn emit_item_mods_for_path_descendants(
    project_dir: &std::path::Path,
    out_dir: &std::path::Path,
    path: &std::path::Path,
    skip_paths: &mut BTreeSet<std::path::PathBuf>,
    path_descendants: &mut BTreeSet<::alloc::string::String>,
    items: &mut ::alloc::vec::Vec<syn::ItemMod>,
) -> BoxResult<()> {
    skip_paths.insert(path.to_path_buf());
    find_immediate_path_descendants(path, path_descendants)?;
    let span = Span::call_site();
    for descendant in &*path_descendants {
        let ident = syn::Ident::new(descendant, span);
        let descendant_path = path.join(descendant).with_extension("rs");
        let mod_suffix = descendant_path.strip_prefix(project_dir)?;
        let mod_path = out_dir.join(mod_suffix);
        let mod_path_string = mod_path.to_string_lossy();
        items.push(syn::parse_quote! {
            #[path = #mod_path_string]
            pub mod #ident;
        });
    }
    Ok(())
}

#[cfg(feature = "std")]
fn emit_item_write_module_for_dir(
    path_components: &::alloc::vec::Vec<::alloc::string::String>,
    path_descendants: &BTreeSet<::alloc::string::String>,
) -> syn::ItemFn {
    syn::parse_quote! {
        pub(crate) fn write_module(out_dir: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
            let path_components = &[#(#path_components),*];
            let path_descendants = &[#(#path_descendants),*];
            ::cxx_auto::CxxAutoArtifactInfo::write_module_for_dir(out_dir, path_components, path_descendants)
        }
    }
}

#[cfg(feature = "std")]
fn emit_item_fn_process_artifact_infos<'a>(
    walked_path_components: impl Iterator<Item = &'a ::alloc::vec::Vec<::alloc::string::String>>,
) -> syn::ItemFn {
    let span = Span::call_site();
    let items = walked_path_components.map(|path_components| -> syn::Stmt {
        let path = syn::Path {
            leading_colon: None,
            segments: path_components
                .iter()
                .map(|component| syn::PathSegment::from(syn::Ident::new(component, span)))
                .collect(),
        };
        if path_components.is_empty() {
            syn::parse_quote!(self::write_module(auto_out_dir_root)?;)
        } else {
            syn::parse_quote!(self::#path::write_module(auto_out_dir_root)?;)
        }
    });
    syn::parse_quote! {
        pub fn process_artifacts(out_dir: &::std::path::Path) -> ::cxx_auto::BoxResult<()> {
            let auto_out_dir_root = &out_dir.join("src/auto");
            #(#items)*
            Ok(())
        }
    }
}

#[cfg(feature = "std")]
fn find_immediate_path_descendants(
    path: &std::path::Path,
    path_descendants: &mut BTreeSet<::alloc::string::String>,
) -> BoxResult<()> {
    for result in walkdir::WalkDir::new(path).min_depth(1).max_depth(1) {
        let entry = result?;
        if let Some(file_stem) = entry
            .path()
            .file_stem()
            .map(|s| {
                s.to_os_string()
                    .into_string()
                    .map_err(|err| ::alloc::format!("Failed to convert to String: {err:?}"))
            })
            .transpose()?
        {
            path_descendants.insert(file_stem);
        }
    }
    Ok(())
}

#[cfg(feature = "std")]
fn relativized_components_from_path(path: &std::path::Path) -> BoxResult<::alloc::vec::Vec<::alloc::string::String>> {
    let path_components = path
        .with_extension("")
        .components()
        .skip_while(|component| component.as_os_str() != "auto")
        .skip(1)
        .map(|component| {
            component
                .as_os_str()
                .to_os_string()
                .into_string()
                .map_err(|err| ::alloc::format!("Failed to convert to String: {err:?}"))
        })
        .collect::<Result<::alloc::vec::Vec<_>, _>>()?;
    Ok(path_components)
}

#[cfg(feature = "std")]
fn write_auto_sub_module(
    path: &std::path::Path,
    item_mods: &[syn::ItemMod],
    item_mod_cxx_bridge: &[syn::Item],
    items_write_module: &[syn::ItemFn],
) -> BoxResult<()> {
    let file: syn::File = syn::parse_quote! {
        #(#item_mods)*
        #(#item_mod_cxx_bridge)*
        #(#items_write_module)*
    };
    let tokens = file.to_token_stream();
    let contents = rust_format::RustFmt::default().format_tokens(tokens)?;
    std::fs::write(path.with_extension("rs"), contents)?;
    Ok(())
}
