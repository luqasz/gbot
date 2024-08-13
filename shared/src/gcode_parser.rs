use gcode;

enum Error {
    UnknownContent(gcode::Span),
    UnexpectedLineNum(gcode::Span),
    ArgNoCmd(gcode::Span),
    NumNoLetter(gcode::Span),
    LetterNoNum(gcode::Span),
}

type Elist = heapless::Vec<Error, 10>;

struct ParseErrors {
    unknown_content: usize,
    unexpected_line_number: usize,
    argument_without_command: usize,
    number_without_letter: usize,
    letter_without_number: usize,
    buffer_overflow: usize,
}

impl gcode::Callbacks for ParseErrors {
    fn gcode_buffer_overflowed(
        &mut self,
        _mnemonic: gcode::Mnemonic,
        _major_number: u32,
        _minor_number: u32,
        _arguments: &[gcode::Word],
        _span: gcode::Span,
    ) {
        self.buffer_overflow += 1;
    }

    fn gcode_argument_buffer_overflowed(
        &mut self,
        _mnemonic: gcode::Mnemonic,
        _major_number: u32,
        _minor_number: u32,
        _argument: gcode::Word,
    ) {
        self.buffer_overflow += 1;
    }

    fn comment_buffer_overflow(
        &mut self,
        _comment: gcode::Comment,
    ) {
        self.buffer_overflow += 1;
    }

    fn unknown_content(
        &mut self,
        _text: &str,
        _span: gcode::Span,
    ) {
        self.unknown_content += 1;
    }

    fn unexpected_line_number(
        &mut self,
        _line_number: f32,
        _span: gcode::Span,
    ) {
        self.unexpected_line_number += 1;
    }

    fn argument_without_a_command(
        &mut self,
        _letter: char,
        _value: f32,
        _span: gcode::Span,
    ) {
        self.argument_without_command += 1;
    }

    fn number_without_a_letter(
        &mut self,
        _value: &str,
        _span: gcode::Span,
    ) {
        self.number_without_letter += 1;
    }

    fn letter_without_a_number(
        &mut self,
        _value: &str,
        _span: gcode::Span,
    ) {
        self.letter_without_number += 1;
    }
}
