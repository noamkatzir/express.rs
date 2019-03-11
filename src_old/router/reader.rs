use bytes::Bytes;

pub struct RouteReader {
    buffer: Bytes,
    pos: usize,
    state: State
}

impl RouteReader {
    pub fn new(buffer: Bytes) -> Self {
        RouteReader {
            buffer,
            pos: 0,
            state: State::Segment
        }
    }
}

enum State {
    Segment,
    Variable
}

pub enum Event {
    Segment(Bytes),
    Variable(Bytes)
}

impl Iterator for RouteReader {
    type Item = Event;

    fn next(&mut self) -> Option<Event> {
        let runner = &self.buffer[self.pos..];
        let mut start = 0;
        for (index, value) in runner.iter().enumerate() {
            match self.state {
                State::Segment if index == 0 && value == &b':' => {
                    self.state = State::Variable;
                    start = 1;
                }
                State::Segment if value == &b'/' => {
                    let res = Some(Event::Segment(Bytes::from(&runner[start..index])));
                    self.pos += index+1;
                    return res;
                },
                State::Variable if value == &b'/' => {
                    let res = Some(Event::Variable(Bytes::from(&runner[start..index])));
                    self.pos += index+1;
                    self.state = State::Segment;
                    return res;
                },
                _ => ()
            }
        };

        self.pos += runner.len();
        match self.state {
            State::Segment if runner.len() > 0 => {
                Some(Event::Segment(Bytes::from(&runner[start..])))
            }
            State::Variable if runner.len() > 0 => {
                Some(Event::Variable(Bytes::from(&runner[start..])))
            }

            _ => None
        }
    }
    
}