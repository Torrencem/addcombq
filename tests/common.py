
def from_table(asserter, table):
    for func in table.keys():
        for params in table[func].keys():
            asserter(func(*params), table[func][params])