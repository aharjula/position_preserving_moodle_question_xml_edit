//! A diff-friendly XML-editor
//!
//! For when you need to query Moodle question.xml files and do small fixes.
//!
//! By diff friendly we mean here that we minimize the changes to the general structure of 
//! the file, leave comments and whitespace as it was outside the modified bits, thus making
//! version control commit diffs easier to read and the changes simpler to spot.
//! 
//! This tool does not do any entity-decoding or CDATA unwrapping on its own
//! when dealing with attributes or CDATA wrapped things you will need to either 
//! instruct the tool to do things or do everythign yoursef. This tools philosophy
//! is to try to avoid doing anythign withotu being asked so that no one needs to 
//! work around "helpful" features.

// Some extra question type specific structs are in other files.
pub mod stack;

/// References to values in content.
#[derive(Debug, PartialEq, Clone)]
pub struct ContentRef {
    /// The textual content of whatever was requested.
    pub content: String,
    /// The position, in bytes, in the XML of that.
    start: usize,
    /// And the matching end
    end: usize,
    /// During which search were these valid.
    version_num: usize
}
impl ContentRef {
    /// When accessing content that might be CDATA wrapped one might want it unwrapped.
    pub fn unwrap_cdata(&self) -> String {
        if self.content.contains("<![CDATA[") {
            // The silliest of parsers, but avoids doing anything extra.
            let mut out: String = String::new();
            let mut in_cdata: bool = false;
            let mut cdata_start: usize = 0;
            let mut cdata_end: usize = 0;

            for c in self.content.chars() {
                match c {
                    '<' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 0 {
                            cdata_start = 1;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    '!' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 1 {
                            cdata_start = 2;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    '[' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 2 {
                            cdata_start = 3;
                        } else if !in_cdata && cdata_start == 8 {
                            cdata_start = 0;
                            in_cdata = true;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    'C' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 3 {
                            cdata_start = 4;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    'D' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 4 {
                            cdata_start = 5;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    'A' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 5 {
                            cdata_start = 6;
                        } else if !in_cdata && cdata_start == 7 {
                            cdata_start = 8;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    'T' => {
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        if !in_cdata && cdata_start == 6 {
                            cdata_start = 7;
                        } else {
                            if cdata_start > 0 {
                                out.push_str(&"<![CDATA["[..cdata_start]);
                                cdata_start = 0;
                            }
                            out.push(c);
                        }
                    },
                    ']' => {
                        if cdata_start > 0 {
                            out.push_str(&"<![CDATA["[..cdata_start]);
                            cdata_start = 0;
                        }
                        if in_cdata && cdata_end == 0 {
                            cdata_end = 1;
                        } else if in_cdata && cdata_end == 1 {
                            cdata_end = 2;
                        } else {
                            if cdata_end == 1 {
                                out.push(']');
                                cdata_end = 0;
                            }
                            out.push(c);
                        }
                    },
                    '>' => {
                        if cdata_start > 0 {
                            out.push_str(&"<![CDATA["[..cdata_start]);
                            cdata_start = 0;
                        }
                        if in_cdata && cdata_end == 2 {
                            cdata_end = 0;
                            in_cdata = false;
                        } else {
                            if cdata_end > 0 {
                                out.push_str(&"]]>"[..cdata_end]);
                                cdata_end = 0;
                            }
                            out.push(c);
                        }
                    },
                    _ => {
                        if cdata_start > 0 {
                            out.push_str(&"<![CDATA["[..cdata_start]);
                            cdata_start = 0;
                        }
                        if cdata_end > 0 {
                            out.push_str(&"]]>"[..cdata_end]);
                            cdata_end = 0;
                        }
                        out.push(c);
                    }
                }
            }
            // Should it be collecting these durign the end.
            if cdata_start > 0 {
                out.push_str(&"<![CDATA["[..cdata_start]);
            }
            if cdata_end > 0 {
                out.push_str(&"]]>"[..cdata_end]);
            }

            out
        } else {
            self.content.clone()
        }
    }

    /// When dealign with attribute values one might want basic entities to be unescaped.
    pub fn basic_entity_decode(&self) -> String {
        self.content
            .replace("&lt;","<")
            .replace("&gt;",">")
            .replace("&quot;","\"")
            .replace("&apos;","'")
            .replace("&#xA;","\n")
            .replace("&#xD;","\r")
            .replace("&amp;","&")
    }
}


/// Types of references, if it is an atribute value we probaby need to entity convert things when writing.
/// Some types provide more, details and related objects.
///
/// Do note that when a type provides access to parts of itself you must not change the whole and its parts
/// at the same time.
#[derive(Debug, Clone)]
pub enum ContentType {
    /// Name of an atribute and a refrence to the content inside those quotes '' or "".
    AttributeValue(String, ContentRef),
    /// The content inside some element.
    ElementContent(ContentRef),
    /// Name of an element, a reference to the whole element, as well as a list of the attributes with the last element being the content of the element.
    Element(String, ContentRef, Vec<ContentType>),
    /// The tagname, reference to the format and as the first item in the list the contents of the text-element and the rest of the items are attachments as elements.
    MoodleTextElement(String, ContentRef, Vec<ContentType>)
}
impl ContentType {
    /// If this is an element or attribute and the attribute requested 
    /// is available return a ref to it.
    pub fn get_attr(self, name: String) -> Option<ContentRef> {
        match self {
            ContentType::AttributeValue(aname, value) => {
                if aname == name {
                    Some(value)
                } else {
                    None
                }
            },
            ContentType::Element(_, _, attributes_and_value) => {
                for item in attributes_and_value {
                    if let ContentType::AttributeValue(aname, value) = item {
                        if aname == name {
                            return Some(value);
                        }       
                    }
                }
                None
            },
            ContentType::MoodleTextElement(_, format, _) => {
                if name == "format".to_string() {
                    Some(format)
                } else {
                    None
                }
            },
            _ => {
                None
            }
        }
    }

    /// Extracts the primary content of various types of things.
    ///  - AttributeValue -> the value of the attribute
    ///  - Element -> the content unwrapped from ElementContent, if this is an `<empty/>`-tag then None.
    ///  - ElementContent -> the content
    ///  - MoodleTextElement -> the content of the `<text>`-element.
    pub fn get_content(self) -> Option<ContentRef> {
        match self {
            ContentType::AttributeValue(_, value) => {
                return Some(value);
            },
            ContentType::Element(_, _, attributes_and_value) => {
                if let ContentType::ElementContent(content) = attributes_and_value.last().unwrap() {
                    return Some(content.clone());
                }
            },
            ContentType::ElementContent(content) => {
                return Some(content);
            },
            ContentType::MoodleTextElement(_, _, content_and_files) => {
                if let ContentType::ElementContent(content) = content_and_files.first().unwrap() {
                    return Some(content.clone());
                }
            }
        }
        None
    }
}


/// Results from question identification.
#[derive(Debug)]
pub struct Question {
    /// The index of this question in the document. Use this when querying elements inside questions.
    pub index: usize,
    /// The type of the question, from the `type` attribute of the `<question>`-element.
    pub qtype: String,
    /// Name element contents.
    pub name: ContentRef,
    /// Content reference ot the whole `<question>`-element. For when you want to copy or remove the whole question from the document.
    pub whole_element: ContentRef
}

/// A change to be executed.
pub struct Change {
    /// The position that will change.
    pub position: ContentRef,
    /// The new content of that position.
    pub new_content: String
}
impl Change {
    /// Create a Change, entity escaped version of the given value. For when modifying attribute values.
    pub fn attribute_escaped_version(position: ContentRef, value: String) -> Change {
        let escaped: String = value.replace("&","&amp;")
            .replace("<","&lt;")
            .replace(">","&gt;")
            .replace("\"","&quot;")
            .replace("'","&apos;")
            .replace("\n","&#xA;")
            .replace("\r","&#xD;");
        Change {
            position: position,
            new_content: escaped
        }
    }

    /// Create a Change, and CDATA wrap the content if need be.
    ///
    /// Note that this is not a proper CDATA escape dealing with CDATA parts inside CDATA, 
    /// it matches the Moodle one used in for example STACK: 
    /// <https://github.com/moodle/moodle/blob/d7bb4636df0cdb40b7eb6af32abb4ee6615fc78f/question/format/xml/format.php#L1027>
    pub fn cdata_wrapped_version(position: ContentRef, value: String) -> Change {
        // https://www.php.net/manual/en/function.htmlspecialchars.php
        if value.contains("&") || value.contains("\"") || value.contains("'") || value.contains("<") || value.contains(">") {
            Change {
                position: position,
                new_content: format!("<![CDATA[{value}]]>")
            }
        } else {
            Change {
                position: position,
                new_content: value
            }
        }
    }

    /// Just create a Change struct
    pub fn new(position: ContentRef, value: String) -> Change {
        Change {
            position: position,
            new_content: value
        }
    }
}


/// The parser object, holding the current in-memory version of the document and keeping track of changes that are to be made to it.
pub struct QParser {
    /// Current text content.
    content: String,
    /// Version number of the document currently held in memory, any content refs pointing to different versions are invalid and cannot be used to target changes.
    version_num: usize,
    /// Changes currently waiting for execution. We collect multiple of them and execute them at the same time so that everyone can keep using the original references for positioning, before actually executing and those lose meaning.
    changes: Vec<Change>
}
impl QParser {
    /// Simply initialise a parser from the contents of a file.
    pub fn load_xml_file(file_name: String) -> Result<QParser, String> {
        let content = std::fs::read_to_string(file_name.clone()).expect("Problem reading the XML file.");

        // Check if it parses.
        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        match roxmltree::Document::parse_with_options(&content, opt) {
            Ok(_doc) => {
                Ok(QParser {
                    content: content,
                    version_num: 0,
                    changes: Vec::new()
                })
            },
            Err(_e) => {
                Err(String::from("Errors parsing the original document."))
            }
        }
    }

    /// Parse a String that has appeared from somewhere.
    pub fn from_string(content: String) -> Result<QParser, String> {
        // Check if it parses.
        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        match roxmltree::Document::parse_with_options(&content, opt) {
            Ok(_doc) => {
                Ok(QParser {
                    content: content,
                    version_num: 0,
                    changes: Vec::new()
                })
            },
            Err(_e) => {
                Err(String::from("Errors parsing the original document."))
            }
        }
    }

    /// Save the current version to a file. Note that will flush the change-buffer before doing so.
    ///
    /// Should the currently held version not be valid XML this will not save to a file. You can
    /// still read the current content with `get_current_content()` and use other means to write 
    /// it anywhere you wish.
    pub fn save_to_file(&mut self, file_name: String) -> Result<(),String> {
        self.execute_changes();
        // We can fail fro many reasons, IO-reasons will panic, but
        // our own insistence on valid XML will give errors.
        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        match roxmltree::Document::parse_with_options(&self.content, opt) {
            Ok(_doc) => {
                match std::fs::write(file_name.clone(), self.content.clone()) {
                    Ok(_) => {
                        Ok(())
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        Err(String::from("Failure writing file."))
                    }
                }
            },
            Err(_e) => {
                Err(String::from("Will not write to a file due to content being broken."))
            }
        }
    }

    /// Mainly for tests and curious minds.
    pub fn get_current_content(&self) -> String {
        self.content.clone()
    }

    /// Provides a list of questions present in the document. Only gives their types and positions
    /// not names or any other details. Mainly used to identify the indices one wants to act on by type.
    pub fn find_questions(&mut self) -> Vec<Question> {
        self.execute_changes();

        let mut result: Vec<Question> = Vec::new();

        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        let doc = match roxmltree::Document::parse_with_options(&self.content, opt) {
            Ok(doc) => doc,
            Err(e) => {
                panic!("Error: {}.", e);
            }
        };

        let mut qn: usize = 0;
        for node in doc.descendants() {
            if node.is_element() && node.tag_name().name() == "question" {
                match node.attribute("type") {
                    Some(qtype) => {
                        if qtype == "category" {
                            // We ignore these.
                            continue;
                        }

                        let mut name: Option<ContentRef> = None;
                        for n in node.children() {
                            if n.is_element() && n.tag_name().name() == "name" {
                                let text = &self._get_elements(n, vec!["text".to_string()])[0];
                                if let ContentType::Element(_,_,items) = text {
                                    let econtent = &items[0];
                                    if let ContentType::ElementContent(c) = econtent {
                                        name = Some(c.clone());
                                    }
                                }
                                break;
                            }
                        }

                        let wholetag: String = self.content[node.range().start..node.range().end].to_string();
                        result.push(Question { qtype: qtype.to_string(), index: qn, name: name.expect("Questions should have names"), whole_element: ContentRef {
                                content: wholetag,
                                start: node.range().start,
                                end: node.range().end,
                                version_num: self.version_num
                            }
                        });
                        qn = qn + 1;
                    },
                    None => {
                        println!("Typeless question-elements are being ignored.");
                    }
                }
            }
        }
        return result;
    }

    /// Executes registered changes. Basically, handles them in order.
    /// DOES not write them out to any file only keeps them in memory.
    pub fn execute_changes(&mut self) {
        // Then do things, a single change happens often enough to be handled seaprately.
        match self.changes.len() {
            0 => {
                // Nothing to do.
                return;    
            },
            1 => {
                let c: Change = self.changes.pop().unwrap();
                let mut new_content: String = self.content[..c.position.start].to_string();
                new_content.push_str(&c.new_content);
                let end: String = self.content[c.position.end..].to_string();
                new_content.push_str(&end);
                self.content = new_content;
                self.changes.clear();
                self.version_num = self.version_num + 1;
            },
            _ => {
                self.changes.sort_by(|a,b| b.position.start.cmp(&a.position.start));
                for c in &self.changes {
                    let mut new_content: String = self.content[..c.position.start].to_string();
                    new_content.push_str(&c.new_content);
                    let end: String = self.content[c.position.end..].to_string();
                    new_content.push_str(&end);
                    self.content = new_content;
                }
                self.changes.clear();
                self.version_num = self.version_num + 1;
            }
        }
    }

    /// Adds a change to the change buffer, to be executed at some later moment.
    ///
    /// Note that this function will panic if given a change that overlaps any
    /// previously given non executed one. Use of this toolset needs to execute changes
    /// in suitable batches and order so that this does not happen.
    pub fn register_change(&mut self, change: Change) {
        if change.position.version_num != self.version_num {
            panic!("Use of a content-reference to a stale search result detected.");
        }
        if self.changes.is_empty() {
            self.changes.push(change);
        } else {
            let mut good = true;
            for existing in &self.changes {
                // Is this inside it?
                if change.position.start < existing.position.end && change.position.start >= existing.position.start {
                    good = false;
                    break;
                } else if change.position.end <= existing.position.end && change.position.end > existing.position.start {
                    good = false;
                    break;
                }
                // Is it inside this?
                if existing.position.start < change.position.end && existing.position.start >= change.position.start {
                    good = false;
                    break;
                } else if existing.position.end <= change.position.end && existing.position.end > change.position.start {
                    good = false;
                    break;
                }
            }
            if !good {
                panic!("Overlap of uncommitted changes, cannot continue.");
            }
            self.changes.push(change);
        }
    }


    /// Searches of various elements from within a singular question.
    ///
    /// You may name as many tag-names you want, and the tool tries to 
    /// return sensible ContentType objects describing things.
    pub fn get_elements(&mut self, qnum: usize, tagnames: Vec<String>) -> Vec<ContentType> {
        self.execute_changes();

        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        let doc = match roxmltree::Document::parse_with_options(&self.content, opt) {
            Ok(doc) => doc,
            Err(e) => {
                panic!("Error: {}.", e);
            }
        };

        let mut qn: usize = 0;
        for node in doc.descendants() {
            if node.is_element() && node.tag_name().name() == "question" {
                match node.attribute("type") {
                    Some(qtype) => {
                        if qtype == "category" {
                            // We ignore these.
                            continue;
                        }
                        if qn == qnum {
                            return self._get_elements(node, tagnames)
                        }
                        qn = qn + 1;
                    },
                    None => {
                        // Ignore here.
                    }
                }
            }
        }
        panic!("{}",format!("Only {qn} questions, but was trying to get index {qnum}."));
    }

    fn _get_elements(&self, qnode: roxmltree::Node, tagnames: Vec<String>) -> Vec<ContentType> {
        // qnode here is a question element.
        let mut result: Vec<ContentType> = Vec::new();

        for node in qnode.descendants() {
            if node.is_element() && tagnames.contains(&node.tag_name().name().to_string()) {
                // Is this element something with format and an internal text element as well as attachements?
                let mut maybe_moodle_text_node: Option<ContentRef> = None;
                let mut surely_moodle_text_node = false;
                let mut parts: Vec<ContentType> = Vec::new();
                // First attributes if any.
                for attr in node.attributes() {
                    // We need the position of the content inside quotes.
                    let start = &self.content[attr.range().start..].find(|c| c == '"' || c == '\'').unwrap() + attr.range().start + 1;
                    let quotetype: char = self.content[start-1..].chars().nth(0).unwrap();
                    let end = &self.content[start..].find(|c| c == quotetype).unwrap() + start;
                    let rawattr: String = self.content[start..end].to_string();
                    let cr: ContentRef = ContentRef {
                        content: rawattr,
                        start: start,
                        end: end,
                        version_num: self.version_num
                    };
                    if attr.name().to_string() == "format" {
                        maybe_moodle_text_node = Some(cr.clone());
                    }
                    let v = ContentType::AttributeValue (attr.name().to_string(), cr);
                    parts.push(v);
                }

                // Then the content, if any...
                let wholetag: String = self.content[node.range().start..node.range().end].to_string();
                if &wholetag[node.range().end-node.range().start-2..] != "/>" && node.children().count() > 0 {
                    // So we can extract the internal bit, thus we have content.
                    let mut children = node.children();
                    // The first child will give us the start of the range and so on.
                    let first = children.next().unwrap();
                    let last = match children.last() {
                        None => {
                            first
                        }, 
                        Some(n) => {
                            n
                        }
                    };
                    // Check for that MoodleTextConstruct.
                    if maybe_moodle_text_node != None {
                        for n in node.children() {
                            if n.is_element() && n.tag_name().name().to_string() == "text".to_string() {
                                surely_moodle_text_node = true;
                                break;
                            }
                        }
                    }

                    let inner: String = self.content[first.range().start..last.range().end].to_string();
                    let v = ContentType::ElementContent (ContentRef {
                        content: inner.to_string().clone(),
                        start: first.range().start,
                        end: last.range().end,
                        version_num: self.version_num
                    });
                    parts.push(v);
                } else if &wholetag[node.range().end-node.range().start-2..] != "/>" {
                    // Not an "empty"-tag but still empty... We need to identify the position of that "><".
                    let pos = node.range().start + wholetag.find("><").unwrap() + 1;
                    let v = ContentType::ElementContent (ContentRef {
                        content: "".to_string(),
                        start: pos,
                        end: pos,
                        version_num: self.version_num             
                    });
                    parts.push(v);
                }
                // Certain common constructs require special handling.
                if !surely_moodle_text_node {
                    result.push(ContentType::Element(node.tag_name().name().to_string(), ContentRef {
                        content: wholetag,
                        start: node.range().start,
                        end: node.range().end,
                        version_num: self.version_num
                    }, parts));
                } else {
                    // Recurse those inner elements
                    let mut els: Vec<ContentType> = self._get_elements(node, vec!["file".to_string()]);
                    // We unwrap the text-element and place it as the first element in the list of parts, for ease of access.
                    let the_text_element: ContentType = self._get_elements(node, vec!["text".to_string()])[0].clone();
                    if let ContentType::Element(_name, _whole, prts) = the_text_element {
                        // Always have the text-elements content as the first in that list that might contain attachemnt files.
                        els.insert(0, prts.last().unwrap().clone());
                    }
                    result.push(ContentType::MoodleTextElement(node.tag_name().name().to_string(), maybe_moodle_text_node.unwrap(), els));
                }
            }
        }

        return result;
    }
}


