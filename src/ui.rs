use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{
    style::{Color, Modifier, Stylize},
    text::Line,
    widgets::*,
    DefaultTerminal, Frame,
};

use crate::{colours::ColourEntry, file::SaveFile, key_events};

const FG_COL: Color = Color::White;

pub struct App {
    pub exit: bool,
    pub savefile: SaveFile,
    pub open_scheme: Option<String>,
    pub dialog_state: DialogState,
    pub content_loc: i32,
    pub active_lce: Option<ListColourEntry>,
}

impl Default for App {
    fn default() -> Self {
        App {
            exit: false,
            open_scheme: None,
            savefile: SaveFile::load(),
            dialog_state: DialogState::Closed,
            content_loc: 0,
            active_lce: None,
        }
    }
}

pub enum DialogState {
    Closed,
    PickScheme(u8),
    NewScheme(String),
    NewExtension(String),
    EditingColour(u8),
    Export,
}

#[derive(Debug, Clone)]
pub struct ListColourEntry {
    pub entry: ColourEntry,
    pub text: String,
    pub description: String,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let title = Line::from("lscoltui".bold());
        #[rustfmt::skip]
        let instructions = Line::from(vec![
            " Open Scheme ".into(), "<F1> ".blue().bold(),
            " New Scheme ".into(), "<F2> ".blue().bold(),
            " Add Extension ".into(), "<F3> ".blue().bold(),
            " Export Scheme ".into(), "<F4> ".blue().bold(),
            " Quit ".into(), "<F5> ".blue().bold(),
            " Edit Colour ".into(), "<Enter>".blue().bold(),
            " Delete Extension ".into(), "<Delete>".blue().bold(),
        ]);

        let mut colour_lines = vec![];

        if self.open_scheme.is_some() {
            let active_scheme = &self
                .savefile
                .schemes
                .iter()
                .find(|x| &x.0 == self.open_scheme.as_ref().unwrap())
                .unwrap()
                .1;

            if self.content_loc >= active_scheme.extensions.len() as i32 + 18 {
                self.content_loc = 0;
            }

            if self.content_loc < 0 {
                self.content_loc = 18 + active_scheme.extensions.len() as i32 - 1;
            }

            let normal = ListColourEntry {
                entry: active_scheme.norm,
                text: "Normal".to_string(),
                description:
                    "Global default, though everything should be set so it should be rarely used"
                        .to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                normal,
                self.content_loc,
                0,
                &mut self.active_lce,
            ));

