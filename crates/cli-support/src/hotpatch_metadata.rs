#[derive(Debug, Clone, Default, serde::Serialize)]
pub(crate) struct HotpatchMetadata {
    pub schema_version: u32,
    pub bindgen_symbol_set: Vec<String>,
    pub cast_mappings: Vec<HotpatchCastMapping>,
    pub placeholder_import_mappings: Vec<HotpatchPlaceholderMapping>,
}

impl HotpatchMetadata {
    pub(crate) fn new() -> Self {
        Self {
            schema_version: 1,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HotpatchCastMapping {
    pub generated_import_name: String,
    pub signature: String,
    pub original_function_names: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct HotpatchPlaceholderMapping {
    pub import_name: String,
    pub kind: String,
    pub detail: Option<String>,
}
