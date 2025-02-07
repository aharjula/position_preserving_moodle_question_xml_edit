# position_preserving_moodle_question_xml_edit well that is a name...

This is a library for those of us that need to do small modifications on Moodle question.xml files and don't want to reimplement the parsing every time. Or don't yet trust that AI would implement it safely every time.

The thing that makes this different from other XML-parsers is that when we modify things we are not actually modifying a DOM of any sorts, we simply write Strings on top of the original items positions, thus leaving the rest of the document alone. Not touching the rest of the XML-document makes this somewhat diff-friendly and will probably keep your version control diffs readable.

## For who?

This will primarily target [STACK](https://stack-assessment.org/)-questions but will happily deal with any other question-formats it sees. In particular, this will probably be of use to those building quick modification scripts working on their [gitsync](https://github.com/maths/moodle-qbank_gitsync)-clones of large question-banks.

## Current state of development

This is the first release 0.1.0, hardly tested or used and written as pretty much the first rust thing of the author, so be careful and don't get too revulsed by what you see in the code.

# Docs...

Check the tests, those should tell everything worth knowing. Or compile the rustdoc for a list of things.
