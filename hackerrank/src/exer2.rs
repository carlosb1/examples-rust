fn countBetween(arr: &[i32], low: &[i32], high: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut index_low = 0;
    for value_low in low {
        if index_low >= high.len() {
            continue;
        }
        let value_high: i32 = high[index_low];
        let index_val_low: i32 = *value_low;
        let index_val_high: i32 = value_high;

        let mut count: i32 = 0;
        for val in arr {
            if *val >= index_val_low && *val <= index_val_high {
                count += 1;
            }
        }
        //let values: Vec<&i32> = arr
        //    .iter()
        //    .filter(|&val| *val >= index_val_low && *val <= index_val_high)
        //    .collect();
        result.push(count);
        index_low += 1;
    }
    result
}

fn main() {
    let arr2 = [1, 3, 5, 6, 8];
    let low2 = [2];
    let high2 = [6];

    let arr = [4, 8, 7];
    let low = [2, 4];
    let high = [8, 4];

    let values = countBetween(&arr, &low, &high);
    println!("{:?}", values);
    let values = countBetween(&arr2, &low2, &high2);
    println!("{:?}", values);
    println!("exer2");
}
