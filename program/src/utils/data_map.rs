use std::collections::BTreeMap;
use borsh::{BorshSerialize, BorshDeserialize, BorshSchema};

/*
DataMap:

datamap exposes a BTreeMap, which is an implementation of a BTree. 
This provides automatic indexing on keys and ordered results.

Generic types:
K --> the key type
  --> extends Ord, which is linear order of elements. This just means that any element needs to be comparable.

V --> the value type
*/
#[derive(Debug)]
pub struct DataMap<K, V>
where 
  K: Ord + BorshSerialize + BorshDeserialize + BorshSchema,
  V: BorshSerialize + BorshDeserialize + BorshSchema
{
  pub map: BTreeMap<K, V>
}


impl<K, V> DataMap<K, V> 
where
  K: Ord + BorshSerialize + BorshDeserialize + BorshSchema,
  V: BorshSerialize + BorshDeserialize + BorshSchema
{
  pub fn new() -> Self { DataMap{ map: BTreeMap::new() } }

  pub fn put(&mut self, key: K, val: V) -> Option<V> {
    self.map.insert(key, val)
  }

  pub fn del(&mut self, key: K) -> Option<V> {
    self.map.remove(&key)
  }

  pub fn get(&self, key: K) -> Option<&V> {
    self.map.get(&key)
  }

  pub fn iter(&self) -> std::collections::btree_map::Iter<K, V> {
    self.map.iter()
  }
}


impl<K, V> BorshSerialize for DataMap<K, V> 
where
  K: Ord + BorshSerialize + BorshDeserialize + BorshSchema,
  V: BorshSerialize + BorshDeserialize + BorshSchema
{
  fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
    let len = self.map.len() as u32;
    len.serialize(writer)?;

    for (key, value) in &self.map {
      key.serialize(writer)?;
      value.serialize(writer)?;
    }

    Ok(())
  }
}


impl<K, V> BorshDeserialize for DataMap<K, V> 
where
  K: Ord + BorshSerialize + BorshDeserialize + BorshSchema,
  V: BorshSerialize + BorshDeserialize + BorshSchema
{
  fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
    let len = u32::deserialize(buf)? as usize;
    let mut map = BTreeMap::new();
    
    for _ in 0..len {
      let key = K::deserialize(buf)?;
      let value = V::deserialize(buf)?;
      map.insert(key, value);
    }

    Ok(DataMap { map })
  }
}