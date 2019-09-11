# addcombq

This is a Sage package (.spkg) for fast Additive Combinatorics functions in Sage. Check out (Bela Bajnok's book)[https://arxiv.org/pdf/1705.07444.pdf] on the subject to get an idea of the types of problems these can be used for.

To install into your local sage, make sure the sage installation is in PATH as "sage", then run INSTALL.sh. This will detect your sage installation, package and copy the correct files into it, and then run the sage package build. To instruct sage to run spkg-check after installation, run `INSTALL.sh test` instead.

Are We Release Yet? Important goals towards release:

- [x] Port all code from the old WASM codebase
- [x] Make all functions support intervals being passed instead of h's
- [x] Clean up the code from interval argument conversions. In particular, find the TODO, and get rid of the .unwrap()s in valid code paths and replaces with try!'s and Results
- [x] Add some tests for error cases
- [ ] Add contributing / writing more functions guideline for codebase
- [ ] Make code styles more consistent
- [x] Add License
- [ ] Fix placeholder SPKG.txt
- [ ] Add documentation for non-exposed rust functions (`exact_set` and `fast_set`)
- [ ] Finish writing specific documentation for each b_function, possibly with the help of contributors
- [x] Setup a structure for a test suite `spkg-check` written in sage, which should run tests against most tables and some bounds in the book.
- [ ] Really test Sidon sets which haven't been tested yet
- [ ] Actually write `spkg-check`. If possible, aim for 100% function coverage (huge)
- [ ] Write benches (big)
- [ ] Bench the current method for `exact_set`s and try to find something that'll be much faster than `Vec`s (possibly a library or SIMD-like arrays)
- [ ] Consider Ivaylo's suggestion of finding faster traversals through the search space, and use benches to measure any improvement
- [ ] Create a web-based documentation framework skeleton with landing pages and links to Bela's book and explanations
- [ ] Be able to compile the current documentation into web format using BeautifulSoup and Markdown packages for python
- [ ] Fix the weird internet behavior from spkg-build on Mac (looks unlikely unfortunately)
- [ ] (optional) Improve error messages for "reasonable" failures in wrapper functions (such as invalid argument types)
- [ ] Make work on sage with Python3. The CPython package in Rust must be given a different feature flag.
- [ ] Implement (perhaps in Python) some of the extra functions Bela mentions in his book (v functions, c function, etc.)