use itertools::Itertools;
use std::clone::Clone;
use std::cmp;
use std::collections::HashSet;
use std::convert;
use std::hash::Hash;
use std::iter::Iterator;
use std::ops;
use std::string::ToString;
use std::vec::Vec;

/// 組み合わせを表す.
#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct Idx(Vec<usize>);
/// `Idx` をビット列のように表現する.
#[derive(Eq, PartialEq, Debug)]
pub struct BIdx(Vec<bool>);

impl Idx {
    /// `BIdx` に変換する.
    ///
    /// # Arguments
    ///
    /// * `n` - `BIdx` の幅
    ///
    /// # Example
    ///
    /// ```
    /// use multicover::setidx;
    /// let i = setidx::Idx::new(vec![1, 2]);
    /// let b = setidx::BIdx::new(vec![false, true, true, false]);
    /// assert_eq!(i.to_bidx(4), b)
    /// ```
    pub fn to_bidx(&self, n: usize) -> BIdx {
        let s: HashSet<usize> = self.0.clone().into_iter().collect();
        let xs: Vec<usize> = (0..n).collect();
        let vs: Vec<bool> = xs.iter().map(|x| s.contains(x)).collect();
        BIdx(vs)
    }
    /// 新しい `Idx` を作る.
    pub fn new(v: Vec<usize>) -> Idx {
        Idx(v)
    }
    /// インデックスの大きさを返す.
    ///
    /// # Example
    ///
    /// ```
    /// use multicover::setidx::Idx;
    /// assert_eq!(3, Idx::new(vec![1usize, 2usize, 4usize]).len());
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// インデックスの大きさが 0 の場合 `true` を返す.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl convert::From<Idx> for Vec<usize> {
    fn from(item: Idx) -> Self {
        item.0
    }
}

impl ops::Index<usize> for Idx {
    type Output = usize;

    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl ToString for BIdx {
    /// `BIdx` のコンパクトな文字列表現.
    ///
    /// # Example
    ///
    /// ```
    /// use multicover::setidx::BIdx;
    /// let b = BIdx::new(vec![false, true, true, false]);
    /// assert_eq!(b.to_string(), "0110");
    /// ```
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| if *x { 1 } else { 0 })
            .fold("".to_owned(), |acc, x| format!("{}{}", acc, x))
    }
}

impl BIdx {
    /// 新しい `BIdx` を作る.
    pub fn new(v: Vec<bool>) -> BIdx {
        BIdx(v)
    }
}

impl BIdx {
    /// `BIdx` を2進数として解釈し、10進数で表現する.
    ///
    /// # Example
    ///
    /// ```
    /// use multicover::setidx::BIdx;
    /// let v = BIdx::new(vec![false, true, true, false]);
    /// assert_eq!(v.to_decimal(), 6);
    /// ```
    pub fn to_decimal(&self) -> usize {
        let n = self.0.len() as u32;
        self.0.iter().enumerate().fold(0usize, |acc, x| {
            acc + if *x.1 {
                2u64.pow(n - 1 - x.0 as u32) as usize
            } else {
                0
            }
        })
    }
}

impl convert::From<Vec<usize>> for Idx {
    fn from(item: Vec<usize>) -> Self {
        Idx(item)
    }
}

struct Combinations {
    it: Box<dyn Iterator<Item = Vec<usize>>>,
}

impl Combinations {
    fn new(n: usize, r: usize) -> Combinations {
        Combinations {
            it: Box::new((0..n).combinations(r)),
        }
    }
}

impl Iterator for Combinations {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Vec<usize>> {
        self.it.next()
    }
}

/// 組み合わせを `Idx` で返す.
pub struct Indexes {
    // 全体の要素数.
    n: usize,
    idxit: Box<dyn Iterator<Item = usize>>,
    it: Option<Box<dyn Iterator<Item = Vec<usize>>>>,
}

impl Indexes {
    /// 新しい `Indexes` を作る.
    ///
    /// # Arguments
    ///
    /// * `size` - 全体の要素数
    /// * `depth_begin` - 何個とる組み合わせから考慮するか. `None` ならば 1 から考慮する
    /// * `depth_end` - 何個とる組み合わせまで考慮するか. `None` ならば `size` まで考慮する
    pub fn new(size: usize, depth_begin: Option<usize>, depth_end: Option<usize>) -> Indexes {
        let dbgn = depth_begin.or(Some(1)).unwrap();
        let dend = if let Some(d) = depth_end {
            cmp::min(size, d)
        } else {
            size
        };
        let idxit: Vec<usize> = (dbgn..=dend).collect();
        Indexes {
            n: size,
            idxit: Box::new(idxit.into_iter()),
            it: None,
        }
    }
}

impl Iterator for Indexes {
    type Item = Idx;

    fn next(&mut self) -> Option<Idx> {
        if let Some(xit) = &mut self.it {
            if let Some(x) = xit.next() {
                return Some(Idx::from(x));
            }
        }
        if let Some(idx) = self.idxit.next() {
            self.it = Some(Box::new(Combinations::new(self.n, idx)));
            self.next()
        } else {
            self.it = None;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations() {
        assert_eq!(
            Combinations::new(4, 3).collect::<Vec<_>>(),
            vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3],]
        )
    }

    fn to_usize_vec(v: Vec<Vec<i32>>) -> Vec<Idx> {
        v.into_iter()
            .map(|v| Idx::from(v.into_iter().map(|x| x as usize).collect::<Vec<usize>>()))
            .collect()
    }

    #[test]
    fn test_indexes() {
        assert_eq!(
            Indexes::new(4, None, None).collect::<Vec<_>>(),
            to_usize_vec(vec![
                vec![0],
                vec![1],
                vec![2],
                vec![3],
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![0, 1, 2],
                vec![0, 1, 3],
                vec![0, 2, 3],
                vec![1, 2, 3],
                vec![0, 1, 2, 3],
            ])
        );

        assert_eq!(
            Indexes::new(4, None, Some(2)).collect::<Vec<_>>(),
            to_usize_vec(vec![
                vec![0],
                vec![1],
                vec![2],
                vec![3],
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ])
        );

        assert_eq!(
            Indexes::new(4, Some(2), Some(2)).collect::<Vec<_>>(),
            to_usize_vec(vec![
                vec![0, 1],
                vec![0, 2],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
            ])
        )
    }
}
