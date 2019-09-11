import unittest

import time
from addcomb import *

class TestInterface(unittest.TestCase):

    def test_auto_dispatch(self):
        """
        Check that the right functions are called when
        different types of parameters are entered
        """
        # Normal functions:
        _ = nu(10, 5, 3)
        # Non-cyclic groups:
        _ = tau_signed_restricted((2, 4), 4)


    def test_no_verbose_error(self):
        """
        Run a function with verbose to make sure no
        exceptions occur
        """
        _ = nu(10, 5, 3, verbose=True)
        _ = tau_signed_restricted((2, 4), 4, verbose=True)

    def test_invalid_formats(self):
        """
        Test to make sure exceptions are raised when:
        - functions are given too many parameters
        - functions are given tuples in the wrong argument place
        - functions are given too few parameters
        But nothing should crash

        Since all of the functions are wrapped twice, it's
        useful to make sure none of the wrappers are behaving
        poorly
        """
        # Too many parameters:
        try:
            nu(10, 3, 2, 1, 0)
            self.fail()
        except Exception as e:
            pass

        try:
            phi((10, 2), 3, 2, 1)
            self.fail()
        except Exception as e:
            pass
        
        # Wrong tuples
        try:
            nu(5, (2, 1), 2)
            self.fail()
        except Exception as e:
            pass

        try:
            mu(5, (1, 2), 2)
            self.fail()
        except Exception as e:
            pass
        
        # Too few parameters:
        try:
            nu(10, 3)
            self.fail()
        except Exception as e:
            pass
        try:
            phi(10)
            self.fail()
        except Exception as e:
            pass

if __name__ == "__main__":
    unittest.main()