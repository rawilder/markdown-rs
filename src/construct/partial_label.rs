//! Label occurs in [definition][] and label end.
//!
//! They’re formed with the following BNF:
//!
//! ```bnf
//! ; Restriction: maximum `999` codes allowed between brackets.
//! ; Restriction: no blank lines.
//! ; Restriction: at least 1 non-space and non-eol code must exist.
//! label ::= '[' *( label_text | label_escape ) ']'
//! label_text ::= code - '[' - '\\' - ']'
//! label_escape ::= '\\' [ '[' | '\\' | ']' ]
//! ```
//!
//! The maximum allowed size of the label, without the brackets, is `999`
//! (inclusive), which is defined in
//! [`LINK_REFERENCE_SIZE_MAX`][link_reference_size_max].
//!
//! Labels can contain line endings and whitespace, but they are not allowed to
//! contain blank lines, and they must not be blank themselves.
//!
//! The label is interpreted as the [string][] content type.
//! That means that [character escapes][character_escape] and
//! [character references][character_reference] are allowed.
//!
//! > 👉 **Note**: this label relates to, but is not, the initial “label” of
//! > what is know as a reference in markdown:
//! >
//! > | Kind      | Link     | Image     |
//! > | --------- | -------- | --------- |
//! > | Shortcut  | `[x]`    | `![x]`    |
//! > | Collapsed | `[x][]`  | `![x][]`  |
//! > | Full      | `[x][y]` | `![x][y]` |
//! >
//! > The 6 above things are references, in the three kinds they come in, as
//! > links and images.
//! > The label that this module focusses on is only the thing that contains
//! > `y`.
//! >
//! > The thing that contains `x` is not a single thing when parsing markdown,
//! > but instead constists of an opening (label start (image) or label start
//! > (link)) and a closing (label end), so as to allow further phrasing such
//! > as code (text) or attention.
//!
//! ## References
//!
//! *   [`micromark-factory-label/index.js` in `micromark`](https://github.com/micromark/micromark/blob/main/packages/micromark-factory-label/dev/index.js)
//!
//! [definition]: crate::construct::definition
//! [string]: crate::content::string
//! [character_escape]: crate::construct::character_escape
//! [character_reference]: crate::construct::character_reference
//! [link_reference_size_max]: crate::constant::LINK_REFERENCE_SIZE_MAX
//!
//! <!-- To do: link label end, label starts. -->

// To do: pass token types in.

use crate::constant::LINK_REFERENCE_SIZE_MAX;
use crate::construct::partial_space_or_tab::space_or_tab_opt;
use crate::tokenizer::{Code, State, StateFnResult, TokenType, Tokenizer};
use crate::util::link::link;

/// Configuration.
///
/// You must pass the token types in that are used.
#[derive(Debug)]
pub struct Options {
    /// Token for the whole label.
    pub label: TokenType,
    /// Token for the markers.
    pub marker: TokenType,
    /// Token for the string (inside the markers).
    pub string: TokenType,
}

/// State needed to parse labels.
#[derive(Debug)]
struct Info {
    /// Whether we’ve seen our first `ChunkString`.
    connect: bool,
    /// Whether there are non-blank characters in the label.
    data: bool,
    /// Number of characters in the label.
    size: usize,
    /// Configuration.
    options: Options,
}

/// Before a label.
///
/// ```markdown
/// |[a]
/// ```
pub fn start(tokenizer: &mut Tokenizer, code: Code, options: Options) -> StateFnResult {
    match code {
        Code::Char('[') => {
            let info = Info {
                connect: false,
                data: false,
                size: 0,
                options,
            };
            tokenizer.enter(info.options.label.clone());
            tokenizer.enter(info.options.marker.clone());
            tokenizer.consume(code);
            tokenizer.exit(info.options.marker.clone());
            tokenizer.enter(info.options.string.clone());
            (State::Fn(Box::new(|t, c| at_break(t, c, info))), None)
        }
        _ => (State::Nok, None),
    }
}

/// In a label, at something.
///
/// ```markdown
/// [|a]
/// [a|]
/// ```
fn at_break(tokenizer: &mut Tokenizer, code: Code, mut info: Info) -> StateFnResult {
    match code {
        Code::None | Code::Char('[') => (State::Nok, None),
        Code::Char(']') if !info.data => (State::Nok, None),
        _ if info.size > LINK_REFERENCE_SIZE_MAX => (State::Nok, None),
        Code::Char(']') => {
            tokenizer.exit(info.options.string.clone());
            tokenizer.enter(info.options.marker.clone());
            tokenizer.consume(code);
            tokenizer.exit(info.options.marker.clone());
            tokenizer.exit(info.options.label);
            (State::Ok, None)
        }
        _ => {
            tokenizer.enter(TokenType::ChunkString);

            if info.connect {
                let index = tokenizer.events.len() - 1;
                link(&mut tokenizer.events, index);
            } else {
                info.connect = true;
            }

            label(tokenizer, code, info)
        }
    }
}

/// After a line ending.
///
/// ```markdown
/// [a
/// |b]
/// ```
fn line_start(tokenizer: &mut Tokenizer, code: Code, info: Info) -> StateFnResult {
    tokenizer.go(space_or_tab_opt(), |t, c| line_begin(t, c, info))(tokenizer, code)
}

/// After a line ending, after optional whitespace.
///
/// ```markdown
/// [a
/// |b]
/// ```
fn line_begin(tokenizer: &mut Tokenizer, code: Code, info: Info) -> StateFnResult {
    match code {
        // Blank line not allowed.
        Code::CarriageReturnLineFeed | Code::Char('\r' | '\n') => (State::Nok, None),
        _ => at_break(tokenizer, code, info),
    }
}

/// In a label, in text.
///
/// ```markdown
/// [a|b]
/// ```
fn label(tokenizer: &mut Tokenizer, code: Code, mut info: Info) -> StateFnResult {
    match code {
        Code::None | Code::Char('[' | ']') => {
            tokenizer.exit(TokenType::ChunkString);
            at_break(tokenizer, code, info)
        }
        _ if info.size > LINK_REFERENCE_SIZE_MAX => {
            tokenizer.exit(TokenType::ChunkString);
            at_break(tokenizer, code, info)
        }
        Code::CarriageReturnLineFeed | Code::Char('\r' | '\n') => {
            tokenizer.consume(code);
            info.size += 1;
            tokenizer.exit(TokenType::ChunkString);
            (State::Fn(Box::new(|t, c| line_start(t, c, info))), None)
        }
        Code::VirtualSpace | Code::Char('\t' | ' ') => {
            tokenizer.consume(code);
            info.size += 1;
            (State::Fn(Box::new(|t, c| label(t, c, info))), None)
        }
        Code::Char('/') => {
            tokenizer.consume(code);
            info.size += 1;
            if !info.data {
                info.data = true;
            }
            (State::Fn(Box::new(|t, c| escape(t, c, info))), None)
        }
        Code::Char(_) => {
            tokenizer.consume(code);
            info.size += 1;
            if !info.data {
                info.data = true;
            }
            (State::Fn(Box::new(|t, c| label(t, c, info))), None)
        }
    }
}

/// After `\` in a label.
///
/// ```markdown
/// [a\|[b]
/// ```
fn escape(tokenizer: &mut Tokenizer, code: Code, mut info: Info) -> StateFnResult {
    match code {
        Code::Char('[' | '\\' | ']') => {
            tokenizer.consume(code);
            info.size += 1;
            (State::Fn(Box::new(|t, c| label(t, c, info))), None)
        }
        _ => label(tokenizer, code, info),
    }
}