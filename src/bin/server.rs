use flume::{Receiver, Sender};
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use ttt::{Board, Event, Player, Tile};

fn main() -> anyhow::Result<()> {
    let mut board = Board {
        tiles: [Tile::Empty; 9],
    };
    let listener = TcpListener::bind("0.0.0.0:9292")?;
    let x_connection = create_connection(listener.accept()?.0)?;
    let o_connection = create_connection(listener.accept()?.0)?;

    let mut player = Player::X;

    loop {
        let (active, inactive) = match player {
            Player::X => (&x_connection, &o_connection),
            Player::O => (&o_connection, &x_connection),
        };
        active.sender.send(Event::Turn)?;
        if let Event::Play(i) = active.receiver.recv()? {
            if board.tiles[i] == Tile::Empty {
                board.tiles[i] = player.tile();
                active.sender.send(Event::Played { player, pos: i })?;
                inactive.sender.send(Event::Played { player, pos: i })?;
            }
        }
        if let Some(outcome) = board.check_outcome() {
            active.sender.send(Event::End(outcome))?;
            inactive.sender.send(Event::End(outcome))?;
            thread::sleep(Duration::from_millis(1000));
            break;
        } else {
            player = player.next();
        }
    }

    Ok(())
}

fn create_connection(stream: TcpStream) -> anyhow::Result<Connection> {
    let (s, r) = flume::unbounded();
    let receiver = start_worker(r, stream)?;
    let connection = Connection {
        sender: s,
        receiver,
    };
    Ok(connection)
}

fn start_worker(receiver: Receiver<Event>, stream: TcpStream) -> anyhow::Result<Receiver<Event>> {
    let (s, r) = flume::unbounded();
    let stream_clone = stream.try_clone()?;

    thread::spawn(move || -> anyhow::Result<()> {
        for event in receiver {
            jsonl::write(&stream, &event)?;
        }

        Ok(())
    });

    thread::spawn(move || -> anyhow::Result<()> {
        let mut stream = BufReader::new(stream_clone);
        loop {
            let event = jsonl::read(&mut stream)?;
            s.send(event)?;
        }
    });

    Ok(r)
}

struct Connection {
    sender: Sender<Event>,
    receiver: Receiver<Event>,
}
