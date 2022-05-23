use std::collections::HashMap;
use std::ops::SubAssign;

pub fn sub_assign_vec<T>(a: &mut Vec<T>, b: &mut Vec<T>) {
    for b_ele in b {
        let c = a.binary_search(b_ele);
        if c.is_ok() {
            let c_idx = c.unwrap();
            a.remove(c_idx);
        }
    }
}

pub fn sub_vec<T>(a: &Vec<T>, b: &Vec<T>) -> Vec<T> {
    let mut c: Vec<T> = Vec::new();
    a.clone_into(&mut c);
    for b_ele in b {
        d = c.binary_search(b_ele);
        if d.is_ok() {
            let d_idx = d.unwrap();
            c.remove(d);
        }
    }
    c
}

pub fn map_to_vector<T, U, V>(in_map: &HashMap<T, U>, map_func: fn(&T, &U, &mut Vector<V>)) -> Vector<V> {
    let mut out_vec: Vec<V> = Vec::new();
    for (a, b) in in_map {
        map_func(a, b, &mut out_vec)
    }
    out_vec
}

pub fn missing_values<T>(values: &Vec<T>, required: &Vec<T>) -> Vec<T> {
    sub_vec(required, values)
}

pub fn unexpected_values<T>(values: &Vec<T>, required: &Vec<T>, optional: Option<&Vec<T>>) -> Vec<T> {
    let mut expected: Vec<T> = Vec::new();
    if optional.is_some() {
        expected = required + optional;
    } else {
        required.clone_into(&mut expected);
    }
    sub_vec(values, &expected)
}

pub fn get_keys<K, V>(in_map: &HashMap<K, V>) -> Vec<K> {
    let mut out_vec: Vec<K> = Vec::new();
    for k in in_map.keys() {
        out_vec.push(k.clone());
    }
    out_vec
}

pub fn merge_by_key<K, V>(defaults: &HashMap<K, V>, priority_values: &HashMap<K, V>) -> HashMap<K, V> {
    let mut results: HashMap<K, V> = HashMap::new();
    defaults.clone_into(&mut results);
    for (k, v) in priority_values {
        results[k] += v;
    }
    results
}


