//! mapの構造体

use std::fmt::Debug;

use crate::{
    node::{
        insert::insert_multi,
        node::NodePtr,
        remove::{remove, RemoveKey},
        search::{get, get_mut},
    },
    util::tree_debug::print_as_tree,
};

/// B木による連想配列
pub struct MyBTreeMap<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    /// ルートノード
    root: Option<NodePtr<D, K, V>>,
    /// 要素数
    size: usize,
}

impl<const D: usize, K, V> MyBTreeMap<D, K, V>
where
    [(); 2 * D - 1]:,
    K: Ord,
{
    /// 新規作成する
    ///
    /// **戻り値**
    /// - Self: 作成したマップ
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }

    /// 値を挿入する
    ///
    /// **引数**
    /// - `key`: 挿入するキー
    /// - `value`: 挿入する値
    pub fn insert(&mut self, key: K, value: V) {
        // 挿入
        self.root = insert_multi(self.root.take(), key, value);
        // 要素数を増やす
        self.size += 1;
    }

    /// 値を削除する
    ///
    /// **引数**
    /// - `key`: 削除するキー
    ///
    /// **戻り値**
    /// - Option<(K, V)>: 削除したキーと値
    pub fn remove(&mut self, key: &K) -> Option<(K, V)> {
        let removed;

        // 削除
        (self.root, removed) = remove(self.root.take(), RemoveKey::Key(key));

        if removed.is_some() {
            // 要素数を減らす
            self.size -= 1;
        }

        removed
    }

    /// 値を取得する
    ///
    /// **引数**
    /// - `key`: 取得するキー
    ///
    /// **戻り値**
    /// - Option<&V>: キーに対応する値
    pub fn get(&self, key: &K) -> Option<&V> {
        get(&self.root, key)
    }

    /// 値の可変参照を取得する
    ///
    /// **引数**
    /// - `key`: 取得するキー
    ///
    /// **戻り値**
    /// - Option<&mut V>: キーに対応する値
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        get_mut(&mut self.root, key)
    }

    /// 要素数を取得する
    ///
    /// **戻り値**
    /// - usize: 要素数
    pub fn len(&self) -> usize {
        self.size
    }

    /// 木の形式で表示する
    pub fn print_as_tree(&self)
    where
        K: Debug,
        V: Debug,
    {
        print_as_tree(&self.root);
    }
}
