use rust_btree::MyBTreeMap;

fn main() {
    let mut map: MyBTreeMap<3, _, _> = MyBTreeMap::new();

    // 1~20までの値を挿入
    for i in 1..=20 {
        map.insert(i, format!("{i:0>4}"));

        eprintln!("> Insert {i}");
        map.print_as_tree();
    }

    // 奇数を削除
    for i in (1..=20).step_by(2) {
        map.remove(&i);

        eprintln!("> Remove {i}");
        map.print_as_tree();
    }
}
