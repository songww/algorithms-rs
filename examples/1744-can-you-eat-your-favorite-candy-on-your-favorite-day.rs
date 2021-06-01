struct Solution;

impl Solution {
    pub fn can_eat(candies_count: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut answer = Vec::with_capacity(queries.len());
        let mut prefix = Vec::with_capacity(candies_count.len());
        prefix.push(1);
        let _ = candies_count.iter().fold(1, |left, right| {
            let sum = left + *right as usize;
            prefix.push(sum);
            sum
        });
        for query in queries.iter() {
            let favorite_type = query[0] as usize;
            let favorite_day = query[1] as usize + 1;
            let daily_cap = query[2] as usize;
            let maximum = favorite_day * daily_cap;
            let minimum = favorite_day;
            let least = prefix[favorite_type];
            let most = prefix[favorite_type + 1];
            println!("{:?}", query);
            let can = maximum > least && minimum < most;
            println!(
                "least {} most {} min {} max {} -> {}",
                least, most, minimum, maximum, can
            );
            answer.push(can);
        }
        answer
    }
}

fn main() {
    assert_eq!(
        Solution::can_eat(
            vec![7, 4, 5, 3, 8],
            vec![vec![0, 2, 2], vec![4, 2, 4], vec![2, 13, 1000000000]]
        ),
        vec![true, false, true]
    );
    assert_eq!(
        Solution::can_eat(
            vec![5, 2, 6, 4, 1],
            vec![
                vec![3, 1, 2],
                vec![4, 10, 3],
                vec![3, 10, 100],
                vec![4, 100, 30],
                vec![1, 3, 1]
            ]
        ),
        vec![false, true, true, false, false]
    );
    assert_eq!(
        Solution::can_eat(
            vec![
                16, 38, 8, 41, 30, 31, 14, 45, 3, 2, 24, 23, 38, 30, 31, 17, 35, 4, 9, 42, 28, 18,
                37, 18, 14, 46, 11, 13, 19, 3, 5, 39, 24, 48, 20, 29, 4, 19, 36, 11, 28, 49, 38,
                16, 23, 24, 4, 22, 29, 35, 45, 38, 37, 40, 2, 37, 8, 41, 33, 8, 40, 27, 13, 4, 33,
                5, 8, 14, 19, 35, 31, 8, 8
            ],
            vec![
                vec![35, 669, 5],
                vec![72, 822, 74],
                vec![47, 933, 94],
                vec![62, 942, 85],
                vec![42, 596, 11],
                vec![56, 1066, 18],
                vec![54, 571, 45],
                vec![39, 890, 100],
                vec![3, 175, 26],
                vec![48, 1489, 37],
                vec![40, 447, 52],
                vec![30, 584, 7],
                vec![26, 1486, 38],
                vec![21, 1142, 21],
                vec![9, 494, 96],
                vec![56, 759, 81],
                vec![13, 319, 16],
                vec![20, 1406, 57],
                vec![11, 1092, 19],
                vec![24, 670, 67],
                vec![38, 1702, 33],
                vec![5, 676, 32],
                vec![50, 1386, 77],
                vec![36, 1551, 87],
                vec![29, 1445, 13],
                vec![58, 977, 13],
                vec![7, 887, 64],
                vec![37, 1396, 23],
                vec![0, 765, 69],
                vec![40, 1083, 86],
                vec![43, 1054, 49],
                vec![48, 690, 92],
                vec![28, 1201, 56],
                vec![47, 948, 43],
                vec![57, 233, 25],
                vec![32, 1293, 65],
                vec![0, 1646, 34],
                vec![43, 1467, 39],
                vec![39, 484, 23],
                vec![21, 1576, 69],
                vec![12, 1222, 68],
                vec![9, 457, 83],
                vec![32, 65, 9],
                vec![10, 1424, 42],
                vec![35, 534, 3],
                vec![23, 83, 22],
                vec![33, 501, 33],
                vec![25, 679, 51],
                vec![2, 321, 42],
                vec![1, 240, 68],
                vec![7, 1297, 42],
                vec![45, 480, 72],
                vec![26, 1472, 9],
                vec![6, 649, 90],
                vec![26, 361, 57],
                vec![49, 1592, 7],
                vec![11, 158, 95],
                vec![35, 448, 24],
                vec![41, 1654, 10],
                vec![61, 510, 43],
                vec![31, 1230, 95],
                vec![11, 1471, 12],
                vec![37, 43, 84],
                vec![56, 1147, 48],
                vec![69, 1368, 65],
                vec![22, 170, 24],
                vec![56, 192, 80],
                vec![34, 1207, 69],
                vec![1, 1226, 22],
                vec![37, 1633, 50],
                vec![11, 98, 58],
                vec![17, 125, 13],
                vec![0, 1490, 5],
                vec![37, 1732, 43],
                vec![45, 793, 14],
                vec![16, 578, 72],
                vec![50, 241, 78]
            ]
        ),
        vec![
            true, true, true, true, true, true, true, true, false, false, true, true, false, false,
            false, true, true, false, false, false, false, false, false, false, false, true, false,
            false, false, false, false, true, false, true, true, false, false, false, true, false,
            false, false, false, false, true, true, true, false, false, false, false, true, false,
            false, true, false, true, true, false, true, false, false, true, true, true, true,
            true, false, false, false, true, true, false, false, true, false, true
        ]
    );
}
