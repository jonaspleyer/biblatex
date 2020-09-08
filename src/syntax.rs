use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use unicode_xid::UnicodeXID;

/// A Bib(La)TeX file's most literal representation, with strings not yet parsed.
#[derive(Debug, Clone)]
pub struct BiblatexFile<'s> {
    /// TeX commands to be prepended to the document, only supported by BibTeX.
    pub preamble: String,
    /// The collection of citation keys and bibliography entries.
    pub entries: Vec<BiblatexEntry<'s>>,
    /// Map of reusable strings, only supported by BibTeX.
    pub strings: HashMap<&'s str, &'s str>,
}

/// A directly extracted Bib(La)TeX file entry, with strings not yet parsed.
#[derive(Debug, Clone)]
pub struct BiblatexEntry<'s> {
    /// The citation key.
    pub cite_key: &'s str,
    /// Denotes the type of bibliography item (e.g. `article`).
    pub entry_type: &'s str,
    /// Maps from field names to their associated values.
    pub fields: HashMap<&'s str, &'s str>,
}

/// Parse a biblatex file from a source string.
pub fn parse_file(src: &str, allow_bibtex: bool) -> BiblatexFile<'_> {
    BiblatexParser::new(src, allow_bibtex).parse()
}

/// Backing struct for parsing a Bib(La)TeX file into a `BiblatexFile` struct.
struct BiblatexParser<'s> {
    #[allow(unused)]
    allow_bibtex: bool,
    src: &'s str,
    mode: ParseMode,
    index: usize,
    comment: bool,
    iter: Peekable<Chars<'s>>,
    res: BiblatexFile<'s>,
}

/// Symbols that may be found when parsing a file.
#[derive(Debug, PartialEq)]
enum Symbols {
    Quotes,
    Braces,
}

#[derive(Debug, PartialEq)]
enum ParseMode {
    Outside,
    Type,
    KeyMode,
    PreambleMode,
    EntryMode,
}

impl<'s> BiblatexParser<'s> {
    /// Constructs a new parser.
    pub fn new(src: &'s str, allow_bibtex: bool) -> Self {
        Self {
            allow_bibtex,
            src,
            mode: ParseMode::Outside,
            index: 0,
            comment: false,
            iter: src.chars().peekable(),
            res: BiblatexFile {
                preamble: String::new(),
                entries: vec![],
                strings: HashMap::new(),
            },
        }
    }

    /// Parses the file, consuming the parser in the process.
    pub fn parse(mut self) -> BiblatexFile<'s> {
        while let Some(c) = self.eat() {
            if c == '@' && !self.comment {
                self.parse_entry();
            } else if c == '%' {
                self.comment = true;
            } else if c == '\n' || c == '\r' {
                self.comment = false;
            }
        }

