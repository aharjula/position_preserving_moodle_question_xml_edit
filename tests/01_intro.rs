use position_preserving_moodle_question_xml_edit::{QParser, Question, ContentType, Change};

/// A question.xml file can contain many questions of different types
/// the first step for acting on them is to identify the ones that are of 
/// suitable types
#[test]
fn count_questions_in_content() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">...</question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	// Parsers need to always be mutable, due to the way they track changes between searches.
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let questions: Vec<Question> = parser.find_questions();

	assert_eq!(questions.len(), 2);
	assert_eq!(questions[0].qtype, "some");
	assert_eq!(questions[0].index, 0);
	assert_eq!(questions[0].whole_element.content, "<question type=\"some\">...</question>".to_string());
	assert_eq!(questions[1].qtype, "stack");
	assert_eq!(questions[1].index, 1);
	assert_eq!(questions[1].whole_element.content, "<question type=\"stack\">...</question>".to_string());
}

/// Once one has identified a question of the type one can query for 
/// elements inside it and gain ContentRefs to parts of them.
#[test]
fn extract_named_element_from_question() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">
  	<name>Test question</name>
  </question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let index: usize = parser.find_questions().into_iter()
		.filter(|q| q.qtype == "some".to_string())
		.map(|q| q.index).next()
		.expect("The above data has atleast one such question.");

	// You might be searching for many different tags at the same time.
	let elements: Vec<ContentType> = parser.get_elements(index, vec!["name".to_string()]);

	assert_eq!(index, 0);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, _whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			// The contents & attibutes list should contain AttributeValue and ElementContent items.
			// Now just the latter as no attributes are in play.
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				assert_eq!(content_ref.content, "Test question".to_string());
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}


/// Once you have a ContentRef you can replace that bit of content.
#[test]
fn update_named_element_from_question() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">
  	<name>Test question</name>
  </question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let target_data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
<!-- question: 1  -->
  <question type=\"some\">
  	<name>Lorem ipsum question</name>
  </question>
<!-- question: 2  -->
  <question type=\"stack\">...</question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	let index: usize = parser.find_questions().into_iter()
		.filter(|q| q.qtype == "some".to_string())
		.map(|q| q.index).next()
		.expect("The above data has atleast one such question.");

	// You might be searching for many different tags at the same time.
	let elements: Vec<ContentType> = parser.get_elements(index, vec!["name".to_string()]);

	assert_eq!(index, 0);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, _whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"name".to_string());
			assert_eq!(contents_and_attributes.len(), 1);
			// The contents & attibutes list should contain AttributeValue and ElementContent items.
			// Now just the latter as no attributes are in play.
			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[0] {
				// content_ref here is a ContentRef which contains necessary 
				// information to map to the original document as well as 
				// the text content of a particular item.
				assert_eq!(content_ref.content, "Test question".to_string());

				// Now generate a change to that elements content.
				let change: Change = Change {
					position: content_ref.clone(),
					new_content: "Lorem ipsum question".to_string()
				};

				// That change will be registered to the parser,
				// many others may be also added as long as they do not 
				// overlap in the original documents indices.
				parser.register_change(change);

				// Every time the parser does a search it first executes 
				// those changes. But we can also ask it to do that now,
				parser.execute_changes();

				assert_eq!(parser.get_current_content(), target_data);
			} else {
				panic!("Wrong type found!");
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

/// One can also target attributes. And change their values,
/// remember to entity encode values though as we allow writing 
/// anything.
#[test]
fn change_attribute_value_wrong_way() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<foo bar='&amp;abc'>test</foo>
  </question>
</quiz>
".to_string();
	let target_data_a = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<foo bar='&'>TEST</foo>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	// You might be searching for many different tags at the same time.
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["foo".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, _whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"foo".to_string());
			assert_eq!(contents_and_attributes.len(), 2);
			// The contents & attibutes list should contain AttributeValue and ElementContent items.
			// Now we have exactyle one attribute and one value.
			if let ContentType::AttributeValue(name, content_ref) = &contents_and_attributes[0] {
				// content_ref here is a ContentRef which contains necessary 
				// information to map to the original document as well as 
				// the text content of a particular item.
				assert_eq!(name, &"bar".to_string());
				// Note that the entity has not been decoded.
				assert_eq!(content_ref.content, "&amp;abc".to_string());
				assert_eq!(content_ref.basic_entity_decode(), "&abc".to_string());

				// Now generate a change to that attributes content.
				// NOTE that we do not entity escape even though we did decode.
				// There is a function for escaping if need be.
				let change: Change = Change {
					position: content_ref.clone(),
					new_content: "&".to_string()
				};

				// That change will be registered to the parser,
				// many others may be also added as long as they do not 
				// overlap in the original documents indices.
				parser.register_change(change);

				// Every time the parser does a search it first executes 
				// those changes. But we can also ask it to do that now,
			} else {
				panic!("Wrong type found!");
			}

			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[1] {
				// content_ref here is a ContentRef which contains necessary 
				// information to map to the original document as well as 
				// the text content of a particular item.
				assert_eq!(content_ref.content, "test".to_string());

				// Now generate a change to that elements content.
				let change: Change = Change {
					position: content_ref.clone(),
					new_content: "TEST".to_string()
				};

				// That change will be registered to the parser,
				// many others may be also added as long as they do not 
				// overlap in the original documents indices.
				parser.register_change(change);

				// Every time the parser does a search it first executes 
				// those changes. But we can also ask it to do that now,
			} else {
				panic!("Wrong type found!");
			}

			parser.execute_changes();

			assert_eq!(parser.get_current_content(), target_data_a);
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}

