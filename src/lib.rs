/*!
Accessing Agda's interaction mode via command line.
This library is created for [agda-tac](https://lib.rs/agda-tac).
It works with stable rust starting from 1.39.0.

This crate will work only with master-branch Agda until Agda 2.6.1 is released.
Tracking issue for the feature is [agda#4183][agda-issue].

## Basic usage

Invoke [`ReplState::start`](crate::agda::ReplState::start) and await the REPL to be available.
That's the wrapper of the Agda REPL's state.

Then you may:
+ Invoke [`ReplState::reload_file`](crate::agda::ReplState::reload_file) to reload the current file
+ Invoke [`ReplState::command`](crate::agda::ReplState::command) to send a command to Agda
+ Invoke [`ReplState::response`](crate::agda::ReplState::response)
  to await for the next response from Agda.
  + Note that Agda sends json to `agda-mode`.
    The deserialized json type is [`Resp`](crate::resp::Resp).

There are more utilities to access Agda, checkout the library documentation to see all of them.

## Implementation notes

This crate deserialize json via `serde_json`,
and do async process io handling via `tokio`.
*/

/// Common types (used in both input/output to Agda).
pub mod base;

/// Debugging utilities.
pub mod debug;

/// Response data types (output of Agda).
pub mod resp;

/// Agda commands (input to Agda).
pub mod cmd;

/// Invoke Agda in command line and interact with it via stdio.
pub mod agda;

#[cfg(test)]
mod tests;
