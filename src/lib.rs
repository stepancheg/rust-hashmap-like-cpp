use std::collections::HashMap;
use std::hash::Hash;
use std::cell::UnsafeCell;

pub struct CppLikeHashMap<K, V> {
    map: UnsafeCell<HashMap<K, Box<V>>>
}

impl<K : Hash + Eq + Clone, V> CppLikeHashMap<K, V> {
    pub fn new() -> Self {
        CppLikeHashMap {
            map: UnsafeCell::new(HashMap::new())
        }
    }

    fn mut_map(&self) -> &mut HashMap<K, Box<V>> {
        unsafe {
            std::mem::transmute(self.map.get())
        }
    }

    pub fn insert(&self, key: K, value: Box<V>) -> &V {
        // TODO: clone
        self.mut_map().insert(key.clone(), value);
        self.get(key)
    }

    pub fn get(&self, key: K) -> &V {
        &self.mut_map()[&key]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let map = CppLikeHashMap::<String, String>::new();
        let aa: &String = map.insert("a".to_string(), Box::new("aa".to_string()));
        let bb: &String = map.insert("b".to_string(), Box::new("bb".to_string()));
        assert_eq!("aa", aa);
        assert_eq!("bb", bb);

        // compare addresses
        assert_eq!(aa as *const String, map.get("a".to_string()) as *const String);
        assert_eq!(bb as *const String, map.get("b".to_string()) as *const String);
    }
}
