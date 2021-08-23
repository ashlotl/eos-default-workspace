use std::fs;

use proc_macro::TokenStream;

use syn::{parse_macro_input, LitStr};

fn cfg_file_load(path: String) -> Vec<String> {
    let contents = fs::read_to_string(path).unwrap();
    serde_json::from_str(contents.as_str()).unwrap()
}

#[proc_macro]
pub fn cfg_file_load_strs(ast: TokenStream) -> TokenStream {
    let path: LitStr = parse_macro_input!(ast);
    let module_paths = cfg_file_load(path.value());
    let mut ret = String::from(
        "
        let mut macroRet = vec![];
    ",
    );
    for module_path in module_paths {
        ret.push_str(
            format!(
                "
            macroRet.push(\"{}\");
        ",
                module_path.as_str(),
            )
            .as_str(),
        );
    }
    ret.parse().unwrap()
}

#[proc_macro]
pub fn cfg_file_load_idents(ast: TokenStream) -> TokenStream {
    let path: LitStr = parse_macro_input!(ast);
    let module_paths = cfg_file_load(path.value());
    let mut string_ret = String::from(
        "
        #[allow(incorrect_ident_case)]
        let mut macroRet = std::collections::BTreeMap::new();
    ",
    );
    for module_path in module_paths {
        let s = module_path.as_str();
        string_ret.push_str(
            format!(
                "
            macroRet.insert(
                String::from(\"{}\"), std::sync::Arc::new(
                    eos_use::modules::EosModuleHandle::new(
                        {}::eos_module_init,
                        {}::eos_objekt_add,
                        {}::eos_objekt_get,
                        {}::eos_objekt_get_invocations,
                        {}::eos_objekt_remove,
                        {}::eos_objekt_remove_all,
                        {}::eos_objekts_len,
                    )
                )
            );
        ",
                module_path, s, s, s, s, s, s, s,
            )
            .as_str(),
        );
    }
    string_ret.parse().unwrap()
}
