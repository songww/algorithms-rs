use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

struct Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut map = HashMap::new();
        let mut foods = HashSet::new();
        for order in orders {
            *map.entry(order[1].to_string())
                .or_insert(HashMap::new())
                .entry(order[2].to_string())
                .or_insert(0) += 1;
            foods.insert(order[2].to_string());
        }
        let mut tables: Vec<&String> = map.keys().collect();
        tables.sort_by_cached_key(|k| u16::from_str(k).unwrap());
        let mut foods: Vec<String> = foods.into_iter().collect();
        foods.sort_unstable();
        let mut ans = Vec::with_capacity(tables.len());
        let mut headers = Vec::with_capacity(foods.len());
        headers.push("Table".to_string());
        headers.extend_from_slice(&foods);
        ans.push(headers);
        for table in tables {
            let mut row = Vec::with_capacity(foods.len());
            row.push(table.to_owned());
            for food in &foods {
                row.push(map.get(table).unwrap().get(food).unwrap_or(&0).to_string());
            }
            ans.push(row);
        }
        ans
    }
}

fn main() {
    assert_eq!(
        Solution::display_table(vec![
            vec![
                "Laura".to_string(),
                "2".to_string(),
                "Bean Burrito".to_string()
            ],
            vec![
                "Jhon".to_string(),
                "2".to_string(),
                "Beef Burrito".to_string()
            ],
            vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()]
        ]),
        vec![
            vec![
                "Table".to_string(),
                "Bean Burrito".to_string(),
                "Beef Burrito".to_string(),
                "Soda".to_string()
            ],
            vec![
                "2".to_string(),
                "1".to_string(),
                "1".to_string(),
                "1".to_string()
            ]
        ]
    );
}
