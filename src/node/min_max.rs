//! 最小値，最大値の検索

use crate::node::{node::NodePtr, util::NodeUtil};

/// 最も左の葉ノードを探索する
fn leftmost_leaf<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> &NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
{
    let mut x = node;

    while let Some(left) = x.nth_child(0) {
        x = left;
    }

    x
}

/// 最も右の葉ノードを探索する
fn rightmost_leaf<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> &NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
{
    let mut x = node;

    while let Some(right) = x.nth_child(*x.size()) {
        x = right;
    }

    x
}

/// 部分木の最小値を返す
pub fn min_key<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> Option<&K>
where
    [(); 2 * D - 1]:,
{
    leftmost_leaf(node).keys[0].as_ref()
}

/// 部分木の最大値を返す
pub fn max_key<const D: usize, K, V>(node: &NodePtr<D, K, V>) -> Option<&K>
where
    [(); 2 * D - 1]:,
{
    let rightmost = rightmost_leaf(node);
    let size = *rightmost.size();
    rightmost.keys[size - 1].as_ref()
}
