use crate::fastset::*;

use crate::setlike::HFolds;

#[inline]
fn in_interval(val: u32, interval: (u32, u32)) -> bool {
    let (a, b) = interval;
    (val >= a) && (val <= b)
}

impl<B: BitSetContents> HFolds for FastSet<B> {
    type Element = u32;
    type Group = u32;

    #[inline]
    fn hfold_sumset(&self, h: u32, n: u32) -> FastSet<B> {
        if h == 0 {
            return singleton(0);
        }
        let mut res = B::zero();
        let mut prev = B::one();
        for _ in 0..h {
            let mut c1 = self.contents;
            while c1 != B::zero() {
                let shift = c1.bit_scan_low();
                let cycled = cycle(prev, shift, n);
                res |= cycled;

                c1 &= c1 - B::one();
            }
            prev = res;
            res = B::zero();
        }
        FastSet { contents: prev }
    }

    #[inline]
    fn hfold_interval_sumset(&self, hs: (u32, u32), n: u32) -> FastSet<B> {
        let (h1, h2) = hs;

        let mut final_res = B::zero();
        let mut res = B::zero();
        let mut prev = B::one();
        for i in 0..=hs.1 {
            if in_interval(i, (h1, h2)) {
                final_res |= prev;
            }
            let mut c1 = self.contents;
            while c1 != B::zero() {
                let shift = c1.bit_scan_low();
                let cycled = cycle(prev, shift, n);
                res |= cycled;

                c1 &= c1 - B::one();
            }
            prev = res;

            res = B::zero();
        }
        FastSet {
            contents: final_res,
        }
    }

    #[inline]
    fn hfold_restricted_sumset(&self, h: u32, n: u32) -> FastSet<B> {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfrs(self.contents, B::one(), h, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_restricted_sumset(&self, hs: (u32, u32), n: u32) -> FastSet<B> {
        FastSet {
            contents: _hfirs(self.contents, B::one(), hs.1, hs, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_signed_sumset(&self, h: u32, n: u32) -> FastSet<B> {
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfss(self.contents, B::one(), h, n, empty_set(), empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_signed_sumset(&self, hs: (u32, u32), n: u32) -> FastSet<B> {
        FastSet {
            contents: _hfiss(
                self.contents,
                B::one(),
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
    fn hfold_restricted_signed_sumset(&self, h: u32, n: u32) -> FastSet<B> {
        if h > self.size() {
            return empty_set();
        }
        if h == 0 {
            return singleton(0);
        }
        FastSet {
            contents: _hfrss(self.contents, B::one(), h, n, empty_set(), n + 1),
        }
    }

    #[inline]
    fn hfold_interval_restricted_signed_sumset(&self, hs: (u32, u32), n: u32) -> FastSet<B> {
        FastSet {
            contents: _hfirss(self.contents, B::one(), hs.1, hs, n, empty_set(), n + 1),
        }
    }
}

fn _hfrss<B: BitSetContents>(stat: B, curr: B, h: u32, n: u32, restrictions: FastSet<B>, ceiling: u32) -> B {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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

        toadd &= toadd - B::one();
    }
    total
}

fn _hfirss<B: BitSetContents>(
    stat: B,
    curr: B,
    h: u32,
    hs: (u32, u32),
    n: u32,
    restrictions: FastSet<B>,
    ceiling: u32,
) -> B {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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

        toadd &= toadd - B::one();
    }
    total
}

fn _hfss<B: BitSetContents>(
    stat: B,
    curr: B,
    h: u32,
    n: u32,
    prestrictions: FastSet<B>,
    nrestrictions: FastSet<B>,
    ceiling: u32,
) -> B {
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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
        toadd &= toadd - B::one();
    }
    total
}

fn _hfiss<B: BitSetContents>(
    stat: B,
    curr: B,
    h: u32,
    hs: (u32, u32),
    n: u32,
    prestrictions: FastSet<B>,
    nrestrictions: FastSet<B>,
    ceiling: u32,
) -> B {
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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
        toadd &= toadd - B::one();
    }
    total
}

fn _hfrs<B: BitSetContents>(stat: B, curr: B, h: u32, n: u32, restrictions: FastSet<B>, ceiling: u32) -> B {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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

        toadd &= toadd - B::one();
    }
    total
}

fn _hfirs<B: BitSetContents>(
    stat: B,
    curr: B,
    h: u32,
    hs: (u32, u32),
    n: u32,
    restrictions: FastSet<B>,
    ceiling: u32,
) -> B {
    // A 1 in restrictions[i] means i has already been added
    if h == 0 {
        return curr;
    }
    let mut total = B::zero();
    if in_interval(hs.1 - h, hs) {
        total = curr;
    }
    let mut toadd = stat;
    while toadd != B::zero() {
        let shift = toadd.bit_scan_low();
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

        toadd &= toadd - B::one();
    }
    total
}
