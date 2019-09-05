from pathlib import Path
import re

b_functions = ['nu', 'phi', 'sigma', 'rho', 'chi', 'tau', 'mu']

variation_props = [('u', 's'), ('u', 'r'), ('b', 'i')]

_variation_prop_to_name = [
    {
        'u': None,
        's': 'signed'
    },
    {
        'u': None,
        'r': 'restricted'
    },
    {
        'b': None,
        'i': 'interval'
    }
]

disallowed_mu_vars = [
    'uui', 'uri', 'sui', 'sri'
]

# Expects a variation like uri
# (returns restricted_interval)
def var_to_name(var):
    res = ""
    for ind, char in enumerate(var):
        v = _variation_prop_to_name[ind][char]
        if v:
            res += "_" + v
    return res

def all_variations():
    s = 0b000
    while s != 0b1000:
        res = ""
        res += variation_props[0][(s & 0b1)   >> 0]
        res += variation_props[1][(s & 0b10)  >> 1]
        res += variation_props[2][(s & 0b100) >> 2]
        yield res
        s += 1

def build():
    # Clean up the build directory
    build_dir = Path("./compiled/")
    build_dir.mkdir(exist_ok=True)
    # Find all base level files in ./b_functions/NAME.md
    base_paths = map(lambda bf: Path("./b_functions/" + bf + ".md"), b_functions)
    base_paths = filter(lambda path: path.is_file(), base_paths)

    for bf_name, p in zip(b_functions, base_paths):
        for var in all_variations():
            # Specific mu case
            if bf_name == "mu" and var in disallowed_mu_vars:
                continue
            # Read the markdown from the file
            raw_data = p.read_text()
            # Remove sections that don't correspond to this variation
            state_rm_sec = False
            proc_data = ""
            for line in raw_data.split("\n"):
                if line.strip() == "--- " + var + " ---":
                    state_rm_sec = False
                    continue
                if re.match("--- \w+ ---", line):
                    state_rm_sec = True
                    continue
                if line.strip() == "------":
                    state_rm_sec = False
                    continue
                if not state_rm_sec:
                    proc_data += line + "\n"
            
            # Add in common_---.md 's
            common_path = Path("./b_functions/common/" + var + ".md")
            if common_path.is_file():
                proc_data += common_path.read_text()
            
            var_name = var_to_name(var)
            file_name = bf_name + var_name + ".md"
            
            out_file_path = build_dir.joinpath(file_name)

            if out_file_path.is_file():
                out_file_path.unlink()

            out_file_path.write_text(proc_data)

if __name__ == "__main__":
    build()