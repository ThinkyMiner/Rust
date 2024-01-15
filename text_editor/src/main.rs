use iced::{Theme, widget::{container, column, text}, Sandbox, widget::{text_editor, row, horizontal_space}, Settings, Length};

fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content
}

#[derive(Debug, Clone)]
enum Message  {
    Edit(text_editor::Action)
}

impl Sandbox for Editor{
    type Message = Message;

    fn new() -> Self {
        Self{
            //content:text_editor::Content::with(include_str!("main.rs")),
            content:text_editor::Content::new(),

        }
    }

    fn title(&self) -> String {
        String::from("ThinkyEditor")
    }

    fn update(&mut self, message: Message) {
        match message{
            Message::Edit(Action)=>{
                //Applying the actions we intend to apply
                self.content.edit(Action);
            }
        };
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let input = text_editor(&self.content).on_edit(Message::Edit);
        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line + 1, column + 1))
        };

        let bottom_bar = row![horizontal_space(Length::Fill), position];

        container(column![input, bottom_bar].spacing(10/2)).padding(10).into() // Check if u want padding like a box around where u type
    }
    fn theme(&self) -> Theme{
        Theme::Dark
    }
}
