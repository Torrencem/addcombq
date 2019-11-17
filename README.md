# addcombq

This is a Sage package (.spkg) for fast Additive Combinatorics functions in Sage. Check out [Bela Bajnok's book](https://arxiv.org/pdf/1705.07444.pdf) on the subject to get an idea of the types of problems these can be used for.

At the moment, the only way to make the install work correctly is to have cargo installed as "cargo" beforehand, and to use INSTALL.sh

To install into your local sage, make sure the sage installation is in PATH as "sage", then run INSTALL.sh. This will detect your sage installation, package and copy the correct files into it, and then run the sage package build. To instruct sage to run spkg-check after installation, run `INSTALL.sh test` instead.
