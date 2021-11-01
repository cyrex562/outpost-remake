use std::collections::HashMap;
use std::ops::{Add, AddAssign, IndexMut, Index};

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Dictionary<T> {
    pub m_dictionary: HashMap<String, T>,
}

impl Dictionary<T> {
    pub fn get<T>(&self, key: &String, default_value: T) -> T {
        if self.m_dictionary.contains_key(key)
        {
            self.m_dictionary[key]
        }
        default_value
    }

    pub fn get2<T>(&self, key: &String) -> Option<T> {
        if self.m_dictionary.contains_key(k) {
            Some(self.m_dictionary[key])
        }
        None
    }

    pub fn set<T>(&mut self, key: &String, value: &T) {
        self.m_dictionary[key] = value;
    }

    pub fn erase(&mut self) {
        self.m_dictionary.clear()
    }

    pub fn has(&self, key: &String) -> bool {
        self.m_dictionary.contains_key(key)
    }

    pub fn keys(&self) -> Vec<String> {
        let mut out_vec: Vec<String> = Vec::new();
        for k in self.m_dictionary.keys() {
            out_vec.push(k.clone())
        }
        out_vec
    }

    pub fn new<T>(initial_entries: &HashMap<String, T>) -> Dictionary<T> {
        let mut out = Dictionary {
            m_dictionary: HashMap::new(),
        };
        initial_entries.clone_into(&mut out.m_dictionary);
        out
    }
}

impl Add for Dictionary<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { m_dictionary: self.m_dictionary + rhs.m_dictionary }
    }
}

impl AddAssign for Dictionary<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.m_dictionary = &self.m_dictionary + rhs.m_dictionary
    }
}

impl Index<String> for Dictionary<T> {
    type Output = T;

    fn index(&self, index: String) -> &Self::Output {
        self.m_dictionary[index]
    }
}

impl IndexMut<String> for Dictionary<T> {
    fn index_mut(&mut self, index: String) -> &mut Self::Output {
        &mut self.m_dictionary[index]
    }
}
