//! STACK specific extensions for the library. 
//! These structs allow dealing with related fields in a saner way.
//! Note that this tooling is for the question-xml format of a relatively 
//! recent STACK in this case ~4.8.0

use crate::{ContentRef, ContentType, QParser};
use std::collections::HashMap;

/// STACK specific struct for working with inputs.
#[derive(Debug, Clone)]
pub struct STACKInput {
    pub name: ContentRef,
    pub r#type: ContentRef,
    pub tans: ContentRef,
    pub boxsize: ContentRef,
    pub strictsyntax: ContentRef,
    pub insertstars: ContentRef,
    pub syntaxhint: ContentRef,
    pub syntaxattribute: ContentRef,
    pub forbidwords: ContentRef,
    pub allowwords: ContentRef,
    pub forbidfloat: ContentRef,
    pub requirelowestterms: ContentRef,
    pub checkanswertype: ContentRef,
    pub mustverify: ContentRef,
    pub showvalidation: ContentRef,
    pub options: ContentRef
}

/// STACK specific struct for working with PRT-nodes.
/// Note that any CASText items are given as ContentType::MoodleTextElements thus
/// alowing access to attachement files and the format.
#[derive(Debug, Clone)]
pub struct STACKPrtNode {
    pub name: ContentRef,
    pub answertest: ContentRef,
    pub sans: ContentRef,
    pub tans: ContentRef,
    pub testoptions: ContentRef,
    pub quiet: ContentRef,
    pub truescoremode: ContentRef,
    pub truescore: ContentRef,
    pub truepenalty: ContentRef,
    pub truenextnode: ContentRef,
    pub trueanswernote: ContentRef,
    pub truefeedback: ContentType,
    pub falsescoremode: ContentRef,
    pub falsescore: ContentRef,
    pub falsepenalty: ContentRef,
    pub falsenextnode: ContentRef,
    pub falseanswernote: ContentRef,
    pub falsefeedback: ContentType
}

/// STACK specific struct for working with PRTs.
#[derive(Debug, Clone)]
pub struct STACKPrt {
    pub name: ContentRef,
    pub value: ContentRef,
    pub autosimplify: ContentRef,
    pub feedbackstyle: ContentRef,
    pub feedbackvariables: ContentRef,
    pub nodes: Vec<STACKPrtNode>
}

/// STACK specific struct for working with question test inputs.
#[derive(Debug, Clone)]
pub struct STACKQtestInput {
	pub name: ContentRef,
    pub value: ContentRef,
}

/// STACK specific struct for working with question test expected results.
#[derive(Debug, Clone)]
pub struct STACKQtestExpected {
	pub name: ContentRef,
    pub expectedscore: ContentRef,
    pub expectedpenalty: ContentRef,
    pub expectedanswernote: ContentRef,
}

/// STACK specific struct for working with question tests.
#[derive(Debug, Clone)]
pub struct STACKQtest {
	pub testcase: ContentRef,
    pub description: ContentRef,
	pub inputs: HashMap<String, STACKQtestInput>,
	pub expected: HashMap<String, STACKQtestExpected>
}

/// STACK specific struct presenting a whole STACK question.
/// Note that any CASText items are given as ContentType::MoodleTextElements thus
/// alowing access to attachement files and the format.
///
/// NOTE That this version requires STACK to be fresh enough to use formatted 
/// question notes.
#[derive(Debug, Clone)]
pub struct STACKQuestion {
	pub name: ContentRef,
	pub questiontext: ContentType,
	pub generalfeedback: ContentType,
	pub defaultgrade: ContentRef,
	pub penalty: ContentRef,
	pub hidden: ContentRef,
	pub idnumber: ContentRef,
	pub stackversion: ContentRef,
	pub questionvariables: ContentRef,
	pub specificfeedback: ContentType,
	pub questionnote: ContentType,
	pub questiondescription: ContentType,
	pub questionsimplify: ContentRef,
	pub assumepositive: ContentRef,
	pub assumereal: ContentRef,
	pub prtcorrect: ContentType,
	pub prtpartiallycorrect: ContentType,
	pub prtincorrect: ContentType,
	pub decimals: ContentRef,
	pub scientificnotation: ContentRef,
	pub multiplicationsign: ContentRef,
	pub sqrtsign: ContentRef,
	pub complexno: ContentRef,
	pub inversetrig: ContentRef,
	pub logicsymbol: ContentRef,
	pub matrixparens: ContentRef,
	pub variantsselectionseed: ContentRef,
	pub inputs: HashMap<String, STACKInput>,
	pub prts: HashMap<String, STACKPrt>,
	pub tests: Vec<STACKQtest>
}

