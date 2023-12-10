pip3 uninstall compute_reputation_graph -y
rm -rf target/wheels/*
maturin build
pip3 install target/wheels/*