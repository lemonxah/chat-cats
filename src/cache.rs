use std::{time::SystemTime, fmt::Debug, sync::Arc, cmp::Ordering};

use tokio::sync::Mutex;


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Default)]
struct Keyed<K, A> where A: Clone + Debug {
    key: K,
    value: A,
}

#[derive(Debug, Clone, PartialOrd, Eq, PartialEq)]
struct TimeStamped<K, A> where A: Clone + Debug {
    timestamp: u128,
    value: Keyed<K, A>,
}

impl<K, A> Ord for TimeStamped<K, A> where A: Clone + Debug + Ord, K: Ord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl<K, A> TimeStamped<K, A> where A: Clone + Debug + Default, K: PartialEq + Default {
    pub fn empty() -> TimeStamped<K, A> {
        TimeStamped {
            timestamp: 0,
            value: Keyed {
                key: K::default(),
                value: A::default(),
            }
        }
    }

} 

#[derive(Clone, Debug)]
pub struct Cache<K, A, const N: usize> where A: Clone + Debug {
    cache: Arc<Mutex<[TimeStamped<K, A>; N]>>,
}

impl<K, A, const N: usize> Cache<K, A, N> where A: Clone + Debug + Default + Ord, K: PartialEq + Debug + Default + Ord {
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
            println!("Cache hit: {:?}", x);
            x.timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            x.value.value.clone()
        })
    }

    pub async fn set(&self, key: K, value: A) {
        let mut cache = self.cache.lock().await;
        cache.sort(); // not the best will look into why the previous oldest get didn't work
        let oldest = cache.first_mut().unwrap();
        oldest.timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
        oldest.value.key = key;
        oldest.value.value = value;
        println!("Cache set: {:?}", oldest);
    }
}