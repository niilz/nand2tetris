#!/usr/bin/env python

import subprocess
import sys

subprocess.call(['./VM', sys.argv[1]])
print("Rust VMtranslator has successfully been called by proxy.py")