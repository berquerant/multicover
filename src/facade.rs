use crate::multiset;
use crate::setidx;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec::Vec;

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
    fn multiset_from_stdin(&self) -> multiset::MultiSet<String> {
        let mut s: multiset::MultiSet<String> = multiset::MultiSet::new();
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let l = line.unwrap();
            let p = l.split(self.separator).enumerate();
            for (i, x) in p {
                // 空文字列を無視する
                if !self.use_empty && x.is_empty() {
                    continue;
                }
                s.insert(i, x.to_owned());
            }
        }
        s
    }
    fn multiset_from_files(&self) -> multiset::MultiSet<String> {
        let mut s: multiset::MultiSet<String> = multiset::MultiSet::new();
        for (i, f) in self.files.iter().enumerate() {
            let file = File::open(f).expect("file not found");
            let r = io::BufReader::new(file);
            for line in r.lines() {
                let l = line.unwrap();
                // 空文字列を無視する
                if !self.use_empty && l.is_empty() {
                    continue;
                }
                s.insert(i, l.to_owned());
            }
        }
        s
    }
    fn gen_multiset(&self) -> multiset::MultiSet<String> {
        if self.files.is_empty() {
            self.multiset_from_stdin()
        } else {
            self.multiset_from_files()
        }
    }
    fn element_to_string(
        e: &multiset::MultiSetElement<String>,
        indexes: &[setidx::Idx],
    ) -> String {
        indexes
            .iter()
            .map(|x| e.indexes().contains(x))
            .map(|x| if x { "1" } else { "0" })
            .collect::<Vec<_>>()
            .join(" ")
    }
    fn output_element(p: &multiset::MultiSetElement<String>, indexes: &[setidx::Idx]) {
        let e = p.element();
        let r = Executor::element_to_string(p, indexes);
        println!("{} {}", e, r);
    }
    fn output_elements(it: multiset::MultiSetIterator<String>, indexes: &[setidx::Idx]) {
        it.for_each(|p| Executor::output_element(&p, indexes));
    }
    fn output_elements_with_sort(
        it: multiset::MultiSetIterator<String>,
        indexes: &[setidx::Idx],
    ) {
        let mut v = it.collect::<Vec<_>>();
        v.sort_by_key(|a| a.element());
        v.iter().for_each(|p| Executor::output_element(p, indexes));
    }
    fn print_elements(&self, it: multiset::MultiSetIterator<String>, indexes: &[setidx::Idx]) {
        if self.use_sort {
            Executor::output_elements_with_sort(it, indexes);
        } else {
            Executor::output_elements(it, indexes);
        }
    }
    pub fn execute(&self) {
        let ms = self.gen_multiset();
        let indexes: Vec<setidx::Idx> =
            setidx::Indexes::new(ms.len(), self.depth_begin, self.depth_end).collect();
        let header = indexes
            .iter()
            .map(|x| x.to_bidx(ms.len()))
            .map(|x| {
                if self.use_decimal_index {
                    format!("{}", x.to_decimal())
                } else {
                    x.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        println!("u {}", header);
        let msi = ms.element_iter(indexes.clone());
        self.print_elements(msi, &indexes);
    }
}
