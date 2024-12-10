use ratatui::style::Color;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct LsColours {
    pub norm: ColourEntry,
    pub rs: ColourEntry,
    pub file: ColourEntry,
    pub dir: ColourEntry,
    pub link: ColourEntry,
    pub pipe: ColourEntry,
    pub door: ColourEntry,
    pub block: ColourEntry,
    pub char: ColourEntry,
    pub orphan: ColourEntry,
    pub sock: ColourEntry,
    pub setuid: ColourEntry,
    pub setgid: ColourEntry,
    pub sticky_other_writable: ColourEntry,
    pub other_writable: ColourEntry,
    pub sticky: ColourEntry,
    pub exec: ColourEntry,
    pub missing: ColourEntry,
    pub extensions: Vec<(String, ColourEntry)>,
}

impl LsColours {
    pub fn parse(&self) -> String {
        let mut out = String::new();

        out.push_str(&format!("no={}:", self.norm.parse()));
        out.push_str(&format!("rs={}:", self.rs.parse()));
        out.push_str(&format!("fi={}:", self.file.parse()));
        out.push_str(&format!("di={}:", self.dir.parse()));
        out.push_str(&format!("ln={}:", self.link.parse()));
        out.push_str(&format!("pi={}:", self.pipe.parse()));
        out.push_str(&format!("do={}:", self.door.parse()));
        out.push_str(&format!("bd={}:", self.block.parse()));
        out.push_str(&format!("cd={}:", self.char.parse()));
        out.push_str(&format!("or={}:", self.orphan.parse()));
        out.push_str(&format!("so={}:", self.sock.parse()));
        out.push_str(&format!("su={}:", self.setuid.parse()));
        out.push_str(&format!("sg={}:", self.setgid.parse()));
        out.push_str(&format!("tw={}:", self.sticky_other_writable.parse()));
        out.push_str(&format!("ow={}:", self.other_writable.parse()));
        out.push_str(&format!("st={}:", self.sticky.parse()));
        out.push_str(&format!("ex={}:", self.exec.parse()));
        out.push_str(&format!("mi={}:", self.missing.parse()));

        for ext in &self.extensions {
            out.push_str(&format!("*{}={}:", ext.0, ext.1.parse()));
        }

        // Remove trailing ':'
        out.pop();

        out
    }
}

