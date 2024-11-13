extern crate alloc;
use alloc::GlobalAllocator;

pub struct HashMap<K, V, S = DefaultHashBuilder, A: GlobalAllocator> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}