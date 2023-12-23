#!/usr/bin/python3
import argparse
import os

# Create the parser
parser = argparse.ArgumentParser()
parser.add_argument('--build', action='store_true', help='Maturin build and reinstall the lib.')
parser.add_argument('--test', action='store_true', help='Run Python tests.')

# Parse the command-line arguments
args = parser.parse_args()

if args.build:
    # Execute the commands
    os.system("pip3 uninstall compute_reputation_graph -y")
    os.system("rm -rf target/wheels/*")
    os.system("maturin build")
    os.system("pip3 install target/wheels/*")

    print("Build Ok!")

if args.test:
    from compute_reputation_graph import spend, compute

    v = spend("", 100)
    spend(v, 50)
    spend(v, 30)

    # m = compute_reputation_graph.compute(v)

    print("Tests Ok!")

print("\nEnd.")
