use std::io;

enum NoteName {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl NoteName {
    fn to_chromatic_index(&self) -> u8 {
        match self {
            NoteName::C => 0,
            NoteName::CSharp => 1,
            NoteName::D => 2,
            NoteName::DSharp => 3,
            NoteName::E => 4,
            NoteName::F => 5,
            NoteName::FSharp => 6,
            NoteName::G => 7,
            NoteName::GSharp => 8,
            NoteName::A => 9,
            NoteName::ASharp => 10,
            NoteName::B => 11,
        }
    }

    fn from_chromatic_index(index: u8) -> Self {
        match index {
            0 => NoteName::C,
            1 => NoteName::CSharp,
            2 => NoteName::D,
            3 => NoteName::DSharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::FSharp,
            7 => NoteName::G,
            8 => NoteName::GSharp,
            9 => NoteName::A,
            10 => NoteName::ASharp,
            11 => NoteName::B,
            _ => panic!("Invalid chromatic index"),
        }
    }
}

struct Note {
    name: NoteName,
    octave: u8,
}

impl Note {
    fn to_midi(&self) -> u8 {
        self.name.to_chromatic_index() + (self.octave + 1) * 12
    }

    fn from_midi(midi: u8) -> Self {
        let name = NoteName::from_chromatic_index(midi % 12);
        let octave = midi / 12 - 1;
        Self { name, octave }
    }

    fn to_string(&self) -> String {
        let note_name = match self.name {
            NoteName::C => "C",
            NoteName::CSharp => "C#",
            NoteName::D => "D",
            NoteName::DSharp => "D#",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::FSharp => "F#",
            NoteName::G => "G",
            NoteName::GSharp => "G#",
            NoteName::A => "A",
            NoteName::ASharp => "A#",
            NoteName::B => "B",
        };
        format!("{}{}", note_name, self.octave)
    }

    fn from_string(string: &str) -> Result<Self, String> {
        if string.is_empty() {
            return Err("Empty input".to_string());
        }

        let mut note_end = 1;
        if string.len() > 1
            && (string.chars().nth(1) == Some('#') || string.chars().nth(1) == Some('b'))
        {
            note_end = 2;
        }

        if note_end >= string.len() {
            return Err("No octave specified".to_string());
        }

        let note_name = &string[..note_end];
        let octave_str = &string[note_end..];

        let octave = octave_str
            .parse::<u8>()
            .map_err(|_| format!("Invalid octave: {}", octave_str))?;

        let name = match note_name {
            "C" => NoteName::C,
            "C#" => NoteName::CSharp,
            "D" => NoteName::D,
            "D#" => NoteName::DSharp,
            "E" => NoteName::E,
            "F" => NoteName::F,
            "F#" => NoteName::FSharp,
            "G" => NoteName::G,
            "G#" => NoteName::GSharp,
            "A" => NoteName::A,
            "A#" => NoteName::ASharp,
            "B" => NoteName::B,
            _ => return Err(format!("Invalid note name: {}", note_name)),
        };

        Ok(Self { name, octave })
    }
}

fn main() {
    let mut input = String::new();

    println!("Enter a note: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read note");

    let trimmed_input = input.trim();
    let note = Note::from_string(trimmed_input).expect("Failed to parse note");
    println!("{} = {}", note.to_string(), note.to_midi());

    input.clear();

    println!("Enter a MIDI value: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read MIDI value");

    let midi = input
        .trim()
        .parse::<u8>()
        .expect("Failed to parse MIDI value");

    if midi > 127 {
        println!("Error: MIDI values must be between 0-127, got {}", midi);
        std::process::exit(1);
    }

    println!("MIDI {} = {}", midi, Note::from_midi(midi).to_string());
}