impl QParser {
	/// Type specific extraction of questions
	pub fn get_as_stack_question(&mut self, qnum: usize) -> STACKQuestion {
		self.execute_changes();

        let opt = roxmltree::ParsingOptions {
            allow_dtd: true,
            ..roxmltree::ParsingOptions::default()
        };
        let binding = self.content.clone();
        let doc = match roxmltree::Document::parse_with_options(&binding, opt) {
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
        					if qtype != "stack" {
        						panic!("Was expecting a 'stack' question, found '{}' instead.", qtype);
        					} 				
        					return self.into_stack_question(node);
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

	/// Internal logic for the top level of a STACK question.
	fn into_stack_question(&mut self, node: roxmltree::Node) -> STACKQuestion {
		let mut name: Option<ContentRef> = None;
		let mut questiontext: Option<ContentType> = None;
		let mut generalfeedback: Option<ContentType> = None;
		let mut defaultgrade: Option<ContentRef> = None;
		let mut penalty: Option<ContentRef> = None;
		let mut hidden: Option<ContentRef> = None;
		let mut idnumber: Option<ContentRef> = None;
		let mut stackversion: Option<ContentRef> = None;
		let mut questionvariables: Option<ContentRef> = None;
		let mut specificfeedback: Option<ContentType> = None;
		let mut questionnote: Option<ContentType> = None;
		let mut questiondescription: Option<ContentType> = None;
		let mut questionsimplify: Option<ContentRef> = None;
		let mut assumepositive: Option<ContentRef> = None;
		let mut assumereal: Option<ContentRef> = None;
		let mut prtcorrect: Option<ContentType> = None;
		let mut prtpartiallycorrect: Option<ContentType> = None;
		let mut prtincorrect: Option<ContentType> = None;
		let mut decimals: Option<ContentRef> = None;
		let mut scientificnotation: Option<ContentRef> = None;
		let mut multiplicationsign: Option<ContentRef> = None;
		let mut sqrtsign: Option<ContentRef> = None;
		let mut complexno: Option<ContentRef> = None;
		let mut inversetrig: Option<ContentRef> = None;
		let mut logicsymbol: Option<ContentRef> = None;
		let mut matrixparens: Option<ContentRef> = None;
		let mut variantsselectionseed: Option<ContentRef> = None;
		let mut inputs: HashMap<String, STACKInput> = HashMap::new();
		let mut prts: HashMap<String, STACKPrt> = HashMap::new();
		let mut tests: Vec<STACKQtest> = Vec::new();

		// Collect the simpler elements.
		let elems = self._get_elements(node, vec![
			"questiontext".to_string(),
			"generalfeedback".to_string(),
			"defaultgrade".to_string(),
			"penalty".to_string(),
			"hidden".to_string(),
			"idnumber".to_string(),
			"specificfeedback".to_string(),
			"questionnote".to_string(),
			"questiondescription".to_string(),
			"questionsimplify".to_string(),
			"assumepositive".to_string(),
			"assumereal".to_string(),
			"prtcorrect".to_string(),
			"prtpartiallycorrect".to_string(),
			"prtincorrect".to_string(),
			"decimals".to_string(),
			"scientificnotation".to_string(),
			"multiplicationsign".to_string(),
			"sqrtsign".to_string(),
			"complexno".to_string(),
			"inversetrig".to_string(),
			"logicsymbol".to_string(),
			"matrixparens".to_string(),
			"variantsselectionseed".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"defaultgrade" => { defaultgrade = cref; }
							"penalty" => { penalty = cref; }
							"hidden" => { hidden = cref; }
							"idnumber" => { idnumber = cref; }
							"questionsimplify" => { questionsimplify = cref; }
							"assumepositive" => { assumepositive = cref; }
							"assumereal" => { assumereal = cref; }
							"decimals" => { decimals = cref; }
							"scientificnotation" => { scientificnotation = cref; }
							"multiplicationsign" => { multiplicationsign = cref; }
							"sqrtsign" => { sqrtsign = cref; }
							"complexno" => { complexno = cref; }
							"inversetrig" => { inversetrig = cref; }
							"logicsymbol" => { logicsymbol = cref; }
							"matrixparens" => { matrixparens = cref; }
							"variantsselectionseed" => { variantsselectionseed = cref; }
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				ContentType::MoodleTextElement(ref nam, _, _) => {
					match nam.as_str() {
						"questiontext" => { questiontext = Some(el); }
						"generalfeedback" => { generalfeedback = Some(el); }
						"specificfeedback" => { specificfeedback = Some(el); }
						"questionnote" => { questionnote = Some(el); }
						"questiondescription" => { questiondescription = Some(el); }
						"prtcorrect" => { prtcorrect = Some(el); }
						"prtpartiallycorrect" => { prtpartiallycorrect = Some(el); }
						"prtincorrect" => { prtincorrect = Some(el); }
						_ => { panic!("Unexpected MoodleTextElement."); }
					}
				},
				_ => {
					panic!("Unexpected type.");
				}
			}
		}

		// Then the few separate ones, and those other structures.
		for n in node.children() {
			if n.is_element() && (
				n.tag_name().name() == "name" || 
				n.tag_name().name() == "stackversion" ||
				n.tag_name().name() == "questionvariables") {
				// Pick the inner text of these.
				let text = &self._get_elements(n, vec!["text".to_string()])[0];
                if let ContentType::Element(_,_,items) = text {
                    let econtent = &items[0];
                    if let ContentType::ElementContent(c) = econtent {
                        let cref = Some(c.clone());
                        match n.tag_name().name() {
                        	"name" => { name = cref; },
                        	"stackversion" => { stackversion = cref; },
                        	"questionvariables" => { questionvariables = cref; },
                			_ => { panic!("This cannot happen."); }
                        }
                    }
                }
			} else if n.is_element() && n.tag_name().name() == "input" {
				let input = self.into_stack_input(n);
				inputs.insert(input.name.content.clone(), input);
			} else if n.is_element() && n.tag_name().name() == "prt" {
				let prt = self.into_stack_prt(n);
				prts.insert(prt.name.content.clone(), prt);
			} else if n.is_element() && n.tag_name().name() == "qtest" {
				let test = self.into_stack_qtest(n);
				tests.push(test);
			}
		}

		STACKQuestion {
			name: name.expect("Missing 'name' element."),
			questiontext: questiontext.expect("Missing 'questiontext' element."),
			generalfeedback: generalfeedback.expect("Missing 'generalfeedback' element."),
			defaultgrade: defaultgrade.expect("Missing 'defaultgrade' element."),
			penalty: penalty.expect("Missing 'penalty' element."),
			hidden: hidden.expect("Missing 'hidden' element."),
			idnumber: idnumber.expect("Missing 'idnumber' element."),
			stackversion: stackversion.expect("Missing 'stackversion' element."),
			questionvariables: questionvariables.expect("Missing 'questionvariables' element."),
			specificfeedback: specificfeedback.expect("Missing 'specificfeedback' element."),
			questionnote: questionnote.expect("Missing 'questionnote' element."),
			questiondescription: questiondescription.expect("Missing 'questiondescription' element."),
			questionsimplify: questionsimplify.expect("Missing 'questionsimplify' element."),
			assumepositive: assumepositive.expect("Missing 'assumepositive' element."),
			assumereal: assumereal.expect("Missing 'assumereal' element."),
			prtcorrect: prtcorrect.expect("Missing 'prtcorrect' element."),
			prtpartiallycorrect: prtpartiallycorrect.expect("Missing 'prtpartiallycorrect' element."),
			prtincorrect: prtincorrect.expect("Missing 'prtincorrect' element."),
			decimals: decimals.expect("Missing 'decimals' element."),
			scientificnotation: scientificnotation.expect("Missing 'scientificnotation' element."),
			multiplicationsign: multiplicationsign.expect("Missing 'multiplicationsign' element."),
			sqrtsign: sqrtsign.expect("Missing 'sqrtsign' element."),
			complexno: complexno.expect("Missing 'complexno' element."),
			inversetrig: inversetrig.expect("Missing 'inversetrig' element."),
			logicsymbol: logicsymbol.expect("Missing 'logicsymbol' element."),
			matrixparens: matrixparens.expect("Missing 'matrixparens' element."),
			variantsselectionseed: variantsselectionseed.expect("Missing 'variantsselectionseed' element."),
			inputs: inputs,
			prts: prts,
			tests: tests
		}
	}

	fn into_stack_input(&mut self, node: roxmltree::Node) -> STACKInput {
		let mut name: Option<ContentRef> = None;
		let mut r#type: Option<ContentRef> = None;
		let mut tans: Option<ContentRef> = None;
		let mut boxsize: Option<ContentRef> = None;
		let mut strictsyntax: Option<ContentRef> = None;
		let mut insertstars: Option<ContentRef> = None;
		let mut syntaxhint: Option<ContentRef> = None;
		let mut syntaxattribute: Option<ContentRef> = None;
		let mut forbidwords: Option<ContentRef> = None;
		let mut allowwords: Option<ContentRef> = None;
		let mut forbidfloat: Option<ContentRef> = None;
		let mut requirelowestterms: Option<ContentRef> = None;
		let mut checkanswertype: Option<ContentRef> = None;
		let mut mustverify: Option<ContentRef> = None;
		let mut showvalidation: Option<ContentRef> = None;
		let mut options: Option<ContentRef> = None;

		let elems = self._get_elements(node, vec![
			"name".to_string(),
			"type".to_string(),
			"tans".to_string(),
			"boxsize".to_string(),
			"strictsyntax".to_string(),
			"insertstars".to_string(),
			"syntaxhint".to_string(),
			"syntaxattribute".to_string(),
			"forbidwords".to_string(),
			"allowwords".to_string(),
			"forbidfloat".to_string(),
			"requirelowestterms".to_string(),
			"checkanswertype".to_string(),
			"mustverify".to_string(),
			"showvalidation".to_string(),
			"options".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"name" => { name = cref; },
							"type" => { r#type = cref; },
							"tans" => { tans = cref; },
							"boxsize" => { boxsize = cref; },
							"strictsyntax" => { strictsyntax = cref; },
							"insertstars" => { insertstars = cref; },
							"syntaxhint" => { syntaxhint = cref; },
							"syntaxattribute" => { syntaxattribute = cref; },
							"forbidwords" => { forbidwords = cref; },
							"allowwords" => { allowwords = cref; },
							"forbidfloat" => { forbidfloat = cref; },
							"requirelowestterms" => { requirelowestterms = cref; },
							"checkanswertype" => { checkanswertype = cref; },
							"mustverify" => { mustverify = cref; },
							"showvalidation" => { showvalidation = cref; },
							"options" => { options = cref; },
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				_ => { panic!("Unexpected type.");}
			}
		}

		STACKInput {
		    name: name.expect("Missing name element."),
		    r#type: r#type.expect("Missing type element."),
		    tans: tans.expect("Missing tans element."),
		    boxsize: boxsize.expect("Missing boxsize element."),
		    strictsyntax: strictsyntax.expect("Missing strictsyntax element."),
		    insertstars: insertstars.expect("Missing insertstars element."),
		    syntaxhint: syntaxhint.expect("Missing syntaxhint element."),
		    syntaxattribute: syntaxattribute.expect("Missing syntaxattribute element."),
		    forbidwords: forbidwords.expect("Missing forbidwords element."),
		    allowwords: allowwords.expect("Missing allowwords element."),
		    forbidfloat: forbidfloat.expect("Missing forbidfloat element."),
		    requirelowestterms: requirelowestterms.expect("Missing requirelowestterms element."),
		    checkanswertype: checkanswertype.expect("Missing checkanswertype element."),
		    mustverify: mustverify.expect("Missing mustverify element."),
		    showvalidation: showvalidation.expect("Missing showvalidation element."),
		    options: options.expect("Missing options element.")
		}
	}

	fn into_stack_prt(&mut self, node: roxmltree::Node) -> STACKPrt {
	    let mut name: Option<ContentRef> = None;
	    let mut value: Option<ContentRef> = None;
	    let mut autosimplify: Option<ContentRef> = None;
	    let mut feedbackstyle: Option<ContentRef> = None;
	    let mut feedbackvariables: Option<ContentRef> = None;
	    let mut nodes: Vec<STACKPrtNode> = Vec::new();

	    // First the simple ones
		let elems = self._get_elements(node, vec![
			"name".to_string(),
			"value".to_string(),
			"autosimplify".to_string(),
			"feedbackstyle".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"name" => { 
								// Here we have a bit of a problem as there is a <name> inside
								// the nodes and we don't restrict the search to direct children
								// but the library returns stuff in document order so.
								if name == None {
									name = cref;
								}
							},
							"value" => { value = cref; },
							"autosimplify" => { autosimplify = cref; },
							"feedbackstyle" => { feedbackstyle = cref; },
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				_ => { panic!("Unexpected type.");}
			}
		}

		// Then the <text> and nodes.
		for n in node.children() {
			if n.is_element() && n.tag_name().name() == "node" {
				nodes.push(self.into_stack_prt_node(n));
			} else if n.is_element() && n.tag_name().name() == "feedbackvariables" {
				let text = &self._get_elements(n, vec!["text".to_string()])[0];
                if let ContentType::Element(_,_,items) = text {
                    let econtent = &items[0];
                    if let ContentType::ElementContent(c) = econtent {
                		feedbackvariables = Some(c.clone());
                    }
                }
			}
		}

		STACKPrt {
		    name: name.expect("Missing name element."),
		    value: value.expect("Missing value element."),
		    autosimplify: autosimplify.expect("Missing autosimplify element."),
		    feedbackstyle: feedbackstyle.expect("Missing feedbackstyle element."),
		    feedbackvariables: feedbackvariables.expect("Missing feedbackvariables element."),
		   	nodes: nodes
		}
	}


	fn into_stack_prt_node(&mut self, node: roxmltree::Node) -> STACKPrtNode {
	    let mut name: Option<ContentRef> = None;
	    let mut answertest: Option<ContentRef> = None;
	    let mut sans: Option<ContentRef> = None;
	    let mut tans: Option<ContentRef> = None;
	    let mut testoptions: Option<ContentRef> = None;
	    let mut quiet: Option<ContentRef> = None;
	    let mut truescoremode: Option<ContentRef> = None;
	    let mut truescore: Option<ContentRef> = None;
	    let mut truepenalty: Option<ContentRef> = None;
	    let mut truenextnode: Option<ContentRef> = None;
	    let mut trueanswernote: Option<ContentRef> = None;
	    let mut truefeedback: Option<ContentType> = None;
	    let mut falsescoremode: Option<ContentRef> = None;
	    let mut falsescore: Option<ContentRef> = None;
	    let mut falsepenalty: Option<ContentRef> = None;
	    let mut falsenextnode: Option<ContentRef> = None;
	    let mut falseanswernote: Option<ContentRef> = None;
	    let mut falsefeedback: Option<ContentType> = None; 


		// Collect the simpler elements.
		let elems = self._get_elements(node, vec![
		    "name".to_string(),
		    "answertest".to_string(),
		    "sans".to_string(),
		    "tans".to_string(),
		    "testoptions".to_string(),
		    "quiet".to_string(),
		    "truescoremode".to_string(),
		    "truescore".to_string(),
		    "truepenalty".to_string(),
		    "truenextnode".to_string(),
		    "trueanswernote".to_string(),
		    "truefeedback".to_string(),
		    "falsescoremode".to_string(),
		    "falsescore".to_string(),
		    "falsepenalty".to_string(),
		    "falsenextnode".to_string(),
		    "falseanswernote".to_string(),
		    "falsefeedback".to_string()
		]);


		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
						    "name" => { name = cref; }
						    "answertest" => { answertest = cref; }
						    "sans" => { sans = cref; }
						    "tans" => { tans = cref; }
						    "testoptions" => { testoptions = cref; }
						    "quiet" => { quiet = cref; }
						    "truescoremode" => { truescoremode = cref; }
						    "truescore" => { truescore = cref; }
						    "truepenalty" => { truepenalty = cref; }
						    "truenextnode" => { truenextnode = cref; }
						    "trueanswernote" => { trueanswernote = cref; }
						    "falsescoremode" => { falsescoremode = cref; }
						    "falsescore" => { falsescore = cref; }
						    "falsepenalty" => { falsepenalty = cref; }
						    "falsenextnode" => { falsenextnode = cref; }
						    "falseanswernote" => { falseanswernote = cref; }
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				ContentType::MoodleTextElement(ref nam, _, _) => {
					match nam.as_str() {
						"truefeedback" => { truefeedback = Some(el); }
						"falsefeedback" => { falsefeedback = Some(el); }
						_ => { panic!("Unexpected MoodleTextElement."); }
					}
				},
				_ => {
					panic!("Unexpected type.");
				}
			}
		}

		STACKPrtNode {
		    name: name.expect("Missing name element."),
		    answertest: answertest.expect("Missing answertest element."),
		    sans: sans.expect("Missing sans element."),
		    tans: tans.expect("Missing tans element."),
		    testoptions: testoptions.expect("Missing testoptions element."),
		    quiet: quiet.expect("Missing quiet element."),
		    truescoremode: truescoremode.expect("Missing truescoremode element."),
		    truescore: truescore.expect("Missing truescore element."),
		    truepenalty: truepenalty.expect("Missing truepenalty element."),
		    truenextnode: truenextnode.expect("Missing truenextnode element."),
		    trueanswernote: trueanswernote.expect("Missing trueanswernote element."),
		    truefeedback: truefeedback.expect("Missing truefeedback element."),
		    falsescoremode: falsescoremode.expect("Missing falsescoremode element."),
		    falsescore: falsescore.expect("Missing falsescore element."),
		    falsepenalty: falsepenalty.expect("Missing falsepenalty element."),
		    falsenextnode: falsenextnode.expect("Missing falsenextnode element."),
		    falseanswernote: falseanswernote.expect("Missing falseanswernote element."),
		    falsefeedback: falsefeedback.expect("Missing falsefeedback element.")
		}
	}

	fn into_stack_qtest(&mut self, node: roxmltree::Node) -> STACKQtest {
		let mut testcase: Option<ContentRef> = None;
    	let mut description: Option<ContentRef> = None;
		let mut inputs: HashMap<String, STACKQtestInput> = HashMap::new();
		let mut expected: HashMap<String, STACKQtestExpected> = HashMap::new();

	    // First the simple ones
		let elems = self._get_elements(node, vec![
			"testcase".to_string(),
			"description".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"testcase" => { testcase = cref; },
							"description" => { description = cref; },
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				_ => { panic!("Unexpected type.");}
			}
		}

		// Then the others.
		for n in node.children() {
			if n.is_element() && n.tag_name().name() == "testinput" {
				let testinput = self.into_stack_qtest_input(n);
				inputs.insert(testinput.name.content.clone(), testinput);
			} else if n.is_element() && n.tag_name().name() == "expected" {
				let expectation = self.into_stack_qtest_expected(n);
				expected.insert(expectation.name.content.clone(), expectation);
			}	
		}

		STACKQtest {
			testcase: testcase.expect("Missing testcase element."),
			description: description.expect("Missing description element."),
			inputs: inputs,
			expected: expected
		}
	}

	fn into_stack_qtest_input(&mut self, node: roxmltree::Node) -> STACKQtestInput {
		let mut name: Option<ContentRef> = None;
		let mut value: Option<ContentRef> = None;

		let elems = self._get_elements(node, vec![
			"name".to_string(),
			"value".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"name" => { name = cref; },
							"value" => { value = cref; },
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				_ => { panic!("Unexpected type.");}
			}
		}

		STACKQtestInput {
		    name: name.expect("Missing name element."),
		    value: value.expect("Missing value element.")
		}
	}

	fn into_stack_qtest_expected(&mut self, node: roxmltree::Node) -> STACKQtestExpected {
		let mut name: Option<ContentRef> = None;
		let mut expectedscore: Option<ContentRef> = None;
		let mut expectedpenalty: Option<ContentRef> = None;
		let mut expectedanswernote: Option<ContentRef> = None;

		let elems = self._get_elements(node, vec![
			"name".to_string(),
			"expectedscore".to_string(),
			"expectedpenalty".to_string(),
			"expectedanswernote".to_string()
		]);

		for el in elems {
			match el {
				ContentType::Element(nam, _, attr_contents) => {
					// These should not have inner struct.
					if let ContentType::ElementContent(c) = attr_contents.last().unwrap() {
						let cref = Some(c.clone());
						match nam.as_str() {
							"name" => { name = cref; },
							"expectedscore" => { expectedscore = cref; },
							"expectedpenalty" => { expectedpenalty = cref; },
							"expectedanswernote" => { expectedanswernote = cref; },
							_ => { panic!("Unexpected Element."); }
						}
					}
				},
				_ => { panic!("Unexpected type.");}
			}
		}

		STACKQtestExpected {
		    name: name.expect("Missing name element."),
		    expectedscore: expectedscore.expect("Missing expectedscore element."),
		    expectedpenalty: expectedpenalty.expect("Missing expectedpenalty element."),
		    expectedanswernote: expectedanswernote.expect("Missing expectedanswernote element.")
		}
	}
}
