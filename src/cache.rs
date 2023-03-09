use std::{time::SystemTime, fmt::Debug, sync::Arc};

use tokio::sync::Mutex;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct Keyed<K, A> where A: Clone + Default + Debug {
    key: K,
    value: A,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct TimeStamped<K, A> where A: Clone + Default + Debug {
    timestamp: u128,
    value: Keyed<K, A>,
}

impl<K, A> TimeStamped<K, A> where A: Clone + Default + Debug, K: PartialEq + Default {
    pub fn empty() -> TimeStamped<K, A> {
        TimeStamped {
            timestamp: 0,
            value: Keyed::default(),
        }
    }

} 

#[derive(Clone)]
pub struct Cache<K, A, const N: usize> where A: Clone + Default + Debug {
    cache: Arc<Mutex<[TimeStamped<K, A>; N]>>,
}

impl<K, A, const N: usize> Cache<K, A, N> where A: Clone + Default + Debug, K: PartialEq + Debug + Default {
    pub fn new() -> Cache<K, A, N> {
        let mut cache = vec![];
        for _ in 0..N {
            cache.push(TimeStamped::empty());
        }
        let cache: [TimeStamped<K, A>; N] = cache.try_into().unwrap();
        Cache {
            cache: Arc::new(Mutex::new(cache)),
        }
    }

    pub async fn get(&self, key: K) -> Option<A> {
        let mut cache = self.cache.lock().await;
        cache.iter_mut().find(|x| x.value.key == key).map(|x| {
            x.timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            x.value.value.clone()
        })
    }

    pub async fn set(&self, key: K, value: A) {
        let mut cache = self.cache.lock().await;
        let mut oldest = &mut TimeStamped::empty();
        for x in cache.iter_mut() {
            if x.timestamp < oldest.timestamp {
                oldest = x;
            }
        }
        if oldest.timestamp == 0 {
            oldest = &mut cache[0];
        }
        oldest.timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        oldest.value.key = key;
        oldest.value.value = value;
    }
}