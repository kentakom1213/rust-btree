use rust_btree::collections::map::MyBTreeMap;

fn main() {
    let mut map: MyBTreeMap<3, _, _> = MyBTreeMap::new();

    // 1~20までの値を挿入
    for i in 1..=20 {
        map.insert(i, format!("{i:0>4}"));

        println!("> Insert {i}");
        map.print_as_tree();
    }

    // 奇数を削除
    for i in (1..=20).step_by(2) {
        map.remove(&i);

        println!("> Remove {i}");
        map.print_as_tree();
    }
}
