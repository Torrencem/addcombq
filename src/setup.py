
import os, errno
from setuptools import setup, Extension
from setuptools.command.build_ext import build_ext

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
              ''' Copies the already-compiled extension
              (based on https://stackoverflow.com/a/12012086/6504760)
              '''
              import shutil
              # 3 specific:
              #os.makedirs(os.path.dirname(self.get_ext_fullpath(ext.name)), exist_ok=True)

              try:
                     os.makedirs(os.path.dirname(self.get_ext_fullpath(ext.name)))
              except OSError as e:
                     if e.errno != errno.EEXIST:
                            raise
              
              print("Setup.py: Copied file to location:")
              print(self.get_ext_fullpath(ext.name))
              shutil.copyfile(ext.source, self.get_ext_fullpath(ext.name))
              

setup (name = 'addcomb',
       version = '0.1',
       author = "Matt Torrence <gh-Torrencem>",
       description = """Fast Additive Combinatorics for use in Sage""",
       cmdclass = {
              'build_ext': cp_build_ext
       },
       ext_modules=[PreCompExtension("addcomb", "./build/_addcomb.so")],
       zip_safe=False
       )
