use std::collections::HashSet;
use std::fmt;
use std::iter::IntoIterator;

use itertools::Combinations;
use itertools::Itertools;

use std::rc::Rc;

use std::iter;

pub struct CombWithReplacement {
    indices: Vec<u32>,
    n: u32,
    r: u32,
    first: bool,
}

impl Iterator for CombWithReplacement {
    type Item = Vec<u32>;

    // Based very much on https://docs.python.org/3/library/itertools.html#itertools.combinations
    fn next(&mut self) -> Option<Vec<u32>> {
        if self.first {
            self.first = false;
            return Some(vec![0; self.r as usize]);
        }
        let mut found: bool = false;
        let mut found_index: usize = 0;

        for i in (0..self.r).rev() {
            if self.indices[i as usize] != self.n - 1 {
                found_index = i as usize;
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }

        let val_at = self.indices[found_index] + 1;

        self.indices.splice(
            found_index..,
            iter::repeat(val_at).take(self.r as usize - found_index),
        );

        Some(self.indices.clone())
    }
}

pub fn combinations_with_replacement(n: u32, r: u32) -> CombWithReplacement {
    CombWithReplacement {
        indices: vec![0; r as usize],
        n: n,
        r: r,
        first: true,
    }
}

pub struct EachElement {
    pub curr: Vec<u32>,
    pub mod_v: Rc<Vec<u32>>,
    pub first: bool,
}

impl Iterator for EachElement {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Vec<u32>> {
        if self.first {
            self.first = false;
            return Some(vec![0; self.mod_v.len()]);
        }
        let mut indx = 0;
        while self.curr[indx] == self.mod_v[indx] - 1 {
            self.curr[indx] = 0;
            indx += 1;
            if indx == self.curr.len() {
                return None;
            }
        }
        self.curr[indx] += 1;
        Some(self.curr.clone())
    }
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct GElem(pub Vec<u32>);

pub fn each_set_exact(size: u32, mod_v: Rc<Vec<u32>>) -> EachSetExact {
    EachSetExact {
        c: (EachElement {
            curr: vec![0; mod_v.len()],
            mod_v: mod_v.clone(),
            first: true,
        })
        .combinations(size as usize),
    }
}

pub struct EachSetExact {
    pub c: Combinations<EachElement>,
}

impl Iterator for EachSetExact {
    type Item = Vec<GElem>;

    fn next(&mut self) -> Option<Vec<GElem>> {
        let v: Vec<Vec<u32>> = self.c.next()?;
        Some(v.iter().map(|elem| GElem(elem.to_vec())).collect())
    }
}

pub fn each_set_exact_no_zero(size: u32, mod_v: Rc<Vec<u32>>) -> EachSetExact {
    EachSetExact {
        c: (EachElement {
            curr: vec![0; mod_v.len()],
            mod_v: mod_v.clone(),
            first: false,
        })
        .combinations(size as usize),
    }
}

#[inline]
pub fn mod_sum(x: &GElem, y: &GElem, mod_v: Rc<Vec<u32>>) -> GElem {
    let GElem(xc) = x;
    let GElem(yc) = y;
    debug_assert!(xc.len() == yc.len());
    let mut res: Vec<u32> = vec![0; xc.len()];
    for (((zref, xval), yval), mod_val) in res
        .iter_mut()
        .zip(xc.into_iter())
        .zip(yc.into_iter())
        .zip(mod_v.iter())
    {
        *zref = (xval + yval) % mod_val;
    }
    GElem(res)
}

#[inline]
pub fn elem_sub(x: &GElem, y: &GElem) -> GElem {
    let GElem(xc) = x;
    let GElem(yc) = y;
    debug_assert!(xc.len() == yc.len());
    let mut res: Vec<u32> = vec![0; xc.len()];
    for ((zref, xval), yval) in res.iter_mut().zip(xc.into_iter()).zip(yc.into_iter()) {
        *zref = xval - yval;
    }
    GElem(res)
}

impl fmt::Display for GElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let GElem(x) = self;
        let asstrs: Vec<String> = x.into_iter().map(|s| s.to_string()).collect();
        let asstr: String = asstrs.join(", ");
        write!(f, "({})", asstr)
    }
}

impl fmt::Debug for GElem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let GElem(x) = self;
        let asstrs: Vec<String> = x.into_iter().map(|s| s.to_string()).collect();
        let asstr: String = asstrs.join(", ");
        write!(f, "({})", asstr)
    }
}

pub fn hfold_sumset(set: &Vec<GElem>, h: u32, mod_v: Rc<Vec<u32>>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set.clone();
    let n: usize = mod_v.len();
    if as_vec.len() == 0 || h == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in combinations_with_replacement(as_vec.len() as u32, h) {
        res.insert(
            indices
                .into_iter()
                .map(|index| as_vec[index as usize].clone())
                .fold(GElem(vec![0; n]), |prev, curr| {
                    mod_sum(&prev, &curr, mod_v.clone())
                }),
        );
    }
    res
}

pub fn hfold_interval_sumset(
    set: &Vec<GElem>,
    intv: (u32, u32),
    mod_v: Rc<Vec<u32>>,
) -> HashSet<GElem> {
    let mut res: HashSet<_> = HashSet::new();
    let (ia, ib) = intv;
    for i in ia..=ib {
        let tmp = hfold_sumset(set, i, mod_v.clone());
        res = res.union(&tmp).cloned().collect();
    }
    res
}

