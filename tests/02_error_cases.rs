use position_preserving_moodle_question_xml_edit::*;
/// Some actions are mistakes and the toolset tries to detect them.
/// But we don't typically deal with them gracefully.

#[test]
#[should_panic(expected = "Only 2 questions, but was trying to get index 2.")]
fn referencing_a_nonexistent_question() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">...</question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let questions: Vec<Question> = parser.find_questions();

	assert_eq!(questions.len(), 2);

	// Now try to get something from question at index 2.
	parser.get_elements(2, vec!["something".to_string()]);
}

#[test]
#[should_panic(expected = "Overlap of uncommitted changes, cannot continue.")]
fn overlapping_updates_1_same_target() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<name>Test question</name>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["name".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, _whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				assert_eq!(content_ref.content, "Test question".to_string());

				// The first change is fine to register.
				let change = Change::new(content_ref.clone(), "Foo".to_string());
				parser.register_change(change);

				// But changing the same bit again should not be allowed.
				let change2 = Change::new(content_ref.clone(), "Bar".to_string());
				parser.register_change(change2);
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

#[test]
#[should_panic(expected = "Overlap of uncommitted changes, cannot continue.")]
fn overlapping_updates_2_child_and_parent() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<name>Test question</name>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["name".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				assert_eq!(content_ref.content, "Test question".to_string());

				// The first change is fine to register.
				let change = Change::new(content_ref.clone(), "Foo".to_string());
				parser.register_change(change);

				// Now if we try to modify the whole element 
				// as well as its contents. Things should explode.
				let change2 = Change::new(whole_element_ref.clone(), "<nimi>Bar</nimi>".to_string());
				parser.register_change(change2);
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

#[test]
#[should_panic(expected = "Overlap of uncommitted changes, cannot continue.")]
fn overlapping_updates_3_parent_and_child() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<name>Test question</name>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["name".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				assert_eq!(content_ref.content, "Test question".to_string());

				// The first change is fine to register.
				let change = Change::new(whole_element_ref.clone(), "<nimi>Bar</nimi>".to_string());
				parser.register_change(change);

				// The second would change something already marked as changing.
				let change2 = Change::new(content_ref.clone(), "Foo".to_string());
				parser.register_change(change2);
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

#[test]
#[should_panic(expected = "Use of a content-reference to a stale search result detected.")]
fn using_old_references() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<name>Test question</name>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["name".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				assert_eq!(content_ref.content, "Test question".to_string());

				// Lets change something.
				let change = Change::new(whole_element_ref.clone(), "<nimi>Bar</nimi>".to_string());
				parser.register_change(change);

				// Execute those changes.
				parser.execute_changes();

				// Then try to use those same serch results to change things again.
				let change2 = Change::new(content_ref.clone(), "Test".to_string());
				// This should fail as the references were from a search targeting a previous version of he document.
				parser.register_change(change2);
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

#[test]
fn trying_to_parse_a_broken_document() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<pair></missing>
  </question>
</quiz>
".to_string();
	let parser: Result<QParser, String> = QParser::from_string(data);

	match parser {
		Ok(_) => {
			panic!("It was supposed to fail.");
		},
		Err(s) => {
			assert_eq!(s, "Errors parsing the original document.".to_string());
		}
	}
}

/// As we do not check the changes or if they lead to a broken
/// document it is possible to not be able to search after one
/// has broken a document.
/// Likewise, this tool won't allow one to save the document into
/// a file if one breaks it. Although if you intentionally break
/// things you can always ask for the contents of the whole document
/// and save it anywhere you wish.
/// You just cannot continue using this toolset after the document 
/// has been broken.
#[test]
#[should_panic]
fn continuing_after_breaking_a_document() {
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

			// But what happens when we try to search again?
			parser.get_elements(0, vec!["anything".to_string()]);
		},
		_ => {
			// Not panicing here, as we will leave the actual panic message undefined.
		}
	}
}
