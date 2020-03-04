#!/usr/bin/python3

import matplotlib.pyplot as plt
import numpy as np
import json

delays = json.load(open("delay-data.json"))
mins = list(map(lambda l:None if len(l) == 0 else min(l), delays))
maxs = list(map(lambda l:None if len(l) == 0 else max(l), delays))
medians = list(map(lambda l:None if len(l) == 0 else np.median(l), delays))
perc_90s = list(map(lambda l:None if len(l) == 0 else np.percentile(l, 90), delays))
bandwidths = [json.load(open(f"tcplog-{i}.json")) for i in range(1, 4)] 

plt.subplot(2, 1, 1)
plt.title("Delay Statistics")
plt.xlabel("Time (s)")
plt.ylabel("Delay (s)")
plt.plot(mins, label='minimum delay')
plt.plot(maxs, label='maximum delay')
plt.plot(medians, label='median delay')
plt.plot(perc_90s, label='90th percentile delay')
plt.legend()

plt.subplot(2, 1, 2)
plt.title("Bandwidth Statistics")
plt.xlabel("Time (s)")
plt.ylabel("Bandwidth (byte)")
for i in range(3):
    plt.plot(bandwidths[i], label=f'node {i}')
plt.legend()
plt.show()
