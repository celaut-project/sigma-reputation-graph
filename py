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
        os.system("pip3 uninstall sigma_reputation_graph -y")
        os.system("rm -rf target/wheels/*")
    except:
        pass
    os.system("maturin build --strip --release --features pyo3-bindings")
    os.system("pip3 install target/wheels/*")

    print("Build Ok!")

if args.test:
    from sigma_reputation_graph import spend, compute

    def simple_test():
        print("\n\nSimple test")
        gh_pointer = "github.com"
        sdb_pointer = "surrealdb.com"
        rl_pointer = "rust-lang.org"
        spend("", 100, None, None)
        spend("", 50, None, None)
        spend("", 15, sdb_pointer, None)
        spend("", 20, None, None)
        spend("", 10, gh_pointer, None)
        spend("", 30, None, None)
        spend("", 8, rl_pointer, None)

        assert 0.04291845493562232 == compute("", gh_pointer, None)
        assert 0.06437768240343347 == compute("", sdb_pointer, None)
        assert 0.034334763948497854 == compute("", rl_pointer, None)

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
            spend(parent, amount, _pointer, None)
            proof_number += 1
            if depth >= MAX_DEPTH or amount <= MIN_AMOUNT:
                leaf_number += 1  # <- leaf
            else:
                while amount > MIN_AMOUNT:
                    _amount = randint(MIN_AMOUNT, max(int(amount/5), MIN_AMOUNT))
                    _, _p = random_proofs(amount=_amount, depth=depth + 1)
                    if _p:
                        _pointers.extend(_p)
                    amount -= _amount
            return "", _pointers

        v, pointers = random_proofs(amount=MAX_AMOUNT)

        times = []
        for pointer in pointers:
            start = time()
            r = compute(None, pointer, None)
            end = time()
            time_lapse = end - start
            times.append(time_lapse)
            print(f"\nResult -> {r} with the pointer {pointer}, time lapse -> {time_lapse}")

        print(f"\n      Performance: "
              f"\n          nº proofs -> {proof_number}, "
              f"\n          nº leafs  -> {leaf_number}, "
              f"\n          avg time lapse -> {mean(times)}, "
              f"\n          SCORE -> {proof_number/mean(times)}.")

    os.system("rm -rf reputation.db")
    simple_test()
    performance_test()
    os.system("rm -rf reputation.db")
    print("\n\nTests Ok!")

print("\nEnd.")