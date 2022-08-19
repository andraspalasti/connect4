use connect4::bitboard::{Bitboard, Token};
use yew::prelude::*;

enum Msg {
    MakeMove(usize),
}

struct Board {
    bitboard: Bitboard,
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bitboard: Bitboard::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MakeMove(col) => {
                if col < 7 {
                    self.bitboard.make_move(col);
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let has_won = self.bitboard.has_won();
        let available_cols = self.bitboard.list_moves();

        let moves = self.bitboard.moves().iter().fold(String::new(), |s, col| s + &col.to_string());

        html! {
            <>
                <div class="board">
                    {(0..42).into_iter().map(|i| {
                        let (row, col) = (i / 7, i % 7);

                        let onclick = if available_cols.contains(&col) && !has_won {
                            Some(link.callback(move |_| Msg::MakeMove(col)))
                        } else {
                            None
                        };

                        match self.bitboard.get(row, col) {
                            Token::Red => html! { <div {onclick} class="token red"/> },
                            Token::Yellow => html! { <div {onclick} class="token yellow"/> },
                            Token::Empty => html! { <div {onclick} class="token"/> }
                        }
                    }).collect::<Html>()}
                </div>
                <p>{moves}</p>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Board>();
}
