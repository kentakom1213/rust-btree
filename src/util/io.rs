//! jsonとB木の相互変換

use serde::{ser::SerializeStruct, Serialize};

use crate::node::node::BTreeNode;

impl<const D: usize, K, V> Serialize for BTreeNode<D, K, V>
where
    [(); 2 * D - 1]:,
    K: Serialize,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let BTreeNode {
            keys,
            vals,
            size,
            children,
        } = &self;

        let keys_vec = (0..*size)
            .map(|i| keys[i].as_ref().unwrap())
            .collect::<Vec<_>>();
        let vals_vec = (0..*size)
            .map(|i| vals[i].as_ref().unwrap())
            .collect::<Vec<_>>();

        let mut state =
            serializer.serialize_struct("BTreeNode", if children.is_some() { 3 } else { 2 })?;

        state.serialize_field("keys", &keys_vec)?;
        state.serialize_field("vals", &vals_vec)?;

        if let Some(children) = children {
            let children_vec = (0..*size + 1)
                .map(|i| children[i].as_ref().unwrap())
                .collect::<Vec<_>>();
            state.serialize_field("children", &children_vec)?;
        }

        state.end()
    }
}
