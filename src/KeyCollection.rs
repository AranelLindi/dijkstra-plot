use std::collections::hash_map::Keys;
use crate::Graph::Key;
use minidom::Element;
use crate::Graph::key_type::key_enum::KeyType;

trait KeyScope {
    const SCOPE: &'static str;
}

struct NodeScope;
struct EdgeScope;
struct GraphScope;

impl KeyScope for NodeScope {
    const SCOPE: &'static str = "node";
}
impl KeyScope for EdgeScope {
    const SCOPE: &'static str = "edge";
}
impl KeyScope for GraphScope {
    const SCOPE: &'static str = "graph";
}

fn collect_keys_for<T: KeyScope>(keys: &[Element]) -> Vec<Key> {
    keys.iter()
        .filter(|key| key.attr("for") == Some(T::SCOPE))
        .filter_map(|key| {
            // is is a must
            let id = key.attr("id")?;
            // attr.name is a must
            let attrname = key.attr("attr.name")?.to_string();
            // attr.type has to be parsed in KeyType
            let attrtype_str = key.attr("attr.type")?;
            let attrtype = match attrtype_str {
                "boolean" => KeyType::Boolean,
                "int" => KeyType::Int,
                "long" => KeyType::Long,
                "float" => KeyType::Float,
                "double" => KeyType::Double,
                "string" => KeyType::String,
                _ => return None, // invalid type
            };
            
            // default value (optional, empty string otherwise)
            let default = key.children()
                .find(|c| c.name() == "default")
                .map(|c| c.text())
                .unwrap_or_else(|| "".to_string());
            
            Some(Key {
                id: id.to_string(),
                attrname,
                attrtype,
                default,
            })
        })
        .collect()
}