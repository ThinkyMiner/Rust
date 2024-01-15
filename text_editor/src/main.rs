use iced::{widget::container, Sandbox, widget::text_editor, Settings};

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
        container(input).padding(10).into() // Check if u want padding like a box around where u type
    }
}
