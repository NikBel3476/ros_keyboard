use crossterm::event::{
    poll, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use rosrust;
use rosrust_msg;
use rosrust_msg::geometry_msgs::{Twist, Vector3};
use std::process::exit;
use std::time::Duration;

fn main() {
    rosrust::init("key_reader");
    enable_raw_mode().unwrap();

    let cmd_vel_pub = rosrust::publish("cmd_vel", 10).unwrap();
    cmd_vel_pub.wait_for_subscribers(None).unwrap();

    let mut vel_msg = rosrust_msg::geometry_msgs::Twist::default();
    let linear_speed = 0.5;
    let angular_speed = 0.3;

    while rosrust::is_ok() {
        if poll(Duration::from_millis(200)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::Key(event) => println!("{:?}", event),
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: KeyEventState::NONE,
                }) => {
                    disable_raw_mode().unwrap();
                    exit(0);
                }
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
                Event::Key(KeyEvent {
                    code: KeyCode::Char('w'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                }) => {
                    vel_msg.linear.x = 0.0;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                })
                | Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    modifiers: KeyModifiers::NONE,
                    kind: KeyEventKind::Release,
                    state: KeyEventState::NONE,
                }) => {
                    vel_msg.angular.z = 0.0;
                }
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                Event::Paste(_) => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        }

        /* match read().unwrap() {
            Event::Key(KeyEvent {

                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                disable_raw_mode().unwrap();
                exit(0);
            }
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
                vel_msg.angular.z = -angular_speed;
            }
            Event::Key(KeyEvent {
                code: KeyCode::Char('d'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                vel_msg.angular.z = angular_speed;
            }
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("Pasted {:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
            Event::Paste(_) => {
                vel_msg = Twist::default();
            }
        } */

        cmd_vel_pub.send(vel_msg.clone()).unwrap();

        // `poll()` waits for an `Event` for a given time period
        /* if poll(Duration::from_millis(500)).unwrap() {
            // It's guaranteed that the `read()` won't block when the `poll()`
            // function returns `true`
            match read().unwrap() {
                Event::FocusGained => println!("FocusGained"),
                Event::FocusLost => println!("FocusLost"),
                Event::Key(event) => println!("{:?}", event),
                Event::Mouse(event) => println!("{:?}", event),
                #[cfg(feature = "bracketed-paste")]
                Event::Paste(data) => println!("Pasted {:?}", data),
                Event::Resize(width, height) => println!("New size {}x{}", width, height),
                Event::Paste(_) => {}
            }
        } else {
            // Timeout expired and no `Event` is available
        } */
    }

    //disabling raw mode
    disable_raw_mode().unwrap();
}

/* fn main() {
    env_logger::init();

    // Initialize node
    rosrust::init("listener");

    // Create subscriber
    // The subscriber is stopped when the returned object is destroyed
    let subscriber_info = rosrust::subscribe("chatter", 2, |v: rosrust_msg::std_msgs::String| {
        // Callback for handling received messages
        rosrust::ros_info!("Received: {}", v.data);
    })
    .unwrap();

    let log_names = rosrust::param("~log_names").unwrap().get().unwrap_or(false);

    if log_names {
        let rate = rosrust::rate(1.0);
        while rosrust::is_ok() {
            rosrust::ros_info!("Publisher uris: {:?}", subscriber_info.publisher_uris());
            rate.sleep();
        }
    } else {
        // Block the thread until a shutdown signal is received
        rosrust::spin();
    }
} */
