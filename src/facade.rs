use crate::multiset;
use crate::setidx;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter;
use std::path::PathBuf;
use std::vec::Vec;

struct MultiSetGenerator<'a> {
    files: &'a [PathBuf],
    separator: char,
    use_empty: bool,
}

impl<'a> MultiSetGenerator<'a> {
    fn new(files: &'a [PathBuf], separator: char, use_empty: bool) -> MultiSetGenerator {
        MultiSetGenerator {
            files,
            separator,
            use_empty,
        }
    }
    fn generate(&self) -> io::Result<multiset::MultiSet<String>> {
        if self.files.is_empty() {
            self.from_stdin()
        } else {
            self.from_files()
        }
    }
    fn from_stdin(&self) -> io::Result<multiset::MultiSet<String>> {
        let mut s: multiset::MultiSet<String> = Default::default();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let l = line?;
            let p = l.split(self.separator).enumerate();
            for (i, x) in p {
                // 空文字列を無視する
                if !self.use_empty && x.is_empty() {
                    continue;
                }
                s.insert(i, x.to_owned());
            }
        }
        Ok(s)
    }
    fn from_files(&self) -> io::Result<multiset::MultiSet<String>> {
        let mut s: multiset::MultiSet<String> = Default::default();
        for (i, f) in self.files.iter().enumerate() {
            let file = File::open(f)?;
            let r = io::BufReader::new(file);
            for line in r.lines() {
                let l = line?;
                // 空文字列を無視する
                if !self.use_empty && l.is_empty() {
                    continue;
                }
                s.insert(i, l.to_owned());
            }
        }
        Ok(s)
    }
}

struct HeaderGenerator<'a> {
    indexes: &'a Vec<setidx::Idx>,
    size: usize,
    use_decimal_index: bool,
}

impl<'a> HeaderGenerator<'a> {
    fn new(indexes: &'a Vec<setidx::Idx>, size: usize, use_decimal_index: bool) -> HeaderGenerator {
        HeaderGenerator {
            indexes,
            size,
            use_decimal_index,
        }
    }
    fn generate(&self) -> String {
        self.indexes
            .iter()
            .map(|x| {
                let i = x.to_bidx(self.size);
                if self.use_decimal_index {
                    format!("{}", i.to_decimal())
                } else {
                    i.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

struct ElementStringer<'a> {
    use_sort: bool,
    indexes: &'a Vec<setidx::Idx>,
    ms: &'a multiset::MultiSet<String>,
}

impl<'a> ElementStringer<'a> {
    fn new(
        indexes: &'a Vec<setidx::Idx>,
        ms: &'a multiset::MultiSet<String>,
        use_sort: bool,
    ) -> ElementStringer<'a> {
        ElementStringer {
            use_sort,
            indexes,
            ms,
        }
    }
}

impl<'a> iter::IntoIterator for ElementStringer<'a> {
    type Item = String;
    type IntoIter = ElementStringerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        let f = || -> Box<dyn Iterator<Item = multiset::MultiSetElement<String>>> {
            let it = self.ms.clone().element_iter(self.indexes.clone());
            if self.use_sort {
                let mut v: Vec<_> = it.collect();
                v.sort_by_key(|a| a.element().clone());
                Box::new(v.into_iter())
            } else {
                Box::new(it)
            }
        };
        ElementStringerIntoIterator {
            it: f(),
            indexes: self.indexes.clone(),
        }
    }
}

struct ElementStringerIntoIterator {
    it: Box<dyn Iterator<Item = multiset::MultiSetElement<String>>>,
    indexes: Vec<setidx::Idx>,
}

impl Iterator for ElementStringerIntoIterator {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        self.it.next().map(|i| {
            let e = i.element();
            let r = self
                .indexes
                .iter()
                .map(|x| if i.indexes().contains(x) { "1" } else { "0" })
                .collect::<Vec<_>>()
                .join(" ");
            format!("{} {}", e, r)
        })
    }
}

pub struct Executor {
    files: Vec<PathBuf>,
    use_decimal_index: bool,
    depth_begin: Option<usize>,
    depth_end: Option<usize>,
    separator: char,
    use_empty: bool,
    use_sort: bool,
}

impl Executor {
    pub fn new(
        files: Vec<PathBuf>,
        use_decimal_index: bool,
        depth_begin: Option<usize>,
        depth_end: Option<usize>,
        separator: char,
        use_empty: bool,
        use_sort: bool,
    ) -> Executor {
        Executor {
            files,
            use_decimal_index,
            depth_begin,
            depth_end,
            separator,
            use_empty,
            use_sort,
        }
    }
    pub fn execute(&self) {
        let msg = MultiSetGenerator::new(&self.files, self.separator, self.use_empty);
        let ms = msg.generate().unwrap();
        let indexes: Vec<setidx::Idx> =
            setidx::Indexes::new(ms.len(), self.depth_begin, self.depth_end).collect();
        let hg = HeaderGenerator::new(&indexes, ms.len(), self.use_decimal_index);
        let h = hg.generate();
        println!("u {}", h);
        let es = ElementStringer::new(&indexes, &ms, self.use_sort);
        es.into_iter().for_each(|x| println!("{}", x));
    }
}
