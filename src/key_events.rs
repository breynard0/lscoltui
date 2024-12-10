use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    colours::{ColourEntry, Effects, LsColours, TerminalColour},
    ui::{DialogState, ListColourEntry},
};

use super::App;

pub fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::F(x) => match x {
            1 => {
                app.dialog_state = if app.open_scheme.is_some() {
                    DialogState::PickScheme(
                        app.savefile
                            .schemes
                            .iter()
                            .position(|x| x.0 == app.open_scheme.clone().unwrap())
                            .unwrap() as u8,
                    )
                } else {
                    DialogState::PickScheme(0)
                }
            }
            2 => app.dialog_state = DialogState::NewScheme(String::new()),
            3 => app.dialog_state = DialogState::NewExtension(String::new()),
            4 => {
                if app.open_scheme.is_some() {
                    app.dialog_state = DialogState::Export
                }
            }
            5 => {
                if app.open_scheme.is_some() {
                    app.savefile.most_recent = app.open_scheme.clone().unwrap();
                }
                app.savefile.save().unwrap();
                app.exit = true
            }
            _ => {}
        },
        KeyCode::Up => {
            if let DialogState::PickScheme(x) = app.dialog_state {
                if x > 0 {
                    app.dialog_state = DialogState::PickScheme(x - 1)
                }
            }

            if let DialogState::EditingColour(i) = app.dialog_state {
                if i > 0 {
                    app.dialog_state = DialogState::EditingColour(i - 1)
                } else {
                    app.dialog_state = DialogState::EditingColour(2)
                }
            }

            if let DialogState::Closed = app.dialog_state {
                app.content_loc -= 1;
            }
        }
        KeyCode::Down => {
            if let DialogState::PickScheme(x) = app.dialog_state {
                if x < app.savefile.schemes.len() as u8 - 1 {
                    app.dialog_state = DialogState::PickScheme(x + 1)
                }
            }

            if let DialogState::EditingColour(i) = app.dialog_state {
                if i < 2 {
                    app.dialog_state = DialogState::EditingColour(i + 1)
                } else {
                    app.dialog_state = DialogState::EditingColour(0)
                }
            }

            if let DialogState::Closed = app.dialog_state {
                app.content_loc += 1;
            }
        }
        KeyCode::Esc => {
            if let DialogState::Closed = app.dialog_state {
            } else {
                app.dialog_state = DialogState::Closed;
            }
        }
        KeyCode::Enter => {
            if let DialogState::Closed = app.dialog_state {
                if app.open_scheme.is_some() {
                    app.dialog_state = DialogState::EditingColour(0)
                }
            }

            if let DialogState::PickScheme(x) = app.dialog_state {
                if x < app.savefile.schemes.len() as u8 {
                    if app.savefile.schemes.len() != 0 {
                        app.open_scheme = Some(app.savefile.schemes[x as usize].0.clone());
                    }
                    app.dialog_state = DialogState::Closed;
                    app.content_loc = 0;
                }
            }

            if let DialogState::NewScheme(s) = &app.dialog_state {
                if !s.is_empty() && app.savefile.schemes.iter().find(|x| &x.0 == s).is_none() {
                    app.open_scheme = Some(s.clone());
                    app.savefile.schemes.push((s.clone(), LsColours::default()));
                    app.savefile.save().unwrap();
                    app.dialog_state = DialogState::Closed;
                    app.content_loc = 0;
                }
            }

            if let DialogState::NewExtension(s) = &app.dialog_state {
                if !s.is_empty() && app.savefile.schemes.iter().find(|x| &x.0 == s).is_none() {
                    if app.open_scheme.as_ref().is_some() {
                        app.savefile
                            .schemes
                            .iter_mut()
                            .find(|x| &x.0 == app.open_scheme.as_ref().unwrap())
                            .unwrap()
                            .1
                            .extensions
                            .push((
                                s.clone(),
                                ColourEntry::new(crate::colours::Effects::Default, None, None),
                            ));

                        app.dialog_state = DialogState::Closed;
                    }
                }
            }
        }
        KeyCode::Char(c) => {
            if let DialogState::NewScheme(s) = &app.dialog_state {
                app.dialog_state = DialogState::NewScheme(format!("{}{}", s, c))
            }

            if let DialogState::NewExtension(s) = &app.dialog_state {
                app.dialog_state = DialogState::NewExtension(format!("{}{}", s, c))
            }
        }
        KeyCode::Backspace => {
            if let DialogState::NewScheme(s) = &app.dialog_state {
                let mut s = s.clone();
                s.pop();
                app.dialog_state = DialogState::NewScheme(format!("{}", s))
            }

            if let DialogState::NewExtension(s) = &app.dialog_state {
                let mut s = s.clone();
                s.pop();
                app.dialog_state = DialogState::NewExtension(format!("{}", s))
            }
        }
        KeyCode::Delete => {
            if let DialogState::PickScheme(x) = app.dialog_state {
                if app.savefile.schemes.get(x as usize).is_some() {
                    app.savefile.schemes.remove(x as usize);
                    app.open_scheme = None;
                }
            }

            if let DialogState::Closed = app.dialog_state {
                if app.open_scheme.is_some() {
                    let active_scheme = &app
                        .savefile
                        .schemes
                        .iter()
                        .find(|x| &x.0 == app.open_scheme.as_ref().unwrap())
                        .unwrap()
                        .1;

                    if app.content_loc >= 18
                        && app.content_loc < active_scheme.clone().extensions.len() as i32 + 18
                    {
                        app.savefile
                            .schemes
                            .iter_mut()
                            .find(|x| &x.0 == app.open_scheme.as_ref().unwrap())
                            .unwrap()
                            .1
                            .extensions
                            .remove(app.content_loc as usize - 18);
                    }
                }
            }
        }
        KeyCode::Left => {
            if let DialogState::EditingColour(i) = app.dialog_state {
                if app.active_lce.clone().is_some() {
                    let mut lce = app.active_lce.clone().unwrap();
                    match i {
                        0 => {
                            lce.entry.fg = match &lce.entry.fg {
                                Some(c) => {
                                    if c == TerminalColour::ORDER.first().unwrap() {
                                        None
                                    } else {
                                        let x = TerminalColour::ORDER
                                            .iter()
                                            .position(|y| y == c)
                                            .unwrap();
                                        Some(TerminalColour::ORDER[x - 1])
                                    }
                                }
                                None => Some(*TerminalColour::ORDER.last().unwrap()),
                            };
                        }
                        1 => {
                            lce.entry.bg = match &lce.entry.bg {
                                Some(c) => {
                                    if c == TerminalColour::ORDER.first().unwrap() {
                                        None
                                    } else {
                                        let x = TerminalColour::ORDER
                                            .iter()
                                            .position(|y| y == c)
                                            .unwrap();
                                        Some(TerminalColour::ORDER[x - 1])
                                    }
                                }
                                None => Some(*TerminalColour::ORDER.last().unwrap()),
                            };
                        }
                        2 => {
                            lce.entry.effects =
                                if lce.entry.effects == *Effects::ORDER.first().unwrap() {
                                    *Effects::ORDER.last().unwrap()
                                } else {
                                    let x = Effects::ORDER
                                        .iter()
                                        .position(|y| *y == lce.entry.effects)
                                        .unwrap();

                                    Effects::ORDER[x - 1]
                                }
                        }
                        _ => {}
                    }
                    let active_scheme = &mut app
                        .savefile
                        .schemes
                        .iter_mut()
                        .find(|x| &x.0 == app.open_scheme.as_ref().unwrap())
                        .unwrap()
                        .1;
                    update_lce(&lce, active_scheme);
                    app.active_lce = Some(lce);
                }
            }
        }
        KeyCode::Right => {
            if let DialogState::EditingColour(i) = app.dialog_state {
                if app.active_lce.clone().is_some() {
                    let mut lce = app.active_lce.clone().unwrap();
                    match i {
                        0 => {
                            lce.entry.fg = match &lce.entry.fg {
                                Some(c) => {
                                    if c == TerminalColour::ORDER.last().unwrap() {
                                        None
                                    } else {
                                        let x = TerminalColour::ORDER
                                            .iter()
                                            .position(|y| y == c)
                                            .unwrap();
                                        Some(TerminalColour::ORDER[x + 1])
                                    }
                                }
                                None => Some(*TerminalColour::ORDER.first().unwrap()),
                            };
                        }
                        1 => {
                            lce.entry.bg = match &lce.entry.bg {
                                Some(c) => {
                                    if c == TerminalColour::ORDER.last().unwrap() {
                                        None
                                    } else {
                                        let x = TerminalColour::ORDER
                                            .iter()
                                            .position(|y| y == c)
                                            .unwrap();
                                        Some(TerminalColour::ORDER[x + 1])
                                    }
                                }
                                None => Some(*TerminalColour::ORDER.first().unwrap()),
                            };
                        }
                        2 => {
                            lce.entry.effects =
                                if lce.entry.effects == *Effects::ORDER.last().unwrap() {
                                    *Effects::ORDER.first().unwrap()
                                } else {
                                    let x = Effects::ORDER
                                        .iter()
                                        .position(|y| *y == lce.entry.effects)
                                        .unwrap();

                                    Effects::ORDER[x + 1]
                                }
                        }
                        _ => {}
                    }
                    let active_scheme = &mut app
                        .savefile
                        .schemes
                        .iter_mut()
                        .find(|x| &x.0 == app.open_scheme.as_ref().unwrap())
                        .unwrap()
                        .1;
                    update_lce(&lce, active_scheme);
                    app.active_lce = Some(lce);
                }
            }
        }
        _ => {}
    }
}