        self.res
    }

    /// Parse a bibliography entry.
    fn parse_entry(&mut self) {
        self.mode = ParseMode::Type;

        let type_start = self.index;
        let mut type_end = self.index;
        let mut key_start: usize = 0;
        let mut key_end: usize = 0;
        let mut is_string = false;

        let mut entry_type = None;
        let mut fields = HashMap::new();

        while let Some(c) = self.peek() {
            match self.mode {
                ParseMode::Type => {
                    if is_ident(c, type_start == type_end) {
                        self.eat();
                        type_end = self.index;
                    } else {
                        entry_type = Some(&self.src[type_start .. type_end]);
                        if c.is_whitespace() {
                            self.eat();
                        } else if c == '{' {
                            self.eat();

                            let lower_type =
                                &self.src[type_start .. type_end].to_lowercase();

                            if lower_type == "string" {
                                self.mode = ParseMode::EntryMode;
                                is_string = true;
                            } else if lower_type == "preamble" {
                                self.mode = ParseMode::PreambleMode;
                            } else {
                                key_start = self.index;
                                key_end = self.index;
                                self.mode = ParseMode::KeyMode;
                            }
                        } else {
                            // TODO: Invalid
                            self.mode = ParseMode::Outside;
                            break;
                        }
                    }
                }

                ParseMode::KeyMode => {
                    if is_ident(c, key_start == key_end) {
                        self.eat();
                        key_end = self.index;
                    } else if c.is_whitespace() {
                        self.eat();
                    } else if c == ',' {
                        self.eat();
                        self.mode = ParseMode::EntryMode;
                    } else {
                        // TODO: Invalid
                        self.mode = ParseMode::Outside;
                        break;
                    }
                }

                ParseMode::PreambleMode => {
                    self.skip_ws();

                    // This does not allow string concatenation in preambles.
                    if c == '\"' {
                        self.eat();
                        while let Some(c) = self.eat() {
                            if c == '\"' {
                                break;
                            }
                            self.res.preamble.push(c);
                        }
                    }

                    self.mode = ParseMode::Outside;
                }

                ParseMode::EntryMode => {
                    self.skip_ws();
                    if self.peek() == Some('}') {
                        self.mode = ParseMode::Outside;
                        continue;
                    }
                    let s = self.read_prop().expect("Hallo");
                    if is_string {
                        self.res.strings.insert(s.0, s.1);
                    } else {
                        fields.insert(s.0, s.1);
                    }
                }
                _ => break,
            }
        }

        if !is_string {
            self.res.entries.push(BiblatexEntry {
                cite_key: &self.src[key_start .. key_end],
                entry_type: entry_type.unwrap_or_default(),
                fields,
            });
        }

        self.mode = ParseMode::Outside;
    }

    /// Read a field.
    fn read_prop(&mut self) -> Result<(&'s str, &'s str), ()> {
        self.skip_ws();

        let start = self.index;
        let mut end = self.index;

        while let Some(c) = self.peek() {
            if is_ident(c, start == end) {
                self.eat();
                end = self.index;
            } else {
                break;
            }
        }

        let name = &self.src[start .. end];
        while let Some(c) = self.eat() {
            if c == '=' {
                break;
            }
        }
        self.skip_ws();

        let mut stack: Vec<Symbols> = vec![];
        let val_start = self.index;
        let mut val_end = self.index;
        let mut escape = false;

        while let Some(c) = self.eat() {
            match c {
                '\\' => {
                    escape = true;
                    continue;
                }
                '{' if escape => {}
                '}' if escape => {}
                '"' if escape => {}
                ',' if stack.is_empty() => break,
                '}' if stack.is_empty() => {
                    self.mode = ParseMode::Outside;
                    break;
                }
                '"' if stack.last() == Some(&Symbols::Quotes) => {
                    stack.pop();
                }
                '"' if stack.is_empty() => stack.push(Symbols::Quotes),
                '{' => stack.push(Symbols::Braces),
                '}' => {
                    let x = stack.pop();
                    if x != Some(Symbols::Braces) {
                        return Err(());
                    }
                }
                _ => {}
            }
            escape = false;
            val_end = self.index;
        }

        Ok((name, &self.src[val_start .. val_end]))
    }

    /// Get the next character without advancing the parsing file iterator.
    fn peek(&mut self) -> Option<char> {
        self.iter.peek().copied()
    }

    /// Advance the parsing file iterator to the next non-whitespace or comment
    /// character.
    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c == '%' {
                self.comment = true;
                self.eat();
                continue;
            } else if c == '\n' || c == '\r' {
                self.comment = false;
            } else if self.comment {
                self.eat();
                continue;
            }

            if c.is_whitespace() || c == '\n' || c == '\r' {
                self.eat();
            } else {
                break;
            }
        }
    }

    /// Advance the parsing file iterator by one and return the consumed character.
    fn eat(&mut self) -> Option<char> {
        let c = self.iter.next()?;
        self.index += c.len_utf8();
        Some(c)
    }
}

/// Characters allowable in identifiers like cite keys.
fn is_ident(c: char, first: bool) -> bool {
    match c {
        '"' | '#' | '\'' | '(' | ')' | ',' | '=' | '{' | '}' | '%' | '\\' | '~' => false,
        ':' | '<' | '-' | '>' | '_' if !first => true,
        _ => {
            if first {
                c.is_xid_start()
            } else {
                c.is_xid_continue()
            }
        }
    }
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::*;

    fn parse(src: &str, allow_bibtex: bool) -> BiblatexFile<'_> {
        BiblatexParser::new(src, allow_bibtex).parse()
    }

    fn test_prop(key: &str, value: &str) -> String {
        let test = format!("@article{{test, {}={}}}", key, value);
        let bt = parse(&test, true);
        let article = &bt.entries[0];
        article.fields.get(key).expect("fail").to_string()
    }

    #[test]
    fn test_parse_article() {
        let file = "@article{haug2020,
            title = \"Great proceedings\\{\",
            year=2002,
            author={Haug, {Martin} and Haug, Gregor}}";

        let bt = parse(file, true);
        let article = &bt.entries[0];

        assert_eq!(article.entry_type, "article");
        assert_eq!(article.fields.get("title"), Some(&"\"Great proceedings\\{\""));
        assert_eq!(article.fields.get("year"), Some(&"2002"));
        assert_eq!(article.fields.get("author"), Some(&"{Haug, {Martin} and Haug, Gregor}"));
    }

    #[test]
    fn test_resolve_string() {
        let bt = parse("@string{BT = \"bibtex\"}", true);
        assert_eq!(bt.strings.get("BT"), Some(&"\"bibtex\""));
    }

    #[test]
    fn test_escape() {
        assert_eq!(test_prop("author", "{Mister A\\}\"B\"}"), "{Mister A\\}\"B\"}");
    }
}
