import os

os.system("./install_python_lib.sh")

from compute_reputation_graph import spend, compute

v = spend("", 100)
spend(v, 50)
spend(v, 30)

# m = compute_reputation_graph.compute(v)
