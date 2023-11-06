use crossterm::{
    event::{Event, EventStream, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use futures::{future::FutureExt, select, StreamExt};
use futures_timer::Delay;
use rosrust;
use rosrust_msg;
use rosrust_msg::geometry_msgs::Twist;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    rosrust::init("key_reader");
    enable_raw_mode()?;

    // let mut stdout = stdout();
    // execute!(stdout, EnableMouseCapture)?;

    async_std::task::block_on(print_events());

    // execute!(stdout, DisableMouseCapture)?;

    disable_raw_mode()
}

async fn print_events() {
    let cmd_vel_pub = rosrust::publish("cmd_vel", 10).unwrap();
    cmd_vel_pub.wait_for_subscribers(None).unwrap();

    let linear_speed = 0.5;
    let angular_speed = 0.5;

    let mut reader = EventStream::new();

    while rosrust::is_ok() {
        let mut delay = Delay::new(Duration::from_millis(100)).fuse();
        let mut event = reader.next().fuse();

        let mut vel_msg = Twist::default();

        select! {
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        match event {
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('c'),
                                modifiers: KeyModifiers::CONTROL,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => break,
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('w'),
                                modifiers: KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                vel_msg.linear.x = linear_speed;
                            }
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('s'),
                                modifiers: KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                vel_msg.linear.x = -linear_speed;
                            }
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('a'),
                                modifiers: KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                vel_msg.angular.z = angular_speed;
                            }
                            Event::Key(KeyEvent {
                                code: KeyCode::Char('d'),
                                modifiers: KeyModifiers::NONE,
                                kind: KeyEventKind::Press,
                                state: KeyEventState::NONE,
                            }) => {
                                vel_msg.angular.z = -angular_speed;
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => println!("Error: {:?}\r", e),
                    None => break,
                }
            }
            _ = delay => {},
        };

        cmd_vel_pub.send(vel_msg.clone()).unwrap();
    }
}
