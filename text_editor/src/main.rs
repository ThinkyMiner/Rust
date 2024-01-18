use std::{sync::Arc, io};
use std::path::{Path, PathBuf};
use iced::widget::{Column, tooltip};
use iced::{Font, Theme, widget::{pick_list, button, container, text}, Application, widget::{text_editor, row, horizontal_space}, Settings, Length, Command};
use iced::{executor, Element, theme, Subscription, keyboard};
use iced::highlighter::{self, Highlighter};

fn main() -> iced::Result{
    Editor::run(Settings{
        default_font: Font::MONOSPACE,
        fonts: vec![include_bytes!("../rust-editor.ttf").as_slice().into()],
        ..Settings::default()
    })
}
// hope this make me learn moressdssss
struct Editor {
    path: Option<PathBuf>,
    content: text_editor::Content,
    error: Option<Error>,
    theme:highlighter::Theme,
    is_dirty:bool,
}
//Testing the ctrl + s 
#[derive(Debug, Clone)]
enum Message  {
    Edit(text_editor::Action),
    Open,
    New,
    Save,
    FileSaved(Result<PathBuf, Error>),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    ThemeSelected(highlighter::Theme),
}
// Hello World saving this now
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
            theme: highlighter:: Theme:: SolarizedDark,
            is_dirty:true,

        }, Command::perform(load_file(default_file()),Message::FileOpened),)
    }

    fn title(&self) -> String {
        String::from("ThinkyEditor")
    }

    fn update(&mut self, message: Message) -> Command<Message>{
        match message{
            Message::Edit(action)=>{
                //Applying the actions we intend to apply
                self.is_dirty = self.is_dirty || action.is_edit();
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
                self.is_dirty = false;
                self.path = Some(path);
                Command::none()
            }
            Message::FileSaved(Err(error))=>{
                
                self.error = Some(error);
                Command::none()
            }
            Message::New=>{
                self.path = None;
                self.is_dirty = true;
                self.content = text_editor::Content::new();
                Command::none()
            }
            Message::FileOpened(Ok((path, content)))=>{
                
                self.path = Some(path);
                self.is_dirty = false;
                self.content = text_editor::Content::with(&content);
                Command::none()
            }
            Message::FileOpened(Err(error))=>{
                self.error = Some(error);
                Command::none()
            }
            Message::ThemeSelected(theme) =>{
                self.theme = theme;
                Command::none()
            }
        }
        
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press (|key_code, modifiers| match key_code{
            keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
            _ => None,
        })
    }
    fn view(&self) -> iced::Element<'_, Message> {
        let controls = row![
            action(new_icon(), "New File", Some(Message::New)),
            action(save_icon(), "Save File", self.is_dirty.then_some(Message::Save)), 
            action(open_icon(), "Open File", Some(Message::Open)),
            horizontal_space(Length::Fill),
            pick_list(highlighter::Theme::ALL, Some(self.theme), Message::ThemeSelected),
            ].spacing(5);

        let input = text_editor(&self.content)
            .on_edit(Message::Edit)
            .highlight::<Highlighter>(highlighter::Settings{
                theme: self.theme, 
                extension: self
                .path.as_ref().and_then(|path| path.extension()?.to_str()).unwrap_or("rs").to_string()}, |highlight, _theme| {highlight. to_format()});

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
        if self.theme.is_dark(){
            Theme::Dark
        }
        else{
            Theme::Light
        }
        
    }
}

fn action<'a>(content: Element<'a, Message>, label:&str, on_press: Option<Message> )-> Element<'a, Message>{
    let is_disabled = on_press.is_none();
    tooltip(button (container (content).width (30).center_x()).on_press_maybe(on_press).padding([5, 10]).style(if is_disabled{theme::Button::Secondary}else{theme::Button::Primary}), label, tooltip::Position::FollowCursor,).style(theme::Container::Box).into()
}

fn new_icon<'a, Message> ()-> Element<'a, Message>{
    icon ('\u{E800}')
}

fn open_icon<'a, Message> ()-> Element<'a, Message>{
    icon ('\u{F115}')
}

fn save_icon<'a, Message> ()-> Element<'a, Message>{
    icon ('\u{E801}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message>{
    const ICON_FONT: Font = Font::with_name ("rust-editor");
    text(codepoint).font(ICON_FONT).into()
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
