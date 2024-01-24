#!/usr/bin/python3
import argparse
import os
import secrets
import string
from random import randint
from statistics import mean
from typing import Tuple, Callable, List
from time import time

# Create the parser
parser = argparse.ArgumentParser()
parser.add_argument('--build', action='store_true', help='Maturin build and reinstall the lib.')
parser.add_argument('--test', action='store_true', help='Run Python tests.')

# Parse the command-line arguments
args = parser.parse_args()

if args.build:
    # Execute the commands
    try:
        os.system("pip3 uninstall reputation_graph -y")
        os.system("rm target/wheels/*")
    except:
        pass
    os.system("/usr/bin/python3 -m maturin build")
    os.system("pip3 install target/wheels/*")

    print("Build Ok!")

if args.test:
    from reputation_graph import spend, compute

    def simple_test():
        print("\n\nSimple test")
        gh_pointer = "github.com"
        sdb_pointer = "surrealdb.com"
        rl_pointer = "rust-lang.org"
        v = spend("", 100)
        v1 = spend(v, 50)
        v12 = spend(v1, 15, sdb_pointer)
        v13 = spend(v1, 20)
        v1234 = spend(v13, 10, gh_pointer)
        v2 = spend(v, 30)
        v23 = spend(v2, 8, rl_pointer)

        assert 0.1 == compute(v, gh_pointer)
        assert 0.15 == compute(v, sdb_pointer)
        assert 0.08 == compute(v, rl_pointer)

        print("Works well!")

    def performance_test():
        print("\n\nPerformance test")

        MAX_DEPTH: int = 30
        MAX_AMOUNT: int = 1200
        MIN_AMOUNT: int = 10

        global proof_number
        proof_number = 0

        global leaf_number
        leaf_number = 0

        random_pointer: Callable[[], str] = \
            lambda: ''.join(secrets.choice(string.ascii_uppercase + string.digits) for _ in range(8))

        def random_proofs(parent: str = "", amount: int = 0, depth: int = 0) -> Tuple[str, List[str]]:
            global proof_number
            global leaf_number
            _pointer = random_pointer() if randint(0, 1) else None
            _pointers = [_pointer] if _pointer else []
            _v: str = spend(parent, amount, _pointer)
            proof_number += 1
            if depth >= MAX_DEPTH or amount <= MIN_AMOUNT:
                leaf_number += 1  # <- leaf
            else:
                while amount > MIN_AMOUNT:
                    _amount = randint(MIN_AMOUNT, max(int(amount/5), MIN_AMOUNT))
                    _, _p = random_proofs(parent=_v, amount=_amount, depth=depth + 1)
                    if _p:
                        _pointers.extend(_p)
                    amount -= _amount
            return _v, _pointers

        v, pointers = random_proofs(amount=MAX_AMOUNT)

        times = []
        for pointer in pointers:
            start = time()
            r = compute(v, pointer)
            end = time()
            time_lapse = end - start
            times.append(time_lapse)
            print(f"\nResult -> {r} with the pointer {pointer}, time lapse -> {time_lapse}")

        print(f"\n      Performance: "
              f"\n          nº proofs -> {proof_number}, "
              f"\n          nº leafs  -> {leaf_number}, "
              f"\n          avg time lapse -> {mean(times)}, "
              f"\n          SCORE -> {proof_number/mean(times)}.")


    performance_test()
    simple_test()
    print("\n\nTests Ok!")

print("\nEnd.")
