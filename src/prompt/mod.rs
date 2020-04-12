mod buffer;
mod char_string;
mod context;
mod navigation;
mod writer;

use crate::completion::{Completer, Suggester};

use buffer::Buffer;
use char_string::{CharString, CharStringView};
use context::Context;
use writer::Writer;

use crate::key_bindings::{action_for, Action, Direction, KeyBindings, Range, Scope};

pub struct Prompt {
    erase_after_read: bool,
    prompt: Option<CharString>,
    bindings: Option<KeyBindings>,
    completer: Option<Box<dyn Completer>>,
    suggester: Option<Box<dyn Suggester>>,
}

impl Prompt {
    #[must_use]
    pub fn new() -> Self {
        Prompt::default()
    }

    pub fn erase_after_read(&mut self, erase_after_read: bool) -> &mut Self {
        self.erase_after_read = erase_after_read;
        self
    }

    pub fn prompt(&mut self, prompt: &str) -> &mut Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn bindings(&mut self, bindings: KeyBindings) -> &mut Self {
        self.bindings = Some(bindings);
        self
    }

    pub fn completer(&mut self, completer: impl Completer + 'static) -> &mut Self {
        self.completer = Some(Box::new(completer));
        self
    }

    pub fn suggester(&mut self, suggester: impl Suggester + 'static) -> &mut Self {
        self.suggester = Some(Box::new(suggester));
        self
    }

    // TODO: Support crossterm async
    pub fn read_line(&mut self) -> Result<Option<String>, crate::ErrorKind> {
        let mut context = Context::new(
            self.erase_after_read,
            self.prompt.as_ref(),
            &self.completer,
            &self.suggester,
        )?;

        context.print()?;
        loop {
            if let crossterm::event::Event::Key(e) = crossterm::event::read()? {
                match action_for(self.bindings.as_ref(), e) {
                    Action::Write(c) => context.write(c)?,
                    Action::Delete(scope) => context.delete(scope)?,
                    Action::Move(range, direction) => context.move_cursor(range, direction)?,
                    Action::Complete(range) => context.complete(range)?,
                    Action::Suggest(direction) => context.suggest(direction)?,
                    Action::Noop => continue,
                    Action::Cancel => {
                        if context.is_suggesting() {
                            context.cancel_suggestion()?;
                        } else {
                            return Ok(None);
                        }
                    }
                    Action::Accept => return Ok(Some(context.buffer_as_string())),
                }
            }
        }
    }
}

impl Default for Prompt {
    fn default() -> Self {
        Self {
            erase_after_read: false,
            prompt: None,
            bindings: None,
            completer: None,
            suggester: None,
        }
    }
}
