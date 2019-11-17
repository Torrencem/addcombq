The _nu_ function is defined so that _nu(G, m, h)_ is the largest size of _hA_, where \|A\| = m. In other words, _nu(G, m, h)_ is the largest the _h_-fold sumset of a size _m_ subset of _G_ can be.

ARGUMENTS:

* G - Either an integer _n_ (representing G = Z\_n) or a tuple _(n1, n2, ..., nm)_ (representing G = Z\_n1 * Z\_n2 * ... * Z\_nm)

* m - An integer representing the size of the subset _A_

* h - An integer

* (optional) verbose \[default: False\] - Print a subset _A_ which maximizes _|hA|_

--- uui ---

Note that we have a relation between _nu_ and _nu interval_:

v(G, m, [0, s]) = v(G, m + 1, s)

(see Proposition A.9)
