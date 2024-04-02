use std::io::{ Result, Error, ErrorKind };

#[allow(async_fn_in_trait)]
pub trait SourceChainAPI {
    async fn finalized_block(&mut self) -> Result<u32>;
    /// Watermark should equal range_end unless max capacity of message batch is hit.
    async fn messages_in_range(&self, range_start: u32, range_end: u32, max_messages: u32) -> Result<(Vec<Message>, FirstNotProcessed)>;
}

/// The highest block for which not all messages have been processed
type FirstNotProcessed = u32;

/// Opaque data to be interpreted by the messanging target
type Message = Vec<u8>;

struct MinimalMockChain {
    finalized_block: u32,
}

impl MinimalMockChain {
    fn new() -> Self {
        MinimalMockChain {
            finalized_block: 0
        }
    }
}

impl SourceChainAPI for MinimalMockChain {
    async fn finalized_block(&mut self) -> Result<u32> {
        // Increment finalized block here to mimic internal progression
        self.finalized_block += 2;
        Ok(self.finalized_block)
    }

    async fn messages_in_range(&self, range_start: u32, range_end: u32, max_messages: u32) -> Result<(Vec<Message>, FirstNotProcessed)> {
        if range_end > self.finalized_block { return Err(Error::from(ErrorKind::Unsupported)) };
        let mut output = Vec::new();
        let mut first_not_processed = range_start;
        let mut messages_processed = 0;
        for i in range_start..=range_end {
            let block_messages = 5;
            // Don't process messages from block at all if they go over cap.
            if messages_processed + block_messages > max_messages { 
                first_not_processed = i; 
                break; 
            }
            for _ in 0..5 { 
                output.push(vec![4, 28, 45]);
                messages_processed += 1;
            };
            if i == range_end {
                first_not_processed = range_end + 1;
            }
        }
        Ok((output, first_not_processed))
    }
}

pub fn get_source_chain() -> impl SourceChainAPI {
    MinimalMockChain::new()
}

struct BadMockChain;

impl SourceChainAPI for BadMockChain {
    async fn finalized_block(&mut self) -> Result<u32> {
        Err(Error::from(ErrorKind::Other))
    }

    #[allow(unused_variables)]
    async fn messages_in_range(&self, range_start: u32, range_end: u32, max_messages: u32) -> Result<(Vec<Message>, FirstNotProcessed)> {
        Err(Error::from(ErrorKind::Other))
    }
}

pub fn get_bad_source_chain() -> impl SourceChainAPI {
    BadMockChain
}