# addcombq

This is a Sage package (.spkg) for fast Additive Combinatorics functions in Sage. Check out [Bela Bajnok's book](https://arxiv.org/pdf/1705.07444.pdf) on the subject to get an idea of the types of problems these can be used for.

At the moment, the only way to make the install work correctly is to have cargo installed as "cargo" beforehand, and to use INSTALL.sh

To install into your local sage, make sure the sage installation is in PATH as "sage", then run INSTALL.sh. This will detect your sage installation, package and copy the correct files into it, and then run the sage package build. To instruct sage to run spkg-check after installation, run `INSTALL.sh test` instead.

In sage, you can run the functions from the book as follows:


    from addcomb import nu_signed

    nu_signed((5, 5), 3, 2)   # Will compute the nu function from chapter A in the group Z5xZ5

    nu_signed((5, 5), 3, 2, verbose=True)    # Will print extra information as well (what sets were found)


If there's a function you don't know how to use, try using `help(function)`:


    help(nu)
    ...
     |  The _nu_ function is defined so that _nu(G, m, h)_ is the largest size of _hA_, where \|A\| = m. In other words, _nu(G, m, h)_ is the largest the _h_-fold sumset of a size _m_ subset of _G_ can be.
     |
     |  ARGUMENTS:
     |
     |  * G - Either an integer _n_ (representing G = Z\_n) or a tuple _(n1, n2, ..., nm)_ (representing G = Z\_n1 * Z\_n2 * ... * Z\_nm)
     |
     |  * m - An integer representing the size of the subset _A_
     |
     |  * h - An integer
     |
     |  * (optional) verbose \[default: False\] - Print a subset _A_ which maximizes _|hA|_
    ...


This code example gives a list of all the available functions:

    import addcomb
    dir(addcomb)
