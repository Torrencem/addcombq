import sys
import os, errno
from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext
import os

class PreCompExtension(Extension):
       def __init__(self, name, source=''):
              '''
              Extension based on an existing compiled file
              (based on https://github.com/pybind/cmake_example/blob/master/setup.py)
              '''
              Extension.__init__(self, name, sources=[])
              self.source = os.path.abspath(source)

class cp_build_ext(build_ext):
       def build_extension(self, ext):
              # run build commands
              import subprocess
              if sys.version_info >= (3, 0):
                  subprocess.check_call("cargo +nightly build --release --no-default-features --features \"python3\"", shell=True)
              else:
                  subprocess.check_call("cargo +nightly build --release", shell=True)

              import shutil
              from os import path

              try:
                     os.makedirs(os.path.dirname(self.get_ext_fullpath(ext.name)))
              except OSError as e:
                     if e.errno != errno.EEXIST:
                            raise
              
              if path.exists("./target/release/libaddcombq.dylib"):
                  shutil.copyfile("./target/release/libaddcombq.dylib", ext.source)
              elif path.exists("./target/release/libaddcombq.so"):
                  shutil.copyfile("./target/release/libaddcombq.so", ext.source)
              print("Setup.py: Copying file to location:")
              print(self.get_ext_fullpath(ext.name))
              shutil.copyfile(ext.source, self.get_ext_fullpath(ext.name))
              

setup (name = 'addcomb',
       version = '0.2.2',
       author = "Matt Torrence <gh-Torrencem>",
       description = """Fast Additive Combinatorics for use in Sage""",
       cmdclass = {
              'build_ext': cp_build_ext
       },
       ext_modules=[PreCompExtension("addcomb", "./build/_addcomb.so")],
       zip_safe=False,
       include_package_data=True
       )
