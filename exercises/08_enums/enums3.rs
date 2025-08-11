#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

enum Message {
    Resize { width: u64, height: u64 },
    Move(Point),
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

#[derive(Debug)]
struct State {
    width: u64,
    height: u64,
    position: Point,
    message: String,
    // RGB color composed of red, green and blue.
    color: (u8, u8, u8),
    quit: bool,
}

impl State {
    fn resize(&mut self, width: u64, height: u64) {
        self.width = width;
        self.height = height;
    }

    fn move_position(&mut self, point: Point) {
        self.position = point;
    }

    fn echo(&mut self, s: String) {
        self.message = s;
    }

    fn change_color(&mut self, red: u8, green: u8, blue: u8) {
        self.color = (red, green, blue);
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn print_fields(&mut self) {
        // println!("width: {0}, height: {1}", self.width, self.height);
        // println!("position: {:?}", self.position);
        // println!("message: {0}", self.message);
        // println!("color: {0:?}", self.color);
        // println!("quit: {0}", self.quit);

        println!("{self:#?}");
    }

    fn process(&mut self, message: Message) {
        // TODO: Create a match expression to process the different message
        // variants using the methods defined above.
        match message {
            Message::Resize{width, height} => {
                self.resize(width, height);
            },
            Message::Move(point) => {
                self.move_position(point);
            },
            Message::Echo(string) => {
                self.echo(string);
            },
            Message::ChangeColor(red, green, blue) => {
                self.change_color(red, green, blue);
            },
            Message::Quit => {
                self.quit();
            },
        }
    }
}

fn main() {
    // You can optionally experiment here.
    let mut state = State {
        width: 0,
        height: 0,
        position: Point {x: 0, y: 0},
        message: String::from("Hello enums"),
        color: (0, 0, 0),
        quit: false,
    };

    state.print_fields();
    state.process(Message::Echo(state.message.to_uppercase()));
    state.process(Message::Move(Point { x: 0, y: 2 }));
    state.process(Message::Resize { width: 5, height: 0 });
    state.process(Message::ChangeColor(255, 0, 100));
    state.process(Message::Quit);
    state.print_fields();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_message_call() {
        let mut state = State {
            width: 0,
            height: 0,
            position: Point { x: 0, y: 0 },
            message: String::from("hello world"),
            color: (0, 0, 0),
            quit: false,
        };

        state.process(Message::Resize {
            width: 10,
            height: 30,
        });
        state.process(Message::Move(Point { x: 10, y: 15 }));
        state.process(Message::Echo(String::from("Hello world!")));
        state.process(Message::ChangeColor(255, 0, 255));
        state.process(Message::Quit);

        assert_eq!(state.width, 10);
        assert_eq!(state.height, 30);
        assert_eq!(state.position.x, 10);
        assert_eq!(state.position.y, 15);
        assert_eq!(state.message, "Hello world!");
        assert_eq!(state.color, (255, 0, 255));
        assert!(state.quit);
    }
}
