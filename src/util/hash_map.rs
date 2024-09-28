use bevy::utils::HashMap;
use std::hash::Hash;

/// Creates a [`HashMap`](bevy_utils::HashMap) from a list of key-value pairs.
///
/// Example:
/// ```rust
/// let map = bhashmap! {
///    "key1" => "value1",
///    "key2" => "value2"
/// };
/// ```
#[macro_export]
macro_rules! bhashmap {
    ($( $key: expr => $val: expr ),*) => {{
        let mut map = bevy_utils::HashMap::new();
        $(
            map.insert($key, $val);
        )*use bevy_utils::HashMap;
use std::hash::Hash;

fn bhashmap_default<K, V>(keys: impl IntoIterator<Item = K>, val: V) -> HashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    let mut map = HashMap::new();
    for key in keys {
        map.insert(key, val.clone());
    }
    map
}
        map
    }};
}

// /// Creates a [`HashMap`](bevy_utils::HashMap) from a list of key and default value.
// ///
// /// Example:
// /// ```rust
// /// let map = bhashmap_default!([ "key1", "key2", "key3" ], "default_value")
// /// ```
// #[macro_export]
// macro_rules! bhashmap_default {
//     ( [ $( $key:expr ),* ], $val:expr ) => {{
//         let mut map = bevy_utils::HashMap::new();
//         $(
//             map.insert($key, $val.clone());
//         )*
//         map
//     }};
// }

pub fn bhashmap_default<K, V>(keys: impl IntoIterator<Item = K>, val: V) -> HashMap<K, V>
where
    K: Eq + Hash,
    V: Clone,
{
    let mut map = HashMap::new();
    for key in keys {
        map.insert(key, val.clone());
    }
    map
}
