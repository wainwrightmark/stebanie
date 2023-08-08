pub mod puzzle;

use std::ops::{AddAssign, SubAssign};

use leptos::{ev::SubmitEvent, *};
use leptos_use::storage::use_local_storage;

use crate::puzzle::*;

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let puzzle_index = use_local_storage(cx, "index", 0usize);
    let puzzle_index = (puzzle_index.0, puzzle_index.1);
    let puzzle = create_memo(cx, move |_| {
        PUZZLES.get(puzzle_index.0.get() % PUZZLES.len()).unwrap().clone()
    });

    view! { cx,
        <main class="container">
        <Puzzle  puzzle puzzle_index=puzzle_index />
        </main>



    }
}

#[component]
fn Puzzle(cx: Scope, puzzle: Memo<Puzzle>, puzzle_index: (Signal<usize>, WriteSignal<usize>)) -> impl IntoView {
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();


        puzzle_index.1.update(|x| x.add_assign(1));
    };



    view! {cx,

            <h3>{move || puzzle.get().title} </h3>

            <div class="google-map">
                <iframe
                 src={move || puzzle.get().map}
                width="400"
                height="300"
                style="border:0;"
                allowfullscreen=""
                loading="lazy"
                referrerpolicy="no-referrer-when-downgrade"></iframe>
                // <iframe src="[your unique google URL] " width="600" height="450" style="border:0;" allowfullscreen="" loading="lazy" referrerpolicy="no-referrer-when-downgrade"></iframe>
            </div>

            <div inner_html={move || puzzle.get().html} />
            <br/>
            <hr/>
            <br/>

            <form on:submit=on_submit>
            <div class="grid">

                <For
                    each={move || puzzle.get().questions}
                    key={|question| question.q.clone()}
                    view={
                        move |cx, question|{
                            view!{ cx, <Question id={question.q.clone()} question=question.clone() />}
                        }
                    }
                />


                <input type="submit" value="Submit"/>

                <details>
                <summary>Cheat</summary>
                <button disabled={move || puzzle_index.0.get() == 0} on:click=move |_| puzzle_index.1.update(|x|x.sub_assign(1))>"Back"</button>
                <button  on:click=move |_| puzzle_index.1.update(|x|x.add_assign(1))>"Skip"</button>
                </details>
                </div>

            </form>

        }
}

#[component]
fn Question(cx: Scope, id: String, question: Question) -> impl IntoView {
    view! { cx,

        <label for={id.clone()}>{question.q.clone()}</label>
        <input
        type="text"

        id={id}
        placeholder={question.placeholder()}
        pattern={question.pattern()}
        minlength={ question.min_length()}
        maxlength={question.max_length()}
        required
        />

    }
}