/// When you want to entity encode attribute values there is 
/// a function for that.
#[test]
fn change_attribute_value_correct_way() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<foo bar='&amp;abc'>test</foo>
  </question>
</quiz>
".to_string();
	let target_data_a = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<foo bar='&amp;&apos;'>TEST&</foo>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	// You might be searching for many different tags at the same time.
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["foo".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::Element(tag_name, _whole_element_ref, contents_and_attributes) => {
			assert_eq!(tag_name, &"foo".to_string());
			assert_eq!(contents_and_attributes.len(), 2);
			// The contents & attibutes list should contain AttributeValue and ElementContent items.
			// Now we have exactyle one attribute and one value.
			if let ContentType::AttributeValue(name, content_ref) = &contents_and_attributes[0] {
				// content_ref here is a ContentRef which contains necessary 
				// information to map to the original document as well as 
				// the text content of a particular item.
				assert_eq!(name, &"bar".to_string());
				// Note that the entity has not been decoded.
				assert_eq!(content_ref.content, "&amp;abc".to_string());
				assert_eq!(content_ref.basic_entity_decode(), "&abc".to_string());

				// Now generate a change to that attributes content.
				// This is the shorthand for generating a properly entity 
				// encoded version.
				let change: Change = Change::attribute_escaped_version(
					content_ref.clone(),
					"&'".to_string()
				);

				// That change will be registered to the parser,
				// many others may be also added as long as they do not 
				// overlap in the original documents indices.
				parser.register_change(change);

				// Every time the parser does a search it first executes 
				// those changes. But we can also ask it to do that now,
			} else {
				panic!("Wrong type found!");
			}

			if let ContentType::ElementContent(content_ref) = &contents_and_attributes[1] {
				// content_ref here is a ContentRef which contains necessary 
				// information to map to the original document as well as 
				// the text content of a particular item.
				assert_eq!(content_ref.content, "test".to_string());

				// Now generate a change to that elements content.
				// This is the "constructor" for raw raplacement change, it does
				// not entity escape anything, nor does it do CDATA processing.
				let change: Change = Change::new(
					content_ref.clone(),
					"TEST&".to_string()
				);

				// That change will be registered to the parser,
				// many others may be also added as long as they do not 
				// overlap in the original documents indices.
				parser.register_change(change);

				// Every time the parser does a search it first executes 
				// those changes. But we can also ask it to do that now,
			} else {
				panic!("Wrong type found!");
			}

			parser.execute_changes();

			assert_eq!(parser.get_current_content(), target_data_a);
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}


/// There are some multi-element constructs that are identified and
/// access provided as a more complex object.
/// For example any element with a format-attribute and a text element
/// as a child is considered a MoodleTextElement and some access methods
/// are made clearer.
#[test]
fn moodle_formated_text_elements() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<prtcorrect format=\"html\">
      <text><![CDATA[<span style=\"font-size: 1.5em; color:green;\"><i class=\"fa fa-check\"></i></span> Correct answer, well done.]]></text>
    </prtcorrect>
  </question>
</quiz>
".to_string();
	let target_data_a = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<prtcorrect format=\"other\">
      <text><![CDATA[[[commonstring key='correct_answer_well_done'/]]]]></text>
    </prtcorrect>
  </question>
</quiz>
".to_string();
	let mut parser = QParser::from_string(data).expect("Valid input should not fail");
	// This query won't return a ContentType::Element, it returns
	// a ContentType::MoodleTextElement which gives access to multiple
	// targets, the format in the root element and the text element-content
	// inside as well as any file-elements containing attachement files.
	let elements: Vec<ContentType> = parser.get_elements(0, vec!["prtcorrect".to_string()]);
	assert_eq!(elements.len(), 1);

	match &elements[0] {
		ContentType::MoodleTextElement(tag_name, format_attribute, text_and_file_elements) => {
			assert_eq!(tag_name, &"prtcorrect".to_string());
			assert_eq!(format_attribute.content, "html".to_string());
			assert_eq!(text_and_file_elements.len(), 1);

			let change1 = Change::attribute_escaped_version(format_attribute.clone(), "other".to_string());
			parser.register_change(change1);

			// This list contains both ElementContent and Element objects.
			// The first will always be the ElementContent of the text-element.
			match &text_and_file_elements[0] {
				ContentType::ElementContent(content_ref) => {
					// The contents are CDATA-wrapped in this case.
					assert_eq!(content_ref.content, "<![CDATA[<span style=\"font-size: 1.5em; color:green;\"><i class=\"fa fa-check\"></i></span> Correct answer, well done.]]>".to_string());
					// If you would rather work with them unwrapped you can ask fro unwrapped version.
					assert_eq!(content_ref.unwrap_cdata(), "<span style=\"font-size: 1.5em; color:green;\"><i class=\"fa fa-check\"></i></span> Correct answer, well done.".to_string());

					// You should always CDATA wrap these, although that wrapping only happens if need be.
					let change2 = Change::cdata_wrapped_version(content_ref.clone(), "[[commonstring key='correct_answer_well_done'/]]".to_string());
					parser.register_change(change2);

					parser.execute_changes();

					assert_eq!(parser.get_current_content(), target_data_a);
				},
				_ => {
					panic!("Wrong type found!");
				}
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}


/// When one has an access to an element one can modify the whole element.
/// One problematic case is dealing with empty elements, as our parser
/// does not provide references to their non existent content one cannot
/// modify that content. In such a case one can modify the element itself.
#[test]
fn whole_element_modification() {
	let data = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<trouble/>
  </question>
</quiz>
".to_string();
	let target_data_a = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<notrouble></notrouble>
  </question>
</quiz>
".to_string();
	let target_data_b = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<quiz>
  <question type=\"some\">
  	<notrouble>something</notrouble>
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

			// You can simply change the whole tag.
			// Note however, that if you do this for an element that
			// has attributes you may not change those attributes
			// through references to them during the same execution.
			parser.register_change(Change::new(whole_element_ref.clone(), "<notrouble></notrouble>".to_string()));

			parser.execute_changes();
			assert_eq!(parser.get_current_content(), target_data_a);

			// After that we find the changed element and it has a reference to its 0-length content.
			let elements2: Vec<ContentType> = parser.get_elements(0, vec!["notrouble".to_string()]);
			assert_eq!(elements2.len(), 1);
			match &elements2[0] {
				ContentType::Element(tag_name2, whole_element_ref2, contents_and_attributes2) => {
					assert_eq!(tag_name2, &"notrouble".to_string());
					assert_eq!(contents_and_attributes2.len(), 1);
					assert_eq!(whole_element_ref2.content, "<notrouble></notrouble>");

					if let ContentType::ElementContent(content_ref) = &contents_and_attributes2[0] {
						assert_eq!(content_ref.content, "");

						parser.register_change(Change::new(content_ref.clone(), "something".to_string()));
						parser.execute_changes();
						assert_eq!(parser.get_current_content(), target_data_b);
					} else {
						panic!("Wrong type found!");	
					}
				},
				_ => {
					panic!("Wrong type found!");
				}
			}
		},
		_ => {
			panic!("Wrong type found!");
		}
	}
}
