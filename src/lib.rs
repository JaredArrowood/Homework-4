use std::collections::HashMap;

/// Returns the first `n` Fibonacci numbers.
pub fn fib(_n: u32) -> Vec<u32> {
    let mut vec = Vec::new();
    if(_n == 0)
    {
        return vec;
    }
    vec.push(0);
    if(_n == 1)
    {
        return vec;
    }
    vec.push(1);
    for i in 2.._n
    {
        let next = vec[(i - 1) as usize] + vec[(i - 2) as usize];
        vec.push(next);
    }
    return vec;
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(_n: u32) -> bool {
    let string = _n.to_string();
    let rev_string: String = string.chars().rev().collect();
    if(string == rev_string)
    {
        return true;
    }
    return false;
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(_n: usize, _a: &[i32]) -> Option<i32> {
    let mut vec = _a.to_vec();
    vec.sort();
    vec.dedup();
    if(_n > vec.len())
    {
        return None;
    }
    return Some(vec[vec.len() - _n]);
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(_s: &str) -> String {
    
    let mut map = HashMap::new();
    for c in _s.chars()
    {
        let count = map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut max_char = ' ';
    let mut max_count = 0;
    for (c, count) in map.iter()
    {
        if(*count > max_count)
        {
            max_count = *count;
            max_char = *c;
        }
    }
    return max_char.to_string();
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    _arr1: &[String],
    _arr2: &[String],
) -> Option<HashMap<String, String>> {
    
    if(_arr1.len() != _arr2.len())
    {
        return None;
    }
    let mut map = HashMap::new();
    for i in 0.._arr1.len()
    {
        map.insert(_arr1[i].clone(), _arr2[i].clone());
    }
    return Some(map);
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    _map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    let mut vec = Vec::new();
    for (k, v) in _map.iter()
    {
        vec.push((k.clone(), v.clone()));
    }
    return vec;
}

// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    // You are free to change this internal representation if you want.
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        // You may also use `Self::default()`
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(
        &mut self,
        _name: String,
        _number: String,
        _is_listed: bool,
    ) -> bool {
        if(self.entries.iter().any(|entry| entry.name == _name))
        {
            return false;
        }
        let parts: Vec<&str> = _number.split('-').collect();
        if(parts.len() != 3 || parts[0].len() != 3 || parts[1].len() != 3 || parts[2].len() != 4 ||
           !parts[0].chars().all(|c| c.is_digit(10)) ||
           !parts[1].chars().all(|c| c.is_digit(10)) ||
           !parts[2].chars().all(|c| c.is_digit(10)))
        {
            return false;
        }
        if(_is_listed && self.entries.iter().any(|entry| entry.number == _number && entry.is_listed))
        {
            return false;
        }
        self.entries.push(PhoneEntry {
            name: _name,
            number: _number,
            is_listed: _is_listed,
        });
        return true;
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, _name: &str) -> Option<String> {
        
        for entry in self.entries.iter()
        {
            if(entry.name == _name && entry.is_listed)
            {
                return Some(entry.number.clone());
            }
        }
        return None;
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, _num: &str) -> Option<String> {
        for entry in self.entries.iter()
        {
            if entry.number == _num && entry.is_listed
            {
                return Some(entry.name.clone());
            }
        }
        return None;
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, _areacode: &str) -> Vec<String> {
        for entry in self.entries.iter()
        {
            if entry.number.starts_with(_areacode)
            {
                return self.entries.iter().filter(|e| e.number.starts_with(_areacode)).map(|e| e.name.clone()).collect();
            }
        }
        return Vec::new();
    }
}