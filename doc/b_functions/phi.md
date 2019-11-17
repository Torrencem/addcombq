The _phi_ function is defined so that _phi(G, h)_ is the minimum size of a _spanning set_ of _G_. A spanning set is a set _A_ so that the _h_ fold sumset of _A_ is the entire group _G_.

ARGUMENTS:

* G - Either an integer _n_ (representing G = Z\_n) or a tuple _(n1, n2, ..., nm)_ (representing G = Z\_n1 * Z\_n2 * ... * Z\_nm)

* h - An integer

* (optional) verbose \[default: False\] - Print a spanning set _A_ of size _phi(G, h)_