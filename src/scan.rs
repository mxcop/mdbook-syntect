use std::collections::VecDeque;

/// An event for parsing in a Markdown file.
#[derive(Debug)]
pub enum Event {
    /// A text block
    Begin(usize),
    TextEnd(usize),
    /// A code block
    BlockEnd(usize),
}

/// Scanner for text to identify block and inline math `Event`s.
#[derive(Debug)]
pub struct Scan<'a> {
    string: &'a str,
    bytes: &'a [u8],
    index: usize,
    /// Buffer for block and inline math `Event`s.
    pub events: VecDeque<Event>
}

impl<'a> Iterator for Scan<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.events.pop_front() {
                Some(item) => return Some(item),
                None => self.process_byte().ok()?,
            }
        }
    }
}

impl<'a> Scan<'a> {
    /// Set up a `Scan` for `string` with given delimiters.
    pub fn new(
        string: &'a str
    ) -> Self {
        Self {
            string,
            bytes: string.as_bytes(),
            index: 0,
            events: VecDeque::new()
        }
    }

    /// Scan, identify and store all `Event`s in `self.events`.
    #[allow(dead_code)]
    pub fn run(&mut self) {
        while let Ok(()) = self.process_byte() {}
    }

    /// Get byte currently pointed to. Returns `Err(())` if out of bound.
    fn get_byte(&self) -> Result<u8, ()> {
        self.bytes.get(self.index).map(|b| b.to_owned()).ok_or(())
    }

    /// Increment index.
    fn inc(&mut self) {
        self.index += 1;
    }

    /// Scan one byte, proceed process based on the byte.
    /// - `\` => skip one byte.
    /// - `` ` `` => call `process_backtick`.
    /// Return `Err(())` if no more bytes to process.
    fn process_byte(&mut self) -> Result<(), ()> {
        let byte = self.get_byte()?;
        self.inc();
        match byte {
            b'\\' => {
                self.inc();
            }
            b'`' => self.process_backtick()?,
            _ => (),
        }
        Ok(())
    }

    /// Process a `` ` ``
    /// Return `Err(())` if no more bytes to process.
    fn process_backtick(&mut self) -> Result<(), ()> {
        let mut n_back_ticks = 1;
        loop {
            let byte = self.get_byte()?;
            if byte == b'`' {
                self.inc();
                n_back_ticks += 1;
            } else {
                break;
            }
        }

        // Only recognize 3 back ticks as a codeblock.
        if n_back_ticks == 3 {
            self.events.push_back(Event::TextEnd(self.index - n_back_ticks));
            self.events.push_back(Event::Begin(self.index));
            self.index += self.string[self.index..]
                .find(&"`".repeat(n_back_ticks))
                .ok_or(())?;
            self.events.push_back(Event::BlockEnd(self.index));
            self.index += n_back_ticks;
            self.events.push_back(Event::Begin(self.index));
        }

        Ok(())
    }
}
