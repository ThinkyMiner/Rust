use std::{sync::Arc, io};
use std::path::Path;
use iced::{Theme, widget::{container, column, text}, Application, widget::{text_editor, row, horizontal_space}, Settings, Length, Command};
use tokio::fs;
use iced::executor;


fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor {
    content: text_editor::Content
}

#[derive(Debug, Clone)]
enum Message  {
    Edit(text_editor::Action),
    FileOpened(Result<Arc<String>, io::ErrorKind>)
}

impl Application for Editor{
    type Message = Message;
    type Theme = Theme; // Will in future use for the customo themes Put Cattputchin
    type Executor = executor::Default; // Custom Executor ??????
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self{
            //content:text_editor::Content::with(include_str!("main.rs")),
            content:text_editor::Content::new(),

        }, Command::perform(load_file(format!("{} /src/main.rs", env!("CARGO_MANIFEST_DIR"))),Message::FileOpened),)
    }

    fn title(&self) -> String {
        String::from("ThinkyEditor")
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message{
            Message::Edit(Action)=>{
                //Applying the actions we intend to apply
                self.content.edit(Action);
            }
            Message::FileOpened(result)=>{
                if let Ok(content) = result{
                    self.content = text_editor::Content::with(&content)
                }
            }
        }
        Command::none()
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
        Theme::Light
    }
}


async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path).await.map(Arc::new).map_err(|error| error.kind())
}
