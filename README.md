# addcombq

This is a Sage package (python package) for fast Additive Combinatorics functions in Sage. Check out [Bela Bajnok's book](https://arxiv.org/pdf/1705.07444.pdf) on the subject to get an idea of the types of problems these can be used for.

To install, make sure to install rust (and cargo), then run `python setup.py install --user`, or (preferred) `python -m pip install . --user` to install to your preferred python installation (2 or 3).

In sage, you can run the functions from the book as follows:


    from addcomb import nu_signed

    nu_signed((5, 5), 3, 2)   # Will compute the nu function from chapter A in the group Z5xZ5

    nu_signed((5, 5), 3, 2, verbose=True)    # Will print extra information as well (what sets were found)


If there's a function you don't know how to use, try using `help(function)`:


    help(nu)
    ...
    |  The nu function is defined so that nu(G, m, h) is the largest size of hA, where |A| = m. In other words, nu(G, m, h) is the largest the h-fold sumset of a size m subset of G can be.
    |
    |  ARGUMENTS:
    |
    |  * G - Either an integer n (representing G = Z_n) or a tuple (n1, n2, ..., nm) (representing G = Z_n1 * Z_n2 * ... * Z_nm)
    |
    |  * m - An integer representing the size of the subset A
    |
    |  * h - An integer
    |
    |  * (optional) verbose [default: False] - Print a subset A which maximizes |hA|
    |
    |  This function uses the _unsigned_, _unrestricted_ variation of sumsets. This means that in the sumset, terms are allowed to repeat and are not allowed to be subtracted. For more information, read the (link forthcoming) master page of details on sumsets.
    ...


This code example gives a list of all the available functions:

    import addcomb
    dir(addcomb)

For any potential developers, there will be a development guide coming soon that might be helpful for optimizing the code in this project. In the meantime, run the tests to make sure everything's working properly with:

    cargo test            # Run integration tests: installation, bindings, etc.
    cd addcomb-comp
    cargo test --release  # Run computation tests: all functions should work as expected