            let file = ListColourEntry {
                entry: active_scheme.file,
                text: "File".to_string(),
                description: "Normal file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                file,
                self.content_loc,
                1,
                &mut self.active_lce,
            ));

            let dir = ListColourEntry {
                entry: active_scheme.dir,
                text: "Directory".to_string(),
                description: "Normal directory".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                dir,
                self.content_loc,
                2,
                &mut self.active_lce,
            ));

            let link = ListColourEntry {
                entry: active_scheme.link,
                text: "Symlink".to_string(),
                description: "A symbolic link to another file on the filesystem".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                link,
                self.content_loc,
                3,
                &mut self.active_lce,
            ));

            let pipe = ListColourEntry {
                entry: active_scheme.pipe,
                text: "Pipe".to_string(),
                description: "A named pipe".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                pipe,
                self.content_loc,
                4,
                &mut self.active_lce,
            ));

            let door = ListColourEntry {
                entry: active_scheme.door,
                text: "Door".to_string(),
                description: "A door file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                door,
                self.content_loc,
                5,
                &mut self.active_lce,
            ));

            let block = ListColourEntry {
                entry: active_scheme.block,
                text: "Block".to_string(),
                description: "A block device file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                block,
                self.content_loc,
                6,
                &mut self.active_lce,
            ));

            let char = ListColourEntry {
                entry: active_scheme.char,
                text: "Character".to_string(),
                description: "A character device file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                char,
                self.content_loc,
                7,
                &mut self.active_lce,
            ));

            let orphan = ListColourEntry {
                entry: active_scheme.orphan,
                text: "Orphaned Symlink".to_string(),
                description: "A symbolic link pointing to a non-existent file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                orphan,
                self.content_loc,
                8,
                &mut self.active_lce,
            ));

            let socket = ListColourEntry {
                entry: active_scheme.sock,
                text: "Socket".to_string(),
                description: "A socket file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                socket,
                self.content_loc,
                9,
                &mut self.active_lce,
            ));

            let setuid = ListColourEntry {
                entry: active_scheme.setuid,
                text: "SetUID".to_string(),
                description: "A file with the SetUID bit enabled".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                setuid,
                self.content_loc,
                10,
                &mut self.active_lce,
            ));

            let setgid = ListColourEntry {
                entry: active_scheme.setgid,
                text: "SetGID".to_string(),
                description: "A file with the SetGID bit enabled".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                setgid,
                self.content_loc,
                11,
                &mut self.active_lce,
            ));

            let sticky_other_writable = ListColourEntry {
                entry: active_scheme.sticky_other_writable,
                text: "Sticky Other Writable".to_string(),
                description: "A directory that is sticky (only the owner can delete files), but that others can write to".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                sticky_other_writable,
                self.content_loc,
                12,
                &mut self.active_lce,
            ));

            let other_writable = ListColourEntry {
                entry: active_scheme.other_writable,
                text: "Other Writable".to_string(),
                description: "A directory that isn't sticky, and others can write to it"
                    .to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                other_writable,
                self.content_loc,
                13,
                &mut self.active_lce,
            ));

            let sticky = ListColourEntry {
                entry: active_scheme.sticky,
                text: "Sticky".to_string(),
                description: "A directory that is sticky (only the owner can delete files), but that others are unable to write to".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                sticky,
                self.content_loc,
                14,
                &mut self.active_lce,
            ));

            let exec = ListColourEntry {
                entry: active_scheme.exec,
                text: "Executable".to_string(),
                description: "An executable file".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                exec,
                self.content_loc,
                15,
                &mut self.active_lce,
            ));

            let missing = ListColourEntry {
                entry: active_scheme.missing,
                text: "Missing".to_string(),
                description: "A non-existent file pointed to by a symbolic link".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                missing,
                self.content_loc,
                16,
                &mut self.active_lce,
            ));

            let endcode = ListColourEntry {
                entry: active_scheme.rs,
                text: "End Code".to_string(),
                description: "Text that isn't filenames (e.g. \'/\')".to_string(),
            };
            colour_lines.push(colour_entry_to_line(
                endcode,
                self.content_loc,
                17,
                &mut self.active_lce,
            ));

            let _ = active_scheme
                .extensions
                .iter()
                .enumerate()
                .map(|(i, x)| {
                    let y = ListColourEntry {
                        entry: x.1,
                        text: x.clone().0,
                        description: format!("Colour of .{} files", x.clone().0),
                    };
                    colour_lines.push(colour_entry_to_line(
                        y,
                        self.content_loc,
                        i as i32 + 18,
                        &mut self.active_lce,
                    ));
                })
                .collect::<Vec<_>>();
        }

        let content_block = Block::new()
            .padding(Padding::proportional(area.width / 64))
            .title(title.centered())
            .title_bottom(instructions.centered());

        let content_inner = content_block.clone().inner(area);

        if let DialogState::Closed = self.dialog_state {
        } else {
            colour_lines = vec![]
        }

        let content = Paragraph::new(colour_lines)
            .left_aligned()
            .wrap(Wrap { trim: false });

        let content_desc = Paragraph::new(match self.active_lce.clone() {
            Some(x) => x.description,
            None => String::new(),
        })
        .right_aligned()
        .wrap(Wrap { trim: true });

        let mut dialog_instructions = Line::raw("content");
        let dialog = match &self.dialog_state {
            DialogState::Closed => Paragraph::new("").centered(),
            DialogState::PickScheme(x) => {
                let schemes = self.savefile.schemes.clone();
                let mut lines = vec![];
                let mut i = 0;
                while i < schemes.len() {
                    lines.push(Line::raw(schemes[i].0.clone()).white().fg(
                        match i == *x as usize {
                            true => Color::LightBlue,
                            false => FG_COL,
                        },
                    ));

                    i += 1;
                }

                if schemes.len() == 0 {
                    lines.push(Line::raw("You have no schemes. Create one!"))
                }

                #[rustfmt::skip]
                let instructions = Line::from(vec![
                    " Move Up ".into(), "<Up> ".blue().bold(),
                    " Move Down ".into(), "<Down> ".blue().bold(),
                    " Select ".into(), "<Enter> ".blue().bold(),
                    " Delete ".into(), "<Delete> ".blue().bold(),
                    " Exit ".into(), "<Escape> ".blue().bold(),
                ]);
                dialog_instructions = instructions;

                Paragraph::new(lines).left_aligned()
            }
            DialogState::NewScheme(s) => {
                #[rustfmt::skip]
                let instructions = Line::from(vec![
                    " Create ".into(), "<Enter> ".blue().bold(),
                    " Exit ".into(), "<Escape> ".blue().bold(),
                ]);
                dialog_instructions = instructions;
                Paragraph::new(vec![
                    Line::raw("Enter name of new scheme: ").bold(),
                    Line::raw(format!("{}|", s)),
                    if self.savefile.schemes.iter().find(|x| &x.0 == s).is_some() {
                        Line::raw("Scheme of same name exists").red()
                    } else if s.is_empty() {
                        Line::raw("Name cannot be empty").red()
                    } else {
                        Line::raw("")
                    },
                ])
                .centered()
            }
            DialogState::NewExtension(s) => {
                if self.open_scheme.as_ref().is_some() {
                    let active_scheme = &self
                        .savefile
                        .schemes
                        .iter()
                        .find(|x| &x.0 == self.open_scheme.as_ref().unwrap())
                        .unwrap()
                        .1;

                    #[rustfmt::skip]
                    let instructions = Line::from(vec![
                        " Create ".into(), "<Enter> ".blue().bold(),
                        " Exit ".into(), "<Escape> ".blue().bold(),
                    ]);

                    dialog_instructions = instructions;
                    Paragraph::new(vec![
                        Line::raw("Enter name of new extension: ").bold(),
                        Line::raw(format!("{}|", s)),
                        if active_scheme
                            .extensions
                            .iter()
                            .find(|x| &x.0 == s)
                            .is_some()
                        {
                            Line::raw("Extension of same name exists").red()
                        } else if s.is_empty() {
                            Line::raw("Name cannot be empty").red()
                        } else {
                            Line::raw("")
                        },
                    ])
                    .centered()
                } else {
                    self.dialog_state = DialogState::Closed;
                    Paragraph::new(vec![])
                }
            }
            DialogState::EditingColour(i) => match &self.active_lce {
                Some(lce) => {
                    let entry = lce.entry;

                    #[rustfmt::skip]
                    let instructions = Line::from(vec![
                        " Switch Backwards ".into(), "<Left> ".blue().bold(),
                        " Switch Forwards ".into(), "<Right> ".blue().bold(),
                        " Exit (Changes automatically saved) ".into(), "<Escape> ".blue().bold(),
                    ]);

                    dialog_instructions = instructions;

                    let mut lines = vec![];
                    lines.push(Line::raw(format!("Edit {}", lce.text)).bold());

                    lines.push(Line::raw(""));

                    lines.push(
                        Line::raw("Preview")
                            .fg(match entry.fg {
                                Some(c) => c.into(),
                                None => Color::Reset,
                            })
                            .bg(match entry.bg {
                                Some(c) => c.into(),
                                None => Color::Reset,
                            })
                            .add_modifier(match entry.effects {
                                crate::colours::Effects::Default => Modifier::empty(),
                                crate::colours::Effects::Bold => Modifier::BOLD,
                                crate::colours::Effects::Underline => Modifier::UNDERLINED,
                                crate::colours::Effects::Italic => Modifier::ITALIC,
                                crate::colours::Effects::Strikethrough => Modifier::CROSSED_OUT,
                            }),
                    );

                    lines.push(Line::raw(""));

                    lines.push(
                        Line::raw(format!(
                            "Foreground: {}",
                            match entry.fg {
                                Some(c) => c.to_string(),
                                None => "None".to_string(),
                            }
                        ))
                        .fg(match entry.fg {
                            Some(c) => c.into(),
                            None => Color::Reset,
                        })
                        .add_modifier(match *i == 0 {
                            true => Modifier::UNDERLINED,
                            false => Modifier::empty(),
                        }),
                    );

                    lines.push(
                        Line::raw(format!(
                            "Background: {}",
                            match entry.bg {
                                Some(c) => c.to_string(),
                                None => "None".to_string(),
                            }
                        ))
                        .fg(match entry.bg {
                            Some(c) => c.into(),
                            None => Color::Reset,
                        })
                        .add_modifier(match *i == 1 {
                            true => Modifier::UNDERLINED,
                            false => Modifier::empty(),
                        }),
                    );

                    lines.push(
                        Line::raw(format!(
                            "Effect: {}",
                            match entry.effects {
                                crate::colours::Effects::Default => "None",
                                crate::colours::Effects::Bold => "Bold",
                                crate::colours::Effects::Underline => "Underline",
                                crate::colours::Effects::Italic => "Italic",
                                crate::colours::Effects::Strikethrough => "Strikethrough",
                            }
                        ))
                        .add_modifier(match *i == 2 {
                            true => Modifier::UNDERLINED,
                            false => Modifier::empty(),
                        }),
                    );

                    Paragraph::new(lines)
                }
                None => Paragraph::new(vec![]),
            },
            DialogState::Export => {
                #[rustfmt::skip]
                let instructions = Line::from(vec![
                    " Exit ".into(), "<Escape> ".blue().bold(),
                ]);

                dialog_instructions = instructions;

                let mut lines = vec![];
                lines.push(Line::raw("Export Colours").bold());

                lines.push(Line::raw(""));

                lines.push(Line::raw("Here is the command to set the colours of ls:").bold());

                lines.push(Line::raw(format!(
                    "export LS_COLORS=\'{}\'",
                    self.savefile
                        .schemes
                        .iter()
                        .find(|x| &x.0 == self.open_scheme.as_ref().unwrap())
                        .unwrap()
                        .1
                        .parse()
                )));

                lines.push(Line::raw(""));

                lines.push(Line::raw(format!(
                    "To run this automatically, place \'eval $(lscoltui export {})\' in your terminal startup file, such as .bashrc for Bash.",
                    self.open_scheme.as_ref().unwrap()
                )));

                Paragraph::new(lines).wrap(Wrap { trim: true })
            }
        };

        let dialog_block = Block::bordered()
            .padding(Padding::proportional(content_inner.width / 64))
            .border_type(BorderType::Rounded)
            .title_bottom(dialog_instructions);

        frame.render_widget(content.clone().block(content_block.clone()), area);

        if self.open_scheme.is_some() {
            frame.render_widget(content_desc.clone().block(content_block), area);
        }

        if let DialogState::Closed = self.dialog_state {
        } else {
            frame.render_widget(dialog.clone().block(dialog_block), content_inner);
        }
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                key_events::handle_key_event(self, key_event);
            }
            _ => {}
        }
        Ok(())
    }
}

fn colour_entry_to_line(
    entry: ListColourEntry,
    selected: i32,
    idx: i32,
    active: &mut Option<ListColourEntry>,
) -> Line<'static> {
    let colours = entry.entry;

    if idx == selected {
        *active = Some(entry.clone());
    }

    Line::raw(format!(
        "[{}] {}",
        match idx == selected {
            true => '*',
            false => ' ',
        },
        entry.text
    ))
    .fg(match colours.fg {
        Some(c) => c.into(),
        None => Color::Reset,
    })
    .bg(match colours.bg {
        Some(c) => c.into(),
        None => Color::Reset,
    })
    .add_modifier(match colours.effects {
        crate::colours::Effects::Default => Modifier::empty(),
        crate::colours::Effects::Bold => Modifier::BOLD,
        crate::colours::Effects::Underline => Modifier::UNDERLINED,
        crate::colours::Effects::Italic => Modifier::ITALIC,
        crate::colours::Effects::Strikethrough => Modifier::CROSSED_OUT,
    })
}
