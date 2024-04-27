# <link>Reputation Graph</link>

This project designed to store an acyclic graph composed of objects and calculate the reputation of an object within the graph. It utilizes <link>Rust</link> and <link>SurrealDB</link>, providing support for both <link>WebAssembly (WASM)</link> and <link>Python 3</link> through <link>PyO3</link>.

## Features
- Storage of acyclic graph structures
- Reputation calculation for individual objects within the graph
- Utilizes <link>Rust</link> for high-performance and reliability
- Compatible with <link>SurrealDB</link> for efficient data management

## Integration
The project offers integration with the following environments:
- **<link>WebAssembly (WASM)</link>:** Enables usage within web applications for client-side computation.
- **<link>Python 3</link>:** Provides a <link>Python</link> interface through <link>PyO3</link>, allowing seamless integration with <link>Python</link> projects and workflows.

## Usage
To use the **<link>reputation-graph</link>** project, follow these steps:
1. Install the necessary dependencies and tools.
2. Integrate the project into your desired environment (<link>WASM</link> or <link>Python 3</link>).
3. Utilize the provided functionalities to store acyclic graphs and perform reputation calculations for objects within the graph.

## Dependencies
The project relies on the following components:
- <link>Rust</link>: Provides the core functionality and performance optimizations.
- <link>SurrealDB</link>: Offers efficient data management and storage for the acyclic graph structures.

## Getting Started
To get started with the **<link>reputation-graph</link>** project, refer to the documentation and examples provided in the respective environment (<link>WASM</link> or <link>Python 3</link>).


## Install

maturin build --release --features pyo3-bindings

wasm-pack build --release --target web -- --features wasm-bindings


### For Python3.11 - Linux - x86_64
`pip3 install https://github.com/reputation-systems/sigma-reputation-graph/raw/master/target/wheels/sigma_reputation_graph-0.0.0-cp311-cp311-manylinux_2_35_x86_64.whl`
