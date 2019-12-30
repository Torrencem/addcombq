# -*- coding: utf-8 -*-

import ctypes
# from ctypes import *
from ctypes import util

# use_errno parameter is optional, because I'm not checking errno anyway.
libraryC = ctypes.util.find_library('c')
libc = ctypes.CDLL(libraryC, use_errno=True)


# libc = cdll.msvcrt


class FILE(ctypes.Structure):
    pass


FILE_p = ctypes.POINTER(FILE)

# Alternatively, we can just use:
# FILE_p = ctypes.c_void_p

# These variables, defined inside the C library, are readonly.
##cstdin = FILE_p.in_dll(libc, 'stdin')
##cstdout = FILE_p.in_dll(libc, 'stdout')
##cstderr = FILE_p.in_dll(libc, 'stderr')

# C function to disable buffering.
csetbuf = libc.setbuf
csetbuf.argtypes = (FILE_p, ctypes.c_char_p)
csetbuf.restype = None

# C function to flush the C library buffer.
cfflush = libc.fflush
cfflush.argtypes = (FILE_p,)
cfflush.restype = ctypes.c_int

import io
import os
import sys
import tempfile
from contextlib import contextmanager
#import cStringIO

def read_as_encoding(fileno, encoding="utf-8"):
    fp = io.open(fileno, mode="r+", encoding=encoding, closefd=False)
    return fp

class Logger(object):
    def __init__(self, file, encoding='utf-8'):
        self.file = file
        self.encoding = encoding

    def write(self, message):
        self.file.flush()  # Meed to flush
        # python2 temp file is always binary
        # msg_unicode = message.('utf-8')
        self.file.write(message)
    
    def flush(self):
        self.file.flush()


def capture_c_stdout(on_output, on_error=None, encoding='utf8'):
    # Flushing, it's a good practice.
    sys.stdout.flush()
    sys.stderr.flush()
    # cfflush(cstdout)
    # cfflush(cstdcerr)

    # We need to use a actual file because we need the file descriptor number.
    with tempfile.NamedTemporaryFile() as temp:
        with tempfile.NamedTemporaryFile() as temp_err:
            # print "TempName:", temp.name
            # print "TempErrName:", temp_err.name

            # Saving a copy of the original stdout.
            prev_sys_stdout = sys.stdout
            prev_stdout_fd = os.dup(1)
            os.close(1)
            # Duplicating the temporary file fd into the stdout fd.
            # In other words, replacing the stdout.
            os.dup2(temp.fileno(), 1)

            if on_error:
                prev_sys_stderr = sys.stderr
                prev_stderr_fd = os.dup(2)
                os.close(2)
                os.dup2(temp_err.fileno(), 2)

            # Replacing sys.stdout for Python code.
            #
            # IPython Notebook version of sys.stdout is actually an
            # in-memory OutStream, so it does not have a file descriptor.
            # We need to replace sys.stdout so that interleaved Python
            # and C output gets captured in the correct order.
            #
            # We enable line_buffering to force a flush after each line.
            # And write_through to force all data to be passed through the
            # wrapper directly into the binary temporary file.
            # No need to use TextIOWrapper in python2, in python2, tempFile is always binary according to official document
            ##temp_wrapper = io.TextIOWrapper(
            ##   read_as_encoding(temp.fileno(), encoding=encoding), encoding=encoding, line_buffering=True) ##, write_through=True)

            # temp_wrapper_python = io.TextIOWrapper(
            #    read_as_encoding(temp.fileno(), encoding=encoding), encoding='ascii', line_buffering=True)
            temp_wrapper_python = Logger(temp, encoding=encoding)
            sys.stdout = temp_wrapper_python

            if on_error:
                # temp_wrapper_err = io.TextIOWrapper(
                #   read_as_encoding(temp_err.fileno(), encoding=encoding), encoding=encoding, line_buffering=True) ##, write_through=True)
                temp_wrapper_python_err = Logger(temp_err, encoding=encoding)
                # string_str_err = cStringIO.StringIO()
                sys.stderr = temp_wrapper_python_err

            # Disabling buffering of C stdout.
            ##csetbuf(cstdout, None)

            yield

            # Must flush to clear the C library buffer.
            ##cfflush(cstdout)

            # Restoring stdout.
            os.dup2(prev_stdout_fd, 1)
            os.close(prev_stdout_fd)
            sys.stdout = prev_sys_stdout

            if on_error:
                os.dup2(prev_stderr_fd, 2)
                os.close(prev_stderr_fd)
                sys.stderr = prev_sys_stderr

            # Printing the captured output.
            # temp_wrapper.seek(0)
            # print "Reading: "
            # print temp_wrapper.read()
            if on_output:
                temp.flush()
                temp.seek(0)
                on_output(temp.read())
            temp.close()

            if on_error:
                temp_err.flush()
                temp_err.seek(0)
                on_error(temp_err.read())
                temp_err.close()

