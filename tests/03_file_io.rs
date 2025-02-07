use position_preserving_moodle_question_xml_edit::*;
use assert_fs::fixture::NamedTempFile;


#[test]
fn load_from_file() {
	// We will first write this to a file.
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">...</question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let test_file = NamedTempFile::new("test.xml").unwrap();
	let file_name: String = test_file.to_str().expect("Some sort of name").to_string();
	let _ = std::fs::write(file_name.clone(), data.clone());
	

	// Then use that files filepath to open it through the parser.
	let parser = QParser::load_xml_file(file_name).expect("Valid input should not fail");

	assert_eq!(parser.get_current_content(), data);
}

#[test]
fn save_to_file() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">...</question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let target_data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">...</question>
<!-- question: 2  -->
  
</quiz>
".to_string();
	let test_file = NamedTempFile::new("test.xml").unwrap();
	let file_name: String = test_file.to_str().expect("Some sort of name").to_string();

	// To have something to check we drop that second question from the file.
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let questions: Vec<Question> = parser.find_questions();
	let change = Change::new(questions[1].whole_element.clone(), "".to_string());
	parser.register_change(change);
	// Note that we do not need to execute changes manually,
	// they will be executed during every search and save action.
	//parser.execute_changes();

	// Write to file.
	let _ = parser.save_to_file(file_name.clone());

	// Read it back.
	let file_content: String = std::fs::read_to_string(file_name).expect("Should have created a file with content.");
	assert_eq!(file_content, target_data);
}



#[test]
fn trying_to_save_a_broken_document() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<trouble/>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	// This query will return a ContentType::Element but it is special
	// in the sense that no ContentType::ElementContent object
	// will be present in its listing.
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["trouble".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"trouble".to_string());
			assert_eq!(contents_and_attributes.len(), 0);
			assert_eq!(whole_element_ref.content, "<trouble/>");

			// For testing, lets replace that whoel element with something broken.
			let change = Change::new(whole_element_ref.clone(), "<missing></part>".to_string());
			// The system will happily execute that change.
			parser.register_change(change);
			parser.execute_changes();

			// The document is now broken, it should not be possible to save it to a file.
			let test_file = NamedTempFile::new("test.xml").unwrap();
			let file_name: String = test_file.to_str().expect("Some sort of name").to_string();
			let _ = std::fs::write(file_name.clone(), "test".to_string());

			// Will it error like it should?
			match parser.save_to_file(file_name.clone()) {
				Ok(_) => {
					panic!("This should not happen!");
				} Err(e) => {
					assert_eq!(e, "Will not write to a file due to content being broken.".to_string());
				}
			}

			// Check that the file is still as it was.
			let test = std::fs::read_to_string(file_name).expect("Should be a readable file.");
			assert_eq!(test, "test".to_string());
		},
		_ => {
			// Not panicing here, as we will leave the actual panic message undefined.
		}
	}
}
