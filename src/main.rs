use mock_cross_chain_messenger::{self, source_chain::*};
use std::io::{ Result, Error, ErrorKind };
extern crate futures;
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use super::*;
    #[test]
    fn source_chain_handle_finalized_block_happy() {
        let mut source_chain_handle = get_source_chain();
        let maybe_finalized = block_on(source_chain_handle.finalized_block());
        match maybe_finalized {
            Ok(finalized) if finalized == 2 => (),
            _ => panic!(),
        }
    }

    #[test]
    fn source_chain_handle_finalized_block_unhappy() {
        let mut source_chain_handle = get_bad_source_chain();
        let maybe_finalized = block_on(source_chain_handle.finalized_block());
        match maybe_finalized {
            Err(error) => {
                match error.kind() {
                    ErrorKind::Other => (),
                    _ => panic!(),
                }
            },
            _ => panic!()
        }
    }

    // All messages from all blocks in specified range are fully processed
    #[test]
    fn source_chain_handle_messages_in_range_happy_path() {
        // Get finalized block from mock source chain
        let mut source_chain_handle = get_source_chain();
        let finalized = block_on(source_chain_handle.finalized_block()).expect("Always Ok in mock source chain");
        let (messages, first_unprocessed) = block_on(source_chain_handle.messages_in_range(0, finalized, 15)).expect("Okay with mock source in test");
        assert_eq!(messages.len(), 15);
        assert_eq!(first_unprocessed, finalized + 1);
    }

    // Full blocks of messages up to the cap are requested. We do not include
    // partial blocks of messages. We make the realistic assumption that on-chain
    // blocks will contain few enough messages that at least one block can be 
    // processed each call.
    #[test]
    fn source_chain_handle_messages_in_range_max_messages() {
        // Get finalized block from mock source chain
        let mut source_chain_handle = get_source_chain();
        let finalized = block_on(source_chain_handle.finalized_block()).expect("Always Ok in mock source chain");
        let (messages, first_unprocessed) = block_on(source_chain_handle.messages_in_range(0, finalized, 11)).expect("Okay with mock source in test");
        assert_eq!(messages.len(), 10);
        assert_eq!(first_unprocessed, finalized);
    }

    // The specified range exceeds the current finalized block
    #[test]
    fn source_chain_handle_messages_in_range_past_finalized() {
        let mut source_chain_handle = get_source_chain();
        let finalized = block_on(source_chain_handle.finalized_block()).expect("Always Ok in mock source chain");
        let error = block_on(source_chain_handle.messages_in_range(0, finalized + 1, 11)).expect_err("Should be err");
        match error.kind() {
            ErrorKind::Unsupported => (),
            _ => panic!()
        }
    }
}