fn update_lce(lce: &ListColourEntry, scheme: &mut LsColours) {
    match lce.text.as_str() {
        "Normal" => scheme.norm = lce.entry,
        "File" => scheme.file = lce.entry,
        "Directory" => scheme.dir = lce.entry,
        "Symlink" => scheme.link = lce.entry,
        "Pipe" => scheme.pipe = lce.entry,
        "Door" => scheme.door = lce.entry,
        "Block" => scheme.block = lce.entry,
        "Character" => scheme.char = lce.entry,
        "Orphaned Symlink" => scheme.orphan = lce.entry,
        "Socket" => scheme.sock = lce.entry,
        "SetUID" => scheme.setuid = lce.entry,
        "SetGID" => scheme.setgid = lce.entry,
        "Sticky Other Writable" => scheme.sticky_other_writable = lce.entry,
        "Other Writable" => scheme.other_writable = lce.entry,
        "Sticky" => scheme.sticky = lce.entry,
        "Executable" => scheme.exec = lce.entry,
        "Missing" => scheme.missing = lce.entry,
        "End Code" => scheme.rs = lce.entry,
        _ => match scheme.extensions.iter_mut().find(|x| x.0 == lce.text) {
            Some(x) => *x = (x.0.clone(), lce.entry),
            None => {}
        },
    }
}
