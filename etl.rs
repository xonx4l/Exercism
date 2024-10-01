use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char , i32> {
    let mut output = BtreeMap::new();
    for (a,b) in input {
       for c in b {
            output.insert(c.to_ascii_lowercase(), *a);
        }
     }
    output 
}
