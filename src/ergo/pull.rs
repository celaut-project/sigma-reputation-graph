use ergo_node_interface::{scanning::NodeError, scanning::Scan, NodeInterface};
use json::object;



pub fn connect_to_node() {
    let api_key = "";
    let ip = "127.0.0.1";
    let port = "9052"; 
    let ergo_tree_template_hash = "fafjdka";
    let reputation_token_label = "adfkjad";
    let node = NodeInterface::new(api_key, ip, port);
    /*
Ports
mainnet 	testnet
API Port 	9053 	9052
P2P Port 	9030 	9022
address prefix 	(0) 0x00 	(16) 0x10
     */
    match node {
        Ok(node) => {
            println!("Current block height: {}", node.current_block_height().unwrap_or_default());
            println!("Recomended fee: {}", node.get_recommended_fee(100000000, 1).unwrap_or_default());
            fetch_current_proofs(&node, ergo_tree_template_hash, reputation_token_label);
        },
        Err(_) => println!("Error connecting to the node"),
    }
} 

/// Function to fetch current proofs.
fn fetch_current_proofs(node: &NodeInterface, ergo_tree_template_hash: &str, reputation_token_label: &str) {
    // Generate the proposition (public key) for the change address.
    // Assuming `generate_pk_proposition` and `string_to_rendered` are defined elsewhere.
    /* 
     let pk_proposition = generate_pk_proposition(node.get_change_address()?);
     let serialized_pk_proposition = serialized_to_rendered(pk_proposition);
    */

    // Create the tracking rule JSON.
    let tracking_rule = object! {
        "predicate": "containsAsset",
        "assetId": "c95d7bd2c74986195bcebf516f619167d8235f3ded4260c0e3a7bc5824f72af8"
    };

    // Register the scan with the node.
    match Scan::register(&"FetchCurrentProofs".to_string(), tracking_rule, node) {
        Ok(scan) => {
            match scan.get_serialized_boxes() {
                Ok(s) => s.iter().map(|b| println!("box -> {}", b)).collect::<Vec<_>>(),
                Err(_) => {println!("Get serialized boxes error."); vec![]},
            }
        },
        Err(_) => {println!("Error getting the scan."); vec![]},
    };

}