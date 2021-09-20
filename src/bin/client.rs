use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{event, queue};
use std::io;
use std::io::Write;
use std::net::TcpStream;
use ultimate_ultimate_ttt::{Board, Event, Outcome, Player, Tile};

fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("0.0.0.0:9292")?;
    let mut connection = jsonl::Connection::new_from_tcp_stream(stream)?;
    let mut std_out = io::stdout();
    println!("Waiting to be assigned X or O");
    enable_raw_mode()?;
    queue!(
        std_out,
        Hide,
        EnableMouseCapture,
        Clear(ClearType::All),
        MoveTo(0, 0)
    )?;
    std_out.flush()?;
    let mut board = Board {
        tiles: [Tile::Empty; 9],
    };
    println!("{}\r", &board);

    'outer: loop {
        match connection.read()? {
            Event::Turn => loop {
                match event::read()? {
                    event::Event::Mouse(event::MouseEvent {
                        kind: event::MouseEventKind::Up(event::MouseButton::Left),
                        column,
                        row,
                        ..
                    }) => {
                        let game_column = match column {
                            0..=2 => 0,
                            4..=6 => 1,
                            8..=10 => 2,
                            _ => continue,
                        };
                        let game_row = match row {
                            0 => 0,
                            2 => 1,
                            4 => 2,
                            _ => continue,
                        };

                        let i = game_row * 3 + game_column;

                        if board.tiles[i] == Tile::Empty {
                            connection.write(&Event::Play(i))?;
                            break;
                        }
                    }
                    event::Event::Key(event::KeyEvent {
                        code: event::KeyCode::Char('c'),
                        modifiers: event::KeyModifiers::CONTROL,
                    })
                    | event::Event::Key(event::KeyEvent {
                        code: event::KeyCode::Char('q'),
                        ..
                    }) => break 'outer,
                    _ => continue,
                }
            },
            Event::Played { player: p, pos } => {
                board.tiles[pos] = match p {
                    Player::X => Tile::X,
                    Player::O => Tile::O,
                };
            }
            Event::End(o) => {
                queue!(std_out, Clear(ClearType::All), MoveTo(0, 0))?;
                println!("{}\r", &board);
                println!("{}\r", o);
                break;
            }
            Event::Play(_) => unreachable!(),
        }
        queue!(std_out, Clear(ClearType::All), MoveTo(0, 0))?;
        println!("{}\r", &board);
    }

    queue!(std_out, DisableMouseCapture, Show)?;
    disable_raw_mode()?;
    std_out.flush()?;

    Ok(())
}
