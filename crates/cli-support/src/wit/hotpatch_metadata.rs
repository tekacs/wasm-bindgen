use super::{AuxImport, Context};
use crate::descriptor::Function;
use crate::hotpatch_metadata::HotpatchPlaceholderMapping;
use crate::PLACEHOLDER_MODULE;
use std::collections::{BTreeSet, HashSet};

impl<'a> Context<'a> {
    fn looks_bindgen_symbol(name: &str) -> bool {
        name.starts_with("__wbindgen")
            || name.starts_with("__wbg_")
            || name.contains("wasm_bindgen")
            || name.contains("wbg_cast")
    }

    pub(super) fn cast_signature(&self, signature: &Function) -> String {
        let [arg] = &signature.arguments[..] else {
            return "<invalid-cast-signature>".to_string();
        };
        format!("{arg:?} -> {:?}", signature.ret)
    }

    fn aux_import_kind(import: &AuxImport) -> (String, Option<String>) {
        match import {
            AuxImport::Cast { sig_comment } => ("cast".to_string(), Some(sig_comment.clone())),
            AuxImport::Intrinsic(_) => ("intrinsic".to_string(), None),
            AuxImport::Value(_) => ("value".to_string(), None),
            AuxImport::ValueWithThis(_, _) => ("value_with_this".to_string(), None),
            AuxImport::Instanceof(_) => ("instanceof".to_string(), None),
            AuxImport::Static { .. } => ("static".to_string(), None),
            AuxImport::String(_) => ("string".to_string(), None),
            AuxImport::StructuralMethod(_) => ("structural_method".to_string(), None),
            AuxImport::StructuralGetter(_) => ("structural_getter".to_string(), None),
            AuxImport::StructuralClassGetter(_, _) => ("structural_class_getter".to_string(), None),
            AuxImport::StructuralSetter(_) => ("structural_setter".to_string(), None),
            AuxImport::StructuralClassSetter(_, _) => ("structural_class_setter".to_string(), None),
            AuxImport::IndexingGetterOfClass(_) => ("indexing_getter_of_class".to_string(), None),
            AuxImport::IndexingGetterOfObject => ("indexing_getter_of_object".to_string(), None),
            AuxImport::IndexingSetterOfClass(_) => ("indexing_setter_of_class".to_string(), None),
            AuxImport::IndexingSetterOfObject => ("indexing_setter_of_object".to_string(), None),
            AuxImport::IndexingDeleterOfClass(_) => ("indexing_deleter_of_class".to_string(), None),
            AuxImport::IndexingDeleterOfObject => ("indexing_deleter_of_object".to_string(), None),
            AuxImport::WrapInExportedClass(_) => ("wrap_in_exported_class".to_string(), None),
            AuxImport::LinkTo(_, _) => ("link_to".to_string(), None),
            AuxImport::UnwrapExportedClass(_) => ("unwrap_exported_class".to_string(), None),
        }
    }

    pub(super) fn finalize_hotpatch_metadata(&mut self) {
        let mut bindgen_symbols = BTreeSet::new();
        for name in self.function_imports.keys() {
            if Self::looks_bindgen_symbol(name) {
                bindgen_symbols.insert(name.clone());
            }
        }
        for name in self.function_exports.keys() {
            if Self::looks_bindgen_symbol(name) {
                bindgen_symbols.insert(name.clone());
            }
        }
        for name in self.descriptors.keys() {
            if Self::looks_bindgen_symbol(name) {
                bindgen_symbols.insert(name.clone());
            }
        }
        for mapping in &self.metadata.cast_mappings {
            bindgen_symbols.insert(mapping.generated_import_name.clone());
            for name in &mapping.original_function_names {
                bindgen_symbols.insert(name.clone());
            }
        }
        self.metadata.bindgen_symbol_set = bindgen_symbols.into_iter().collect();

        let mut placeholder_mappings = Vec::new();
        let mut seen = HashSet::new();
        for (import_id, _, adapter_id) in &self.adapters.implements {
            let import = self.module.imports.get(*import_id);
            if import.module != PLACEHOLDER_MODULE {
                continue;
            }

            let Some(aux_import) = self.aux.import_map.get(adapter_id) else {
                continue;
            };
            let (kind, detail) = Self::aux_import_kind(aux_import);
            let key = (import.name.clone(), kind.clone(), detail.clone());
            if seen.insert(key) {
                placeholder_mappings.push(HotpatchPlaceholderMapping {
                    import_name: import.name.clone(),
                    kind,
                    detail,
                });
            }
        }
        placeholder_mappings.sort_by(|a, b| {
            a.import_name
                .cmp(&b.import_name)
                .then_with(|| a.kind.cmp(&b.kind))
                .then_with(|| a.detail.cmp(&b.detail))
        });
        self.metadata.placeholder_import_mappings = placeholder_mappings;

        for mapping in &mut self.metadata.cast_mappings {
            mapping.original_function_names.sort();
            mapping.original_function_names.dedup();
        }
        self.metadata.cast_mappings.sort_by(|a, b| {
            a.generated_import_name
                .cmp(&b.generated_import_name)
                .then_with(|| a.signature.cmp(&b.signature))
        });
    }
}
