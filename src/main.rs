use reth::cli::Cli;
use reth_node_ethereum::EthereumNode;
use std::sync::Arc;

fn main() {
    Cli::parse()
        .run(|builder, args| async move {
            let handle = builder.node(EthereumNode::default()).launch().await?;

            handle.wait_for_node_exit().await
        })
        .unwrap();
}
