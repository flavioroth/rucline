#![deny(warnings, missing_docs, clippy::pedantic, clippy::all)]
#![warn(rust_2018_idioms)]
// TDO: Remove when ready
#![allow(missing_docs, clippy::missing_errors_doc)]

// TODO: update when Prompt is updated
// TODO: update when key_bindings is updated
// TODO: hooks section

//! Rucline, the Rust CLI Line reader, or simply "recline", is a cross-platform, UTF-8 compatible
//! line reader that provides hooks for autocompletion and tab suggestion. It supports advanced
//! editing [`actions`] and hooks for customizing the line reader behavior of the line reader making
//! it more flexible than simply reading from `stdin`.
//!
//! ### Basic usage:
//!
//! ```no_run
//! use rucline::completion;
//! use rucline::Prompt;
//!
//! if let Ok(Some(string)) = Prompt::from("What's you favorite website? ")
//!     // Add some tab completions (Optional)
//!     .suggester(completion::Basic::new(&[
//!         "https://www.rust-lang.org/",
//!         "https://docs.rs/",
//!         "https://crates.io/",
//!     ]))
//!     //Block until value is ready
//!     .read_line()
//! {
//!     println!("'{}' seems to be your favorite website", string);
//! }
//! ```
//!
//! # Actions
//!
//! Rucline's behavior can be customized and composed with use of [`actions`].
//!
//! There is a built-in set of default [`actions`] that will be executed upon user interaction.
//! These are meant to feel natural when coming from the default terminal, while also adding further
//! functionality and editing commands. For example, a few of the build-ins:
//! * `Tab`: cycle through completions
//! * `Shift` + `Tab`: cycle through completions in reverse
//! * `CTRL` + `W`: delete the current work
//! * `CTRL` + `J`: delete the beginning of the word
//! * `CTRL` + `K`: delete the end of the word
//! * `CTRL` + `U`: delete the line
//! * `CTRL` + `H`: delete the beggining of the line
//! * `CTRL` + `L`: delete the end of the line
//!
//! > See [`Action`][`actions`] for the full default behavior specification
//!
//! The default behavior can be customized by overriding user [`events`] with [`actions`]. Which
//! in turn can be serialized, stored, and loaded at run-time.
//!
//!
//! ### Overriding key bindings
//!
//! ```no_run
//! use rucline::{completion, Prompt};
//! use rucline::key_bindings::{Action, Event, KeyBindings, Range};
//! use crossterm::event::KeyCode;
//!
//! let mut bindings = KeyBindings::new();
//!
//! // Accept the full suggestions if `right` is pressed
//! bindings.insert(Event::from(KeyCode::Right), Action::Complete(Range::Line));
//!
//! if let Ok(Some(string)) = Prompt::from("What's you favorite website? ")
//!     // Add some likely values as completions
//!     .completer(completion::Basic::new(&[
//!         "https://www.rust-lang.org/",
//!         "https://docs.rs/",
//!         "https://crates.io/",
//!     ]))
//!     // Set the new key bindings as an override
//!     .bindings(bindings)
//!     //Block until value is ready
//!     .read_line()
//! {
//!     println!("'{}' seems to be your favorite website", string);
//! }
//! ```
//!
//! # Hooks
//!
//!
//!
//! [`crossterm`]: https://docs.rs/crossterm/
//! [`KeyBindings`]: key_bindings/index.html
//! [`actions`]: key_bindings/enum.Action.html
//! [`events`]: key_bindings/type.Event.html
//! [`prompt`]: prompt/index.html
pub mod completion;
pub mod key_bindings;
pub mod prompt;

pub use prompt::Prompt;

pub use crossterm::ErrorKind;

// TODO: rename `key_bindings`
