use iced::{
    alignment::{Horizontal, Vertical},
    button, scrollable, text_input, Application, Button, Column, Command, Container, Element, Length, Scrollable, Settings, Text, TextInput,
};

fn main() -> iced::Result {
    TodoApp::run(Settings::default())
}

struct TodoApp {
    input_value: String,
    input_state: text_input::State,
    add_button_state: button::State,
    tasks: Vec<String>,
    task_button_states: Vec<button::State>,
    scroll: scrollable::State,
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    AddTask,
    RemoveTask(usize),
}

impl Application for TodoApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (TodoApp, Command<Self::Message>) {
        (
            TodoApp {
                input_value: String::new(),
                input_state: text_input::State::new(),
                add_button_state: button::State::new(),
                tasks: Vec::new(),
                task_button_states: Vec::new(), 
                scroll: scrollable::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Todo App - Rust")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                self.input_value = value;
            }
            Message::AddTask => {
                if !self.input_value.is_empty() {
                    self.tasks.push(self.input_value.clone());
                    self.task_button_states.push(button::State::new()); 
                    self.input_value.clear();
                }
            }
            Message::RemoveTask(index) => {
                self.tasks.remove(index);
                self.task_button_states.remove(index); 
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Self::Message> {
        let mut content = Column::new()
            .spacing(20)
            .align_items(iced::Alignment::Center) 
            .push(
                TextInput::new(&mut self.input_state, "Hello Nicole, what would you like to do today?", &self.input_value, Message::InputChanged)
                    .padding(15)
                    .size(20),
            )
            .push(
                Button::new(&mut self.add_button_state, Text::new("Add").horizontal_alignment(Horizontal::Center)) 
                    .on_press(Message::AddTask)
                    .padding(10),
            );

            for (index, (task, button_state)) in self.tasks.iter().zip(self.task_button_states.iter_mut()).enumerate() {
                content = content.push(
                    Button::new(
                        button_state, // Use unique state
                        Text::new(task.clone()).horizontal_alignment(Horizontal::Center),
                    )
                    .on_press(Message::RemoveTask(index))
                    .padding(10),
                );
            }
        Scrollable::new(&mut self.scroll)
            .padding(40)
            .push(Container::new(content).width(Length::Fill).center_x())
            .into()
    }
}
