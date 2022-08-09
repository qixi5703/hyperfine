#!/usr/bin/env python

"""This program shows `hyperfine` benchmark results in a sequential way
in order to debug possible background interference, caching effects,
thermal throttling and similar effects.
"""

import argparse
import json
from unittest import result
import matplotlib.pyplot as plt
import numpy as np


def moving_average(times, num_runs):
    times_padded = np.pad(
        times, (num_runs // 2, num_runs - 1 - num_runs // 2), mode="edge"
    )
    kernel = np.ones(num_runs) / num_runs
    return np.convolve(times_padded, kernel, mode="valid")


parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("file1", help="JSON file with benchmark results")
parser.add_argument("file2", help="JSON file with benchmark results")
parser.add_argument("--title", help="Plot Title")
parser.add_argument("-o", "--output", help="Save image to the given filename.")


args = parser.parse_args()

with open(args.file1) as f:
    results = json.load(f)["results"]

mean1 = results[0]["mean"]
# label = results[0]["command"]
# times = results[0]["times"]
# num = len(times)
# nums = range(num)

with open(args.file2) as f:
    results = json.load(f)["results"]
mean2 = results[0]["mean"]

# label = results[0]["command"]
# times = results[0]["times"]
# num = len(times)
# nums = range(num)

diff =mean1-mean2
# fi = open('output.txt', 'w')

print(args.title, "difference:", diff)
print('percent:{:.2%}'.format(diff/mean1))
# print 'difference', diff
# fi.close()
