/// btreeを作成する
/// - `build_tree! {}`
#[macro_export]
macro_rules! build_tree {
    ( keys: $keys:expr , vals: $vals:expr , size: $size:expr $(,)* ) => {
        Some(Box::new(BTreeNode {
            keys: $keys,
            vals: $vals,
            children: None,
            size: $size,
        }))
    };
    ( keys: $keys:expr , vals: $vals:expr , children: $children:expr , size: $size:expr $(,)* ) => {
        Some(Box::new(BTreeNode {
            keys: $keys,
            vals: $vals,
            children: Some($children),
            size: $size,
        }))
    };
}