impl Default for LsColours {
    fn default() -> Self {
        Self {
            norm: ColourEntry::new(Effects::Default, Some(TerminalColour::White), None),
            rs: ColourEntry::new(Effects::Default, Some(TerminalColour::White), None),
            file: ColourEntry::new(Effects::Default, Some(TerminalColour::LightBlue), None),
            dir: ColourEntry::new(Effects::Bold, Some(TerminalColour::Blue), None),
            link: ColourEntry::new(Effects::Default, Some(TerminalColour::Gray), None),
            pipe: ColourEntry::new(Effects::Default, Some(TerminalColour::LightPurple), None),
            door: ColourEntry::new(Effects::Default, Some(TerminalColour::Purple), None),
            block: ColourEntry::new(Effects::Default, Some(TerminalColour::Yellow), None),
            char: ColourEntry::new(Effects::Bold, Some(TerminalColour::Yellow), None),
            orphan: ColourEntry::new(Effects::Default, Some(TerminalColour::Red), None),
            sock: ColourEntry::new(Effects::Bold, Some(TerminalColour::Orange), None),
            setuid: ColourEntry::new(Effects::Default, Some(TerminalColour::Blue), None),
            setgid: ColourEntry::new(Effects::Default, Some(TerminalColour::Blue), None),
            sticky_other_writable: ColourEntry::new(
                Effects::Default,
                Some(TerminalColour::Blue),
                None,
            ),
            other_writable: ColourEntry::new(
                Effects::Default,
                Some(TerminalColour::LightBlue),
                None,
            ),
            sticky: ColourEntry::new(Effects::Bold, Some(TerminalColour::LightBlue), None),
            exec: ColourEntry::new(Effects::Default, Some(TerminalColour::Green), None),
            missing: ColourEntry::new(Effects::Default, Some(TerminalColour::Red), None),
            extensions: vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct ColourEntry {
    pub effects: Effects,
    pub fg: Option<TerminalColour>,
    pub bg: Option<TerminalColour>,
}

impl ColourEntry {
    pub fn new(effects: Effects, fg: Option<TerminalColour>, bg: Option<TerminalColour>) -> Self {
        Self { effects, fg, bg }
    }

    pub fn parse(&self) -> String {
        let mut out = String::new();

        out.push_str(match self.effects {
            Effects::Default => "00;",
            Effects::Bold => "01;",
            Effects::Underline => "04;",
            Effects::Italic => "03;",
            Effects::Strikethrough => "09;",
        });

        if self.fg.is_some() {
            out.push_str(match self.fg.unwrap() {
                TerminalColour::Black => "30;",
                TerminalColour::Red => "31;",
                TerminalColour::Green => "32;",
                TerminalColour::Orange => "33;",
                TerminalColour::Blue => "34;",
                TerminalColour::Purple => "35;",
                TerminalColour::Cyan => "36;",
                TerminalColour::Gray => "37;",
                TerminalColour::DarkGray => "90;",
                TerminalColour::LightRed => "91;",
                TerminalColour::LightGreen => "92;",
                TerminalColour::Yellow => "93;",
                TerminalColour::LightBlue => "94;",
                TerminalColour::LightPurple => "95;",
                TerminalColour::Turquoise => "96;",
                TerminalColour::White => "97;",
            })
        }

        if self.bg.is_some() {
            out.push_str(match self.bg.unwrap() {
                TerminalColour::Black => "40;",
                TerminalColour::Red => "41;",
                TerminalColour::Green => "42;",
                TerminalColour::Orange => "43;",
                TerminalColour::Blue => "44;",
                TerminalColour::Purple => "45;",
                TerminalColour::Cyan => "46;",
                TerminalColour::Gray => "47;",
                TerminalColour::DarkGray => "100;",
                TerminalColour::LightRed => "101;",
                TerminalColour::LightGreen => "102;",
                TerminalColour::Yellow => "103;",
                TerminalColour::LightBlue => "104;",
                TerminalColour::LightPurple => "105;",
                TerminalColour::Turquoise => "106;",
                TerminalColour::White => "107;",
            })
        }

        // Remove trailing semicolon
        out.pop();

        out
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum Effects {
    Default,
    Bold,
    Underline,
    Italic,
    Strikethrough,
}

impl Effects {
    pub const ORDER: [Effects; 5] = [
        Effects::Default,
        Effects::Bold,
        Effects::Underline,
        Effects::Italic,
        Effects::Strikethrough,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub enum TerminalColour {
    Black,
    Red,
    Green,
    Orange,
    Blue,
    Purple,
    Cyan,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    Yellow,
    LightBlue,
    LightPurple,
    Turquoise,
    White,
}

impl TerminalColour {
    pub const ORDER: [TerminalColour; 16] = [
        TerminalColour::Red,
        TerminalColour::Green,
        TerminalColour::Orange,
        TerminalColour::Blue,
        TerminalColour::Purple,
        TerminalColour::Cyan,
        TerminalColour::Gray,
        TerminalColour::Black,
        TerminalColour::DarkGray,
        TerminalColour::LightRed,
        TerminalColour::LightGreen,
        TerminalColour::Yellow,
        TerminalColour::LightBlue,
        TerminalColour::LightPurple,
        TerminalColour::Turquoise,
        TerminalColour::White,
    ];
}

impl Into<Color> for TerminalColour {
    fn into(self) -> Color {
        match self {
            crate::colours::TerminalColour::Black => Color::Black,
            crate::colours::TerminalColour::Red => Color::Red,
            crate::colours::TerminalColour::Green => Color::Green,
            crate::colours::TerminalColour::Orange => Color::Yellow,
            crate::colours::TerminalColour::Blue => Color::Blue,
            crate::colours::TerminalColour::Purple => Color::Magenta,
            crate::colours::TerminalColour::Cyan => Color::Cyan,
            crate::colours::TerminalColour::Gray => Color::Gray,
            crate::colours::TerminalColour::DarkGray => Color::DarkGray,
            crate::colours::TerminalColour::LightRed => Color::LightRed,
            crate::colours::TerminalColour::LightGreen => Color::LightGreen,
            crate::colours::TerminalColour::Yellow => Color::LightYellow,
            crate::colours::TerminalColour::LightBlue => Color::LightBlue,
            crate::colours::TerminalColour::LightPurple => Color::LightMagenta,
            crate::colours::TerminalColour::Turquoise => Color::LightCyan,
            crate::colours::TerminalColour::White => Color::White,
        }
    }
}

impl ToString for TerminalColour {
    fn to_string(&self) -> String {
        match self {
            TerminalColour::Black => "Black",
            TerminalColour::Red => "Red",
            TerminalColour::Green => "Green",
            TerminalColour::Orange => "Orange",
            TerminalColour::Blue => "Blue",
            TerminalColour::Purple => "Purple",
            TerminalColour::Cyan => "Cyan",
            TerminalColour::Gray => "Gray",
            TerminalColour::DarkGray => "DarkGray",
            TerminalColour::LightRed => "LightRed",
            TerminalColour::LightGreen => "LightGreen",
            TerminalColour::Yellow => "Yellow",
            TerminalColour::LightBlue => "LightBlue",
            TerminalColour::LightPurple => "LightPurple",
            TerminalColour::Turquoise => "Turquoise",
            TerminalColour::White => "White",
        }
        .to_string()
    }
}
