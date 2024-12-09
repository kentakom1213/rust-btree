//! ノードのユーティリティ

use crate::node::node::NodePtr;

pub trait NodeUtil<const D: usize, K, V>
where
    [(); 2 * D - 1]:,
{
    /// ノードの要素数を取得する
    fn size(&self) -> &usize;
    /// ノードの要素数への可変参照を取得する
    fn size_mut(&mut self) -> &mut usize;
    /// n番目のキーへの不変参照を取得する
    fn nth_key(&self, n: usize) -> Option<&K>;
    /// n番目の値への不変参照を取得する
    fn nth_val(&self, n: usize) -> Option<&V>;
    /// n番目の値への可変参照を取得する
    fn nth_val_mut(&mut self, n: usize) -> Option<&mut V>;
    /// n番目の子ノードへの不変参照を取得する
    fn nth_child(&self, n: usize) -> Option<&NodePtr<D, K, V>>;
    /// n番目の子ノードへの可変参照を取得する
    fn nth_child_mut(&mut self, n: usize) -> Option<&mut NodePtr<D, K, V>>;
    /// 葉ノードか判定する
    fn is_leaf(&self) -> bool;
    /// 空きが存在するか判定
    fn is_full(&self) -> bool {
        *self.size() == 2 * D - 1
    }
}

macro_rules! impl_get_ref {
    ($name:ident, $field:ident, $return:ty) => {
        fn $name(&self) -> $return {
            &self.$field
        }
    };
    ($name:ident, $field:ident, $return:ty, mut) => {
        fn $name(&mut self) -> $return {
            &mut self.$field
        }
    };
}

impl<const D: usize, K, V> NodeUtil<D, K, V> for NodePtr<D, K, V>
where
    [(); 2 * D - 1]:,
    [(); 2 * D]:,
{
    impl_get_ref!(size, size, &usize);
    impl_get_ref!(size_mut, size, &mut usize, mut);

    fn nth_key(&self, n: usize) -> Option<&K> {
        self.keys.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_val(&self, n: usize) -> Option<&V> {
        self.vals.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_val_mut(&mut self, n: usize) -> Option<&mut V> {
        self.vals.get_mut(n).map(|x| x.as_mut()).flatten()
    }
    fn nth_child(&self, n: usize) -> Option<&NodePtr<D, K, V>> {
        self.children.as_ref()?.get(n).map(|x| x.as_ref()).flatten()
    }
    fn nth_child_mut(&mut self, n: usize) -> Option<&mut NodePtr<D, K, V>> {
        self.children
            .as_mut()?
            .get_mut(n)
            .map(|x| x.as_mut())
            .flatten()
    }

    fn is_leaf(&self) -> bool {
        self.children.is_none()
    }
}
