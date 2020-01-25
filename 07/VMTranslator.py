#!/usr/bin/env python

# USAGE:
# Rename RUST-build (VMtranlator compiled for x86_64-linux-musl) to "VM"
# Put "VMTranslator.py" and "VM" in same directory and zip it
# (Make sure "VMTranslator.py" has executing rights)
# Upload zip as projecX.zip (X = number of project e.g. 8)

import subprocess
import sys

subprocess.call(['./VM', sys.argv[1]])
print("Rust VMtranslator has successfully been called by proxy.py")