use std::io;
use termion::input::{Keys, TermRead};
use termion::event::Key;
use termion::*; // {AsyncReader, async_stdin};

/** Struct to hold asynchronous IO provided by Termion
 */
pub struct AsyncIO{
    keys: Keys<AsyncReader>    
}

impl AsyncIO {
    /// Create a new AsyncIO for STDIN
    pub fn new() -> AsyncIO {
        AsyncIO{ keys: async_stdin().keys() }
    }
    
    /// Retrieve the next key from the IO buffer. This returns a Result<Key, io::Error>.
    pub fn get(&mut self) -> Result<Key, io::Error> {
        let k = self.keys.next();
        match k {
            Some(nk) => nk,
            None => Ok(Key::Null),
        }
    }
}
