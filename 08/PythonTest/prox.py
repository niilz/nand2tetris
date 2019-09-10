#!/usr/bin/env python

import subprocess
import sys

subprocess.call(['./test_bin', sys.argv[1]])
print("hello wordl")