use crate::setidx::Idx;
use std::clone::Clone;
use std::cmp::{Eq, Ord, PartialEq};
use std::collections::HashSet;
use std::convert;
use std::default::Default;
use std::hash::Hash;
use std::iter::{Iterator, IntoIterator};
use std::ops;
use std::vec::Vec;

/// `HashSet` のリスト.
#[derive(Clone)]
pub struct MultiSet<T>(Vec<HashSet<T>>);

impl<T> MultiSet<T>
where
    T: Eq + PartialEq + Ord + Hash + Clone,
{
    /// すべての `HashSet` の和集合を返す.
    pub fn union(&self) -> HashSet<T> {
        let mut u = HashSet::new();
        for v in self.0.iter() {
            for x in v.iter() {
                u.insert(x.clone());
            }
        }
        u
    }

    /// 選択した `HashSet` すべてに渡した要素が含まれていれば `true` を返す.
    ///
    /// # Arguments
    ///
    /// * `value` - 要素
    /// * `indexes` - インデックスのリスト
    pub fn contains_with_indexes(&self, value: &T, indexes: &Idx) -> bool {
        if indexes.is_empty() {
            return false;
        }
        (0..indexes.len()).all(|i| {
            let x = indexes[i];
            x < self.0.len() && self.0[x].contains(value)
        })
    }

    fn grow(&mut self, n: usize) {
        let d = n as u32 as i64 - self.0.len() as u32 as i64;
        for _ in 0..d {
            self.0.push(HashSet::new());
        }
    }

    /// 要素を加える.
    ///
    /// # Arguments
    ///
    /// * `i` - インデックス
    /// * `value` - 要素
    ///
    /// インデックス `i` で `insert()` を呼んだなら `len()` は最低でも `i` となる.
    pub fn insert(&mut self, i: usize, value: T) -> bool {
        self.grow(i + 1);
        self.0[i].insert(value)
    }

    /// リストの大きさを返す.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// リストが空ならば `true` を返す.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// すべての `HashSet` の要素数の和を返す.
    pub fn cardinality(&self) -> usize {
        self.0.iter().map(|s| s.len()).sum()
    }

    /// `MultiSetIterator` に変換する.
    ///
    /// # Arguments
    ///
    /// * `indexes` - 調査対象の集合の組み合わせのリスト
    pub fn element_iter(self, indexes: Vec<Idx>) -> MultiSetIterator<T> {
        MultiSetIterator::new(self, indexes)
    }
}

impl<T> Default for MultiSet<T>
where
    T: Clone + Hash + Ord,
{
    fn default() -> Self {
        MultiSet(Vec::new())
    }
}

impl<T> ops::Index<usize> for MultiSet<T> {
    type Output = HashSet<T>;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> convert::From<MultiSet<T>> for Vec<HashSet<T>> {
    fn from(item: MultiSet<T>) -> Self {
        item.0
    }
}

/// `MultiSetIterator` のイテレーションの要素.
pub struct MultiSetElement<T>(T, HashSet<Idx>);

impl<T> MultiSetElement<T> {
    /// 集合の要素.
    pub fn element(&self) -> &T {
        &self.0
    }
    /// どの集合に属しているか.
    pub fn indexes(&self) -> &HashSet<Idx> {
        &self.1
    }
}

/// `MultiSet` に属する要素が、リストの `HashSet` にどのように属しているのかを表現する.
pub struct MultiSetIterator<T> {
    s: MultiSet<T>,
    u: Vec<T>,
    indexes: Vec<Idx>,
    i: usize,
}

impl<T> MultiSetIterator<T>
where
    T: Hash + Eq + Clone + Ord,
{
    fn new(s: MultiSet<T>, indexes: Vec<Idx>) -> MultiSetIterator<T> {
        let u: Vec<T> = s.union().into_iter().collect();
        MultiSetIterator {
            s,
            indexes,
            u,
            i: 0,
        }
    }
}

impl<T> Iterator for MultiSetIterator<T>
where
    T: Clone + Eq + PartialEq + Ord + Hash,
{
    type Item = MultiSetElement<T>;

    fn next(&mut self) -> Option<MultiSetElement<T>> {
        if self.i >= self.u.len() {
            return None;
        }
        let u = &self.u[self.i];
        self.i += 1;
        let v: HashSet<Idx> = self
            .indexes
            .iter()
            .filter(|i| self.s.contains_with_indexes(u, i))
            .cloned()
            .collect();
        Some(MultiSetElement(u.clone(), v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiset() {
        let mut ms: MultiSet<String> = Default::default();
        ms.insert(0, "stella".to_owned());
        ms.insert(0, "luna".to_owned());
        ms.insert(1, "luna".to_owned());
        ms.insert(1, "sun".to_owned());

        let indexes: Vec<Idx> = vec![vec![0usize], vec![1usize], vec![0usize, 1usize]]
            .into_iter()
            .map(Idx::from)
            .collect();

        let msi = ms.element_iter(indexes);
        let mut got = msi.collect::<Vec<_>>();
        got.sort_by_key(|a| a.element().clone());
        let want = vec![
            (
                "luna",
                vec![vec![0usize], vec![1usize], vec![0usize, 1usize]],
            ),
            ("stella", vec![vec![0usize]]),
            ("sun", vec![vec![1usize]]),
        ];

        assert_eq!(got.len(), want.len());
        for (i, g) in got.iter().enumerate() {
            let w = &want[i];
            assert_eq!(g.element(), w.0);
            let wi: HashSet<Idx> = w.1.iter().map(|i| Idx::from(i.clone())).collect();
            assert_eq!(*g.indexes(), wi);
        }
    }
}
