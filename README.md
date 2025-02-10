# position_preserving_moodle_question_xml_edit well that is a name...

This is a library for those of us that need to do small modifications on Moodle question.xml files and don't want to reimplement the parsing every time. Or don't yet trust that AI would implement it safely every time.

The thing that makes this different from other XML-parsers is that when we modify things we are not actually modifying a DOM of any sorts, we simply write Strings on top of the original items positions, thus leaving the rest of the document alone. Not touching the rest of the XML-document makes this somewhat diff-friendly and will probably keep your version control diffs readable.

## For who?

This will primarily target [STACK](https://stack-assessment.org/)-questions but will happily deal with any other question-formats it sees. In particular, this will probably be of use to those building quick modification scripts working on their [gitsync](https://github.com/maths/moodle-qbank_gitsync)-clones of large question-banks.

## Current state of development

This is the first release 0.1.0, hardly tested or used and written as pretty much the first rust thing of the author, so be careful and don't get too revulsed by what you see in the code.

# Docs...

Check the tests, those should tell everything worth knowing. Or compile the rustdoc for a list of things.


# Example

Sometimes you have extra file-attachements in the XML, due to editor issues and no cleanup action taken. In the case, of Moodle question.xml those files are stored as base64 encoded from inside `<file>`-elements and they are referenced by their name and a path (both avaialble as attributes of that `<file>`-element) and a specific prefix `@@PLUGINFILE@@` in the text. This is how you would build a tool that takes a file and looks for all the `<file>`-elements and then deletes those it cannot find references to in the other content of the question.

First start a project, and include this library, also urlencode as this task needs that:
```
> cargo new attachments
> cd attachments
> cargo add position_preserving_moodle_question_xml_edit
> cargo add urlencoding
```
Then modify the `src/main.rs` file to contain this:
```
use position_preserving_moodle_question_xml_edit::{QParser, Question, ContentType, Change};

fn main() {
    // Simple arguments.
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        println!("This tool requires some filenames as arguments.");
        return;
    }
    let files: Vec<&String> = args[1..].into_iter().filter(|a| **a != "--execute".to_string()).collect();
    let execute: bool = args.len()-1 != files.len();

    // Then process the files.
    for file_name in files {
        println!("Checking {}:", file_name.clone());
        let mut parser = QParser::load_xml_file(file_name.clone()).expect("Something bad with the file or file-name.");

        let questions: Vec<Question> = parser.find_questions();
        let mut total_removed: usize = 0;
        for question in &questions {
            println!(" Question {:>3}/{} '{}':", question.index + 1, questions.len(), question.name.content.clone());
            let file_elements = parser.get_elements(question.index, vec!["file".to_string()]);
            if file_elements.len() == 0 {
                if question.whole_element.content.contains("@@PLUGINFILE@@") {
                    println!("   WARNING question references local pluginfiles but has none defined.");
                } else {
                    println!("   Nothing to do.");
                }
            } else {
                for file_element in &file_elements {
                    if let ContentType::Element(_name, whole_element_ref, _attributes_and_content) = file_element {
                        let attachment_path: String = file_element.clone().get_attr("path".to_string()).expect("File element must have a 'path' attribute.").content;
                        let attachment_name: String = file_element.clone().get_attr("name".to_string()).expect("File element must have a 'name' attribute.").content;
                        let test = format!("@@PLUGINFILE@@{attachment_path}{attachment_name}");
                        // Note that as these are used as URLs they may have been urlencoded, well partly.
                        let test2 = format!("@@PLUGINFILE@@{attachment_path}{}", urlencoding::encode(&attachment_name));
                        if question.whole_element.content.contains(test.as_str()) || question.whole_element.content.contains(test2.as_str()) {
                            println!("   Does reference {}", test);
                        } else {
                            total_removed = total_removed + whole_element_ref.content.bytes().count();
                            if execute {
                                // Remove by replaceing element with empty.
                                let change = Change::new(whole_element_ref.clone(), "".to_string());
                                parser.register_change(change);
                                println!("   REMOVING: {}", test);
                            } else {
                                println!("   WOULD REMOVE: {}", test);
                            }
                        }


                    } else {
                        // For basic question-types this will always return elements if it returns anything.
                        panic!("This does not happen.");
                    }
                }
            }
        }
        if total_removed > 0 {
            if execute {
                let _ = parser.save_to_file(file_name.clone());
                println!(" Executed changes, removed {} bytes.", total_removed);
            } else {
                println!(" Did not execute changes, use the '--execute' argument to remove {} bytes.", total_removed);
            }
        }
    }
}
```
You can test running that with:
```
cargo run some_question.xml
```
Note! That this is not a safe implementation. It may miss some references if the file names are suitably complicated and the original toolset uses different url-encoding logic.