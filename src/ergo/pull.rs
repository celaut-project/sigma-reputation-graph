use ergo_node_interface::NodeInterface;



pub fn connect_to_node() {
    let api_key = "";
    let ip = "";
    let port = "9052"; 
    /*
Ports
mainnet 	testnet
API Port 	9053 	9052
P2P Port 	9030 	9022
address prefix 	(0) 0x00 	(16) 0x10
     */
    match NodeInterface::new(api_key, ip, port) {
        Ok(node) => println!("Current height: {}", node.current_block_height().unwrap_or_default()),
        Err(_) => println!("Error connecting to the node"),
    }
} 