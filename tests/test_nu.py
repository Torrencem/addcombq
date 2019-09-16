import unittest

from addcomb import *
from sage.all import binomial
from common import from_table

class TestNu(unittest.TestCase):

    def test_values(self):
        # Based on part of the table in Problem A.5
        correct_table = {
            11: 2, 12: 3
        }

        table = {x:0 for x in range(11, 13)}
        for n in range(11, 13):
            for m in range(1, n):
                for h in range(1, n/2):
                    expected = min(n, binomial(m + h - 1, h))
                    actual = nu(n, m, h)
                    if expected != actual:
                        table[n] += 1

        self.assertDictEqual(correct_table, table)

        # From Problem A.31
        from_table(self.assertEqual, {
            nu_restricted: {
                (10, 5, 2): 9,
                (15, 6, 4): 13,
                (17, 6, 4): 14
            }
        })
    
    def test_edge_cases(self):
        from_table(self.assertEqual, {
            nu: {
                # Based on Proposition A.2
                (30, 3, 0): 1,
                ((50, 10), 2, 0): 1,
                (30, 2, 1): 2,
                ((10, 2), 4, 1): 4,
                ((5, 5), 1, 10): 1
            },
            nu_signed: {
                # Based on Proposition A.12
                (3, 2, 0): 1,
                ((4, 10), 3, 0): 1,
                # Interval
                (6, 5, (0, 1)): 6,
                # Proposition A.19
                (10, 1, (0, 2)): 5
            },
            nu_restricted: {
                # Proposition A.26
                (10, 3, 0): 1,
                ((3, 6), 3, 1): 3,
                ((2, 4), 4, 3): 4,
                (15, 3, 5): 0,
                ((5, 10), 2, 4): 0,
            },
            nu_signed_restricted: {
                # Proposition A.39
                (10, 3, 0): 1,
                ((4, 6), 2, 0): 1,
                (7, 5, 1): 7,
                # Interval
                # Proposition A.46
                (10, 2, (0, 0)): 1,
                ((4, 6), 2, (0, 0)): 1
            }
        })

if __name__ == "__main__":
    unittest.main()