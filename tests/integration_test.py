from __future__ import print_function
import addcomb

expected_exports = ['_chi', '_chi_restricted', '_chi_signed', '_chi_signed_restricted', '_mu', '_mu_restricted', '_mu_signed', '_mu_signed_restricted', '_nu', '_nu_restricted', '_nu_signed', '_nu_signed_restricted', '_phi', '_phi_restricted', '_phi_signed', '_phi_signed_restricted', '_rho', '_rho_restricted', '_rho_signed', '_rho_signed_restricted', '_sigma', '_sigma_restricted', '_sigma_signed', '_sigma_signed_restricted', '_tau', '_tau_restricted', '_tau_signed', '_tau_signed_restricted', 'a', 'c', 'chi', 'chi_restricted', 'chi_signed', 'chi_signed_restricted', 'choose', 'mu', 'mu_restricted', 'mu_signed', 'mu_signed_restricted', 'nu', 'nu_restricted', 'nu_signed', 'nu_signed_restricted', 'phi', 'phi_restricted', 'phi_signed', 'phi_signed_restricted', 'rho', 'rho_restricted', 'rho_signed', 'rho_signed_restricted', 'sigma', 'sigma_restricted', 'sigma_signed', 'sigma_signed_restricted', 'tau', 'tau_restricted', 'tau_signed', 'tau_signed_restricted', 'v', 'v_signed']

for export in expected_exports:
    if export not in dir(addcomb):
        print("Test fail: function not exported: " + export)
        exit(1)

from addcomb import nu, _nu
if nu(5, 3, 2) <= 0:
    exit(1)
if _nu(5, 3, 2) <= 0:
    exit(1)
