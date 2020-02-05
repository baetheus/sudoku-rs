#![recursion_limit = "512"]

use sudoku::Sudoku;
use yew::services::ConsoleService;
use yew::{html, App, Component, ComponentLink, Html, ShouldRender};

const SUDOKU_LINE: &str =
    "...2...633....54.1..1..398........9....538....3........263..5..5.37....847...1...";

#[derive(Debug)]
struct Model {
    link: ComponentLink<Self>,
    board: Sudoku,
    selected: Option<usize>,
    console: ConsoleService,
}

enum SudokuMessage {
    New,
    Solve,
    Guess(u8),
    Select(usize),
}

impl Model {
    fn new_game(&mut self) {
        self.board = Sudoku::from_str_line(SUDOKU_LINE).unwrap()
    }

    fn guess(&mut self, guess: u8) {
        let guess = match guess {
            v if v < 10 => v,
            _ => 0,
        };

        let mut board = self.board.to_bytes();

        if let Some(l) = self.selected {
            board[l] = guess;
        }

        if let Ok(board) = Sudoku::from_bytes(board) {
            self.board = board;
        }
    }

    fn select(&mut self, location: usize) {
        if location < 81 {
            self.selected = Some(location);
        } else {
            self.selected = None;
        }
    }

    fn solve(&mut self) {
        if let Some(board) = self.board.solve_one() {
            self.board = board;
        }
    }

    fn view_square(&self, location: usize, guess: u8) -> Html {
        let guess = match guess {
            0 => String::from("_"),
            v => v.to_string(),
        };

        let selected = match self.selected {
            Some(v) => v == location,
            None => false,
        };

        let class = if selected { "ct-f0" } else { "" };

        html! {
            <>
                <td class=class>
                    <a onclick=self.link.callback(move |_| SudokuMessage::Select(location))>{ guess }</a>
                </td>
                { horz_spc(location) }
            </>
        }
    }

    fn view_row(&self, offset: usize, chunk: &[u8]) -> Html {
        html! {
            <tr>
                {for chunk.iter().enumerate().map(|(i, g)| {
                    self.view_square(offset + i, *g)
                }) }
            </tr>
        }
    }

    fn view_board(&self) -> Html {
        let bytes = self.board.to_bytes();
        let chunks = bytes.chunks(9);

        html! {
            <table>
                { for chunks
                    .enumerate()
                    .map(|(i, c)| {
                        html! {
                            <>
                                { self.view_row(i * 9, c) }
                                { vert_spc(i) }
                            </>
                        }
                    })
                }
            </table>
        }
    }

    fn view_buttons(&self) -> Html {
        html! {
            <table>
                <tr>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(1))>{"1"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(2))>{"2"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(3))>{"3"}</button></td>
                </tr>
                <tr>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(4))>{"4"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(5))>{"5"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(6))>{"6"}</button></td>
                </tr>
                <tr>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(7))>{"7"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(8))>{"8"}</button></td>
                    <td><button onclick=self.link.callback(|_| SudokuMessage::Guess(9))>{"9"}</button></td>
                </tr>
            </table>
        }
    }

    fn view_status(&self) -> Html {
        if self.board.is_solved() {
            html! { <span>{"Congratulations! You've solved it!"}</span> }
        } else {
            html! {}
        }
    }
}

impl Component for Model {
    type Message = SudokuMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            board: Sudoku::from_str_line(SUDOKU_LINE).unwrap(),
            selected: None,
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SudokuMessage::New => self.new_game(),
            SudokuMessage::Solve => self.solve(),
            SudokuMessage::Guess(guess) => self.guess(guess),
            SudokuMessage::Select(location) => self.select(location),
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <main>
                <h1>{ "Sudoku!" }</h1>
                <article>
                    <button onclick=self.link.callback(|_| SudokuMessage::New)>{ "New Game" }</button>
                    <button onclick=self.link.callback(|_| SudokuMessage::Solve)>{ "Solve" }</button>
                    { self.view_board() }
                    { self.view_buttons() }
                    { self.view_status() }
                </article>
            </main>
        }
    }
}

fn vert_spc(row: usize) -> Html {
    match row {
        2 | 5 => {
            html! { <tr>{ for (0..11).map(|_| html!{<td>{"="}</td>})}</tr> }
        }
        _ => html! {},
    }
}

fn horz_spc(loc: usize) -> Html {
    match loc {
        r if (r + 1) % 3 == 0 && (r + 1) % 9 != 0 => html! {<td>{"="}</td>},
        _ => html! {},
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
