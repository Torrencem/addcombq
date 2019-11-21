use crate::fastset::*;

use crate::setlike::HFolds;

#[inline]
fn bit_scan_low(val: u64) -> u32 {
    return val.trailing_zeros() as u32;
}

#[inline]
fn in_interval(val: u32, interval: (u32, u32)) -> bool {
    let (a, b) = interval;
    (val >= a) && (val <= b)
}

impl HFolds for FastSet {
    type Element = u32;
    type Group = u32;

    #[inline]
    fn hfold_sumset(&self, h: u32, n: u32) -> FastSet {
        if h == 0 {
            return singleton(0);
        }
        let mut res = 0u64;
        let mut prev = 1u64;
        for _ in 0..h {
            let mut c1 = self.contents;
            while c1 != 0 {
                let shift = bit_scan_low(c1);
                let cycled = cycle(prev, shift, n);
                res |= cycled;

                c1 &= c1 - 1;
            }
            prev = res;
            res = 0u64;
        }
        FastSet { contents: prev }
    }

    #[inline]
    fn hfold_interval_sumset(&self, hs: (u32, u32), n: u32) -> FastSet {
        let (h1, h2) = hs;

        let mut final_res = 0u64;
        let mut res = 0u64;
        let mut prev = 1u64;
        for i in 0..=hs.1 {
            if in_interval(i, (h1, h2)) {
                final_res |= prev;
            }
            let mut c1 = self.contents;
            while c1 != 0 {
                let shift = bit_scan_low(c1);
                let cycled = cycle(prev, shift, n);
                res |= cycled;

                c1 &= c1 - 1;
            }
            prev = res;

            res = 0u64;
        }
        FastSet {
            contents: final_res,
        }
    }

    #[inline]
    fn hfold_restricted_sumset(&self, h: u32, n: u32) -> FastSet {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfrs(self.contents, 1u64, h, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_restricted_sumset(&self, hs: (u32, u32), n: u32) -> FastSet {
        FastSet {
            contents: _hfirs(self.contents, 1u64, hs.1, hs, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_signed_sumset(&self, h: u32, n: u32) -> FastSet {
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfss(self.contents, 1u64, h, n, empty_set(), empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_signed_sumset(&self, hs: (u32, u32), n: u32) -> FastSet {
        FastSet {
            contents: _hfiss(
                self.contents,
                1u64,
                hs.1,
                hs,
                n,
                empty_set(),
                empty_set(),
                n + 1,
            ),
        }
    }

    #[inline]
    fn hfold_restricted_signed_sumset(&self, h: u32, n: u32) -> FastSet {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfrss(self.contents, 1u64, h, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_restricted_signed_sumset(&self, hs: (u32, u32), n: u32) -> FastSet {
        FastSet {
            contents: _hfirss(self.contents, 1u64, hs.1, hs, n, empty_set(), n + 1),
        }
    }
}

fn _hfrss(stat: u64, curr: u64, h: u32, n: u32, restrictions: FastSet, ceiling: u32) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }
        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrss(stat, cycled, h - 1, n, newrestr, shift);
            total |= rec_call;

            // Also choose -cycled
            let cycled = cycle_rev(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrss(stat, cycled, h - 1, n, newrestr, shift);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}

fn _hfirss(
    stat: u64,
    curr: u64,
    h: u32,
    hs: (u32, u32),
    n: u32,
    restrictions: FastSet,
    ceiling: u32,
) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }
        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfirss(stat, cycled, h - 1, hs, n, newrestr, shift);
            total |= rec_call;

            // Also choose -cycled
            let cycled = cycle_rev(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfirss(stat, cycled, h - 1, hs, n, newrestr, shift);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}

fn _hfss(
    stat: u64,
    curr: u64,
    h: u32,
    n: u32,
    prestrictions: FastSet,
    nrestrictions: FastSet,
    ceiling: u32,
) -> u64 {
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }
        if !prestrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newnrestr = nrestrictions.clone();
            newnrestr.add(shift);

            let rec_call = _hfss(
                stat,
                cycled,
                h - 1,
                n,
                prestrictions.clone(),
                newnrestr,
                shift,
            );
            total |= rec_call;
        }
        if !nrestrictions.access(shift) {
            let cycled = cycle_rev(curr, shift, n);
            let mut newprestr = prestrictions.clone();
            newprestr.add(shift);

            let rec_call = _hfss(
                stat,
                cycled,
                h - 1,
                n,
                newprestr,
                nrestrictions.clone(),
                shift,
            );
            total |= rec_call;
        }
        toadd &= toadd - 1;
    }
    total
}

fn _hfiss(
    stat: u64,
    curr: u64,
    h: u32,
    hs: (u32, u32),
    n: u32,
    prestrictions: FastSet,
    nrestrictions: FastSet,
    ceiling: u32,
) -> u64 {
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }
        if !prestrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newnrestr = nrestrictions.clone();
            newnrestr.add(shift);

            let rec_call = _hfiss(
                stat,
                cycled,
                h - 1,
                hs,
                n,
                prestrictions.clone(),
                newnrestr,
                shift,
            );
            total |= rec_call;
        }
        if !nrestrictions.access(shift) {
            let cycled = cycle_rev(curr, shift, n);
            let mut newprestr = prestrictions.clone();
            newprestr.add(shift);

            let rec_call = _hfiss(
                stat,
                cycled,
                h - 1,
                hs,
                n,
                newprestr,
                nrestrictions.clone(),
                shift,
            );
            total |= rec_call;
        }
        toadd &= toadd - 1;
    }
    total
}

fn _hfrs(stat: u64, curr: u64, h: u32, n: u32, restrictions: FastSet, ceiling: u32) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }

        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfrs(stat, cycled, h - 1, n, newrestr, shift);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}

fn _hfirs(
    stat: u64,
    curr: u64,
    h: u32,
    hs: (u32, u32),
    n: u32,
    restrictions: FastSet,
    ceiling: u32,
) -> u64 {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = 0u64;
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != 0 {
        let shift = bit_scan_low(toadd);
        if shift > ceiling {
            break;
        }

        if !restrictions.access(shift) {
            let cycled = cycle(curr, shift, n);
            let mut newrestr = restrictions.clone();
            newrestr.add(shift);

            let rec_call = _hfirs(stat, cycled, h - 1, hs, n, newrestr, shift);
            total |= rec_call;
        }

        toadd &= toadd - 1;
    }
    total
}
