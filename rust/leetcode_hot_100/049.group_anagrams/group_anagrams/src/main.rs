use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut groups = HashMap::new();
        for s in strs {
            // 新建副本，转字符切片
            let mut sort_s = s.clone().into_bytes();
            // 切片排序
            sort_s.sort_unstable();
            // 这个方法通过键来访问 HashMap 中的条目（entry），
            // 如果键不存在，则插入一个新的条目。
            // 这种模式特别适合于当你需要更新 HashMap 中的元素或者统计频次时。
            groups.entry(sort_s).or_insert(vec![]).push(s);

        }
        // 获取 HashMap 的所有值，并将这些值转换为一个迭代器,
        // 转移所有权。调用 .into_values() 方法后，原始的 HashMap 将不再可用
        // collect这是一个泛型方法，根据期望的输出类型，将迭代器中的元素收集到相应的集合中。它是类型推断友好的，通常可以
        groups.into_values().collect()
    }
}

struct Solution;

fn main() {
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string()
        ];
    let result = Solution::group_anagrams(strs);
    println!("result: {:?}", result);
}
