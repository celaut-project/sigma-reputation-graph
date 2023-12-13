import os

os.system("./install_python_lib.sh")

import compute_reputation_graph

v = compute_reputation_graph.spend("", 100)

print("\n On python3 -> ", v)

m = compute_reputation_graph.compute(v)