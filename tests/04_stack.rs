use position_preserving_moodle_question_xml_edit::*;

#[test]
fn read_as_stack_question() {
	let mut parser = QParser::load_xml_file("tests/tests/minimal-stack.xml".to_string()).expect("Valid input should not fail");

	// Knowing that there is a STACK question there we can simply ask for it.
	// Otherwise first use `parser.find_questions()` to find out which questions 
	// are STACK questions, this won't work for any others.
	let question = parser.get_as_stack_question(0);

	assert_eq!(question.name.content, "minimal".to_string());
	// Note the line changes when producing replacement content one might want to match those.
	assert_eq!(question.questionvariables.content, "a: 1+rand(5);\r\nb: 2+rand(5);\r\nta: a+b;".to_string());

	// Remember to deal with CDATA in those text-segments.
	assert_eq!(question.questiontext.get_content().unwrap().unwrap_cdata(), "<p>\\({@a@}+{@b@}=\\) [[input:ans1]] </p>\r\n<p>[[validation:ans1]]</p>".to_string());

	// Note that for input types the identifier needs to be escaped.
	assert_eq!(question.inputs.get(&"ans1".to_string()).unwrap().r#type.content, "algebraic".to_string());
	assert_eq!(question.inputs.get(&"ans1".to_string()).unwrap().tans.content, "ta".to_string());

	// PRT nodes are a bit far down the chain.
	assert_eq!(question.prts.get(&"prt1".to_string()).unwrap().nodes[0].answertest.content, "AlgEquiv".to_string());

	// For question tests there are maps to help getting specific PRT-expectations and input values.
	// The tests are nto behind mappings as naming has not been used that much.
	assert_eq!(question.tests[0].description.content, "Test case assuming the teacher's input gets full marks.".to_string());
	assert_eq!(question.tests[0].inputs.get(&"ans1".to_string()).unwrap().value.content, "ta".to_string());
	assert_eq!(question.tests[0].expected.get(&"prt1".to_string()).unwrap().expectedscore.content, "1.0000000".to_string());
}

#[test]
fn access_by_type() {
	let mut parser = QParser::load_xml_file("tests/tests/minimal-stack.xml".to_string()).expect("Valid input should not fail");

	// Knowing that there is a STACK question there we can simply ask for it.
	// Otherwise first use `parser.find_questions()` to find out which questions 
	// are STACK questions, this won't work for any others.
	let question = parser.get_as_stack_question(0);

	// Sometimes you only wish to look at all the parts of a given type.
	// For those times we have specific access functions.
	// Obviously you are not concatenating these in normal use, but for 
	// testing this works fine.
	let mut all_castext_catenated: String = String::new();
	for (_, c) in question.get_castext_fields() {
		// These are MoodleText objects so the content is deeper.
		all_castext_catenated.push_str(&c.get_content().expect("These will currently always have content.").unwrap_cdata());
		all_castext_catenated.push('|');
	}
	assert_eq!(all_castext_catenated, "<p>\\({@a@}+{@b@}=\\) [[input:ans1]] </p>\r\n<p>[[validation:ans1]]</p>||[[feedback:prt1]]|\\({@a@}+{@b@}={@ta@}\\)||Correct answer, well done.|Your answer is partially correct.|Incorrect answer.|||");

	let mut all_casstrings_catenated: String = String::new();
	for (_, c) in question.get_castring_fields() {
		// Here the content is directly available.
		all_casstrings_catenated.push_str(&c.unwrap_cdata());
		all_casstrings_catenated.push('|');
	}
	// Here the values are, the tans of the input, sans and tans of the PRT node, 
	// testoption and penalties are blank but the scores are here, finally there 
	// is the input for the question test.
	assert_eq!(all_casstrings_catenated, "ta|ans1|ta||1||0||ta|");

	let mut all_keyval_catenated: String = String::new();
	for (_, c) in question.get_keyval_fields() {
		// Here the content is directly available.
		all_keyval_catenated.push_str(&c.unwrap_cdata());
		all_keyval_catenated.push('|');
	}
	assert_eq!(all_keyval_catenated, "a: 1+rand(5);\r\nb: 2+rand(5);\r\nta: a+b;||");
}

