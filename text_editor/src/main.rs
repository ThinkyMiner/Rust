use std::{sync::Arc, io};
use std::path::{Path, PathBuf};
use iced::widget::Column;
use iced::{Theme, widget::{button, container, text}, Application, widget::{text_editor, row, horizontal_space}, Settings, Length, Command};
use iced::executor;


fn main() -> iced::Result{
    Editor::run(Settings::default())
}

struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
}

#[derive(Debug, Clone)]
enum Message  {
    Edit(text_editor::Action),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, Error>),
    FileOpened(Result<(PathBuf, Arc<String>), Error>)
}

impl Application for Editor{
    type Message = Message;
    type Theme = Theme; // Will in future use for the customo themes Put Cattputchin
    type Executor = executor::Default; // Custom Executor ??????
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self{

            path: None,
            //content:text_editor::Content::with(include_str!("main.rs")),
            content:text_editor::Content::new(),
            error: None,

        }, Command::perform(load_file(default_file()),Message::FileOpened),)
    }

    fn title(&self) -> String {
        String::from("ThinkyEditor")
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message{
            Message::Edit(action)=>{
                //Applying the actions we intend to apply
                self.error = None;
                self.content.edit(action);
                Command::none()
            }
            Message::Open=>Command::perform(pick_file(), Message:: FileOpened),
            Message::Save=>{
                let text = self.content.text();
                Command::perform(save_file(self.path.clone(), text), Message::FileSaved)
            }
            Message::FileSaved(Ok(path)) => {
                self.path = Some(path);
                Command::none()
            }
            Message::FileSaved(Err(error))=>{
                self.error = Some(error);
                Command::none()
            }
            Message::New=>{
                self.path = None;
                self.content = text_editor::Content::new();
                Command::none()
            }
            Message::FileOpened(Ok((path, content)))=>{
                self.path = Some(path);
                self.content = text_editor::Content::with(&content);
                Command::none()
            }
            Message::FileOpened(Err(error))=>{
                self.error = Some(error);
                Command::none()
            }
        }
        
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let controls = row![
            button("Save").on_press(Message::Save), 
            button("New").on_press(Message::New), 
            button("Open").on_press(Message::Open)];

        let input = text_editor(&self.content).on_edit(Message::Edit);

        let bottom_bar = {
            
            let status = if let Some(Error::IOFailed(error)) = self.error.as_ref(){
                    text(error.to_string())
                }
                else{
                    match self.path.as_deref().and_then(Path::to_str){
                    Some(path)=>text(path).size(14),
                    None =>text("New File"),
                }
            };
            let position = {
                let (line, column) = self.content.cursor_position();
                text(format!("{}:{}", line + 1, column + 1))
            };
            row![status, horizontal_space(Length::Fill), position]
        };

        //container(column![controls, input, bottom_bar].spacing(10/2)).padding(10).into()
        container(Column::new().spacing(10).push(controls).push(input).push(bottom_bar)).padding(10).into()

    }
    fn theme(&self) -> Theme{
        Theme::Dark
    }
}

fn default_file() ->PathBuf{
    PathBuf:: from(format! ("{}/src/main.rs", env!("CARGO_MANIFEST_DIR") ))
}

async fn pick_file() -> Result<(PathBuf, Arc<String>), Error>{
    let handle = rfd::AsyncFileDialog::new().set_title("Choose a file ......").pick_file().await.ok_or(Error::DialogueClosed)?;
    // rfd::AsyncFileDialog::new().set_title("Choose a file ... ").pick_file().await.ok_or(Error::DialogueClosed)
    load_file(handle.path().to_owned()).await
}

async fn load_file(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
    let words = tokio::fs::read_to_string(&path).await.map(Arc::new).map_err(|error| error.kind()).map_err(Error::IOFailed)?;
    Ok((path, words))
}


async fn save_file(path: Option<PathBuf>, text: String) -> Result<PathBuf, Error> {
    let path = if let Some (path) = path {
    path
    }
    else {
        rfd:: AsyncFileDialog:: new()
            .set_title("Choose a file name...")
            .save_file()
            .await
            .ok_or (Error::DialogueClosed)
            .map (| handle| handle.path ().to_owned ())?};
        tokio:: fs:: write(&path, &text)
            .await
            .map_err (|error | Error:: IOFailed (error. kind()))?;
        Ok(path)
}

#[derive(Debug, Clone)]
enum Error{
    DialogueClosed,
    IOFailed(io::ErrorKind),
}