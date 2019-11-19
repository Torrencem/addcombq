The nu function is defined so that nu(G, m, h) is the largest size of hA, where |A| = m. In other words, nu(G, m, h) is the largest the h-fold sumset of a size m subset of G can be.

ARGUMENTS:

* G - Either an integer n (representing G = Z_n) or a tuple (n1, n2, ..., nm) (representing G = Z_n1 * Z_n2 * ... * Z_nm)

* m - An integer representing the size of the subset A

* h - An integer

* (optional) verbose [default: False] - Print a subset A which maximizes |hA|

--- uui ---

Note that we have a relation between nu and nu_interval:

v(G, m, [0, s]) = v(G, m + 1, s)

(see Proposition A.9)
