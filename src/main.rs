use iced::widget::{self, column, container, row, text};
use iced::{
    Alignment, Application, Color, Command, Element, Length, Settings, Theme,
};

pub fn main() -> iced::Result {
    Quotes::run(Settings::default())
}

#[derive(Debug)]
enum Quotes {
    Loading,
    Loaded { quote: Quote },
    Errored,
}

#[derive(Debug, Clone)]
enum Message {
    QuoteFound(Result<Quote, Error>),
    Search,
}

impl Application for Quotes {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Quotes, Command<Message>) {
        (
            Quotes::Loading,
            Command::perform(Quote::search(), Message::QuoteFound),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            Quotes::Loading => "Loading",
            Quotes::Loaded { quote, .. } => &quote.author,
            Quotes::Errored { .. } => "Whoops!",
        };

        format!("{subtitle} - Pokédex")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::QuoteFound(Ok(quote)) => {
                *self = Quotes::Loaded { quote };

                Command::none()
            }
            Message::QuoteFound(Err(_error)) => {
                *self = Quotes::Errored;

                Command::none()
            }
            Message::Search => match self {
                Quotes::Loading => Command::none(),
                _ => {
                    *self = Quotes::Loading;

                    Command::perform(Quote::search(), Message::QuoteFound)
                }
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = match self {
            Quotes::Loading => {
                column![text("Searching for Quotes...").size(40),]
                    .width(Length::Shrink)
            }
            Quotes::Loaded { quote } => column![
                quote.view(),
                button("Keep searching!").on_press(Message::Search)
            ]
            .max_width(500)
            .spacing(20)
            .align_items(Alignment::End),
            Quotes::Errored => column![
                text("Whoops! Something went wrong...").size(40),
                button("Try again").on_press(Message::Search)
            ]
            .spacing(20)
            .align_items(Alignment::End),
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
struct Quote {
    content: String,
    author: String,
}

impl Quote {
    fn view(&self) -> Element<Message> {
        row![
            column![
                row![
                    text(&self.content).size(30).width(Length::Fill),
                    text(format!("-{}", self.author))
                        .size(20)
                        .style(Color::from([0.5, 0.5, 0.5])),
                ]
                .align_items(Alignment::Center)
                .spacing(20),
            ]
            .spacing(20),
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
    }

    async fn search() -> Result<Quote, Error> {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        struct Entry {
            content: String,
            author: String,
        }

        let fetch_entry = async {
            let url = format!("https://api.quotable.io/random");

            reqwest::get(&url).await?.json().await
        };
        
        let entry : Entry = fetch_entry.await?;

        Ok(Quote {
            content: entry.content,
            author: entry.author,
        })
    }
}

#[derive(Debug, Clone)]
enum Error {
    APIError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);
        Error::APIError
    }
}

fn button(text: &str) -> widget::Button<'_, Message> {
    widget::button(text).padding(10)
}