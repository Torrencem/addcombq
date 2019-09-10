import unittest

from addcomb import nu
from sage.all import binomial

class TestNu(unittest.TestCase):

    def test_nu_values(self):
        # Based on part of the table in Problem A.5
        correct_table = {
            11: 2, 12: 3,
            13: 1, 14: 2
        }

        table = {x:0 for x in range(11, 15)}
        for n in range(11, 15):
            for m in range(1, n):
                for h in range(1, n/2):
                    expected = min(n, binomial(m + h - 1, h))
                    actual = nu(n, m, h)
                    if expected != actual:
                        table[n] += 1

        self.assertDictEqual(correct_table, table)

    def test_nu_edge_cases(self):
        # Edge cases
        # Based on Proposition A.2

        self.assertEqual(nu(30, 3, 0), 1)
        self.assertEqual(nu((50,10), 2, 0), 1)
        self.assertEqual(nu(30, 2, 1), 2)
        self.assertEqual(nu((10, 2), 4, 1), 4)
        self.assertEqual(nu((5, 5), 1, 10), 1)

    def test_nu_signed_values(self):
        # Based on part of the table in Problem A.18
        pass


    def test_nu_signed_edge_cases(self):
        # Edge cases
        # Based on Proposition A.12

        self.assertEqual(nu(3, 2, 0), 1)
        self.assertEqual(nu((4,10), 3, 0), 1)
if __name__ == "__main__":
    unittest.main()