pub fn hfold_signed_sumset(set: &Vec<GElem>, h: u32, mod_v: Rc<Vec<u32>>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set.clone();
    let n: usize = mod_v.len();
    if as_vec.len() == 0 || h == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in combinations_with_replacement(as_vec.len() as u32, h) {
        let mut coeffs: Vec<u32> = vec![1; indices.len()];
        loop {
            res.insert(
                indices
                    .clone()
                    .into_iter()
                    .map(|index| as_vec[index as usize].clone())
                    .enumerate()
                    .fold((0, GElem(vec![0; n])), |prev, curr| {
                        let (index, elem) = curr;
                        let (_, prev_elem) = prev;
                        if coeffs[index] == 0 {
                            (
                                0,
                                mod_sum(
                                    &prev_elem,
                                    &elem_sub(&GElem((*mod_v).to_vec()), &elem),
                                    mod_v.clone(),
                                ),
                            )
                        } else {
                            (0, mod_sum(&prev_elem, &elem, mod_v.clone()))
                        }
                    })
                    .1,
            );
            let mut found_index: usize = 0;
            let mut found = true;
            while coeffs[found_index] == 0 {
                if found_index == indices.len() - 1 {
                    found = false;
                    break;
                }
                coeffs[found_index] = 1;
                found_index += 1;
            }
            if !found {
                break;
            } else {
                // Fill consecutive indices which have the same value
                let val_at = indices[found_index];
                let mut indx = found_index;
                while indx < indices.len() && indices[indx] == val_at {
                    coeffs[indx] = 0;
                    indx += 1;
                }
            }
        }
    }
    res
}

pub fn hfold_interval_signed_sumset(
    set: &Vec<GElem>,
    intv: (u32, u32),
    mod_v: Rc<Vec<u32>>,
) -> HashSet<GElem> {
    let mut res: HashSet<_> = HashSet::new();
    let (ia, ib) = intv;
    for i in ia..=ib {
        if i == 0 {
            res.insert(GElem(vec![0; set.len()]));
            continue;
        }
        let tmp = hfold_signed_sumset(set, i, mod_v.clone());
        res = res.union(&tmp).cloned().collect();
    }
    res
}

pub fn hfold_restricted_sumset(set: &Vec<GElem>, h: u32, mod_v: Rc<Vec<u32>>) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set.clone();
    let n: usize = mod_v.len();
    if as_vec.len() == 0 || h == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in (0..as_vec.len() as u32).combinations(h as usize) {
        res.insert(
            indices
                .into_iter()
                .map(|index| as_vec[index as usize].clone())
                .fold(GElem(vec![0; n]), |prev, curr| {
                    mod_sum(&prev, &curr, mod_v.clone())
                }),
        );
    }
    res
}

pub fn hfold_interval_restricted_sumset(
    set: &Vec<GElem>,
    intv: (u32, u32),
    mod_v: Rc<Vec<u32>>,
) -> HashSet<GElem> {
    let mut res: HashSet<_> = HashSet::new();
    let (ia, ib) = intv;
    for i in ia..=ib {
        let tmp = hfold_restricted_sumset(set, i, mod_v.clone());
        res = res.union(&tmp).cloned().collect();
    }
    res
}

pub fn hfold_restricted_signed_sumset(
    set: &Vec<GElem>,
    h: u32,
    mod_v: Rc<Vec<u32>>,
) -> HashSet<GElem> {
    let mut res: HashSet<GElem> = HashSet::new();
    let as_vec: Vec<GElem> = set.clone();
    let n: usize = mod_v.len();
    if as_vec.len() == 0 || h == 0 {
        res.insert(GElem(vec![0; n]));
        return res;
    }

    for indices in (0..as_vec.len() as u32).combinations(h as usize) {
        let mut coeffs: Vec<u32> = vec![1; indices.len()];
        loop {
            res.insert(
                indices
                    .clone()
                    .into_iter()
                    .map(|index| as_vec[index as usize].clone())
                    .enumerate()
                    .fold((0, GElem(vec![0; n])), |prev, curr| {
                        let (index, elem) = curr;
                        let (_, prev_elem) = prev;
                        if coeffs[index] == 0 {
                            (
                                0,
                                mod_sum(
                                    &prev_elem,
                                    &elem_sub(&GElem((*mod_v).to_vec()), &elem),
                                    mod_v.clone(),
                                ),
                            )
                        } else {
                            (0, mod_sum(&prev_elem, &elem, mod_v.clone()))
                        }
                    })
                    .1,
            );
            let mut found_index: usize = 0;
            let mut found = true;
            while coeffs[found_index] == 0 {
                if found_index == indices.len() - 1 {
                    found = false;
                    break;
                }
                coeffs[found_index] = 1;
                found_index += 1;
            }
            if !found {
                break;
            } else {
                coeffs[found_index] = 0;
            }
        }
    }
    res
}

pub fn hfold_interval_restricted_signed_sumset(
    set: &Vec<GElem>,
    intv: (u32, u32),
    mod_v: Rc<Vec<u32>>,
) -> HashSet<GElem> {
    let mut res: HashSet<_> = HashSet::new();
    let (ia, ib) = intv;
    for i in ia..=ib {
        let tmp = hfold_restricted_signed_sumset(set, i, mod_v.clone());
        res = res.union(&tmp).cloned().collect();
    }
    res
}

pub fn empty_set() -> Vec<GElem> {
    vec![]
}
