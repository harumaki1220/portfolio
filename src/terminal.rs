use leptos::{html, prelude::*};

use crate::commands::{self, Line, Output};

const PROMPT: &str = "guest@harumaki1220:~$";

#[derive(Clone)]
struct Entry {
    id: usize,
    input: String,
    output: Vec<Line>,
}

#[component]
pub fn Terminal() -> impl IntoView {
    let history = RwSignal::new(Vec::<Entry>::new());
    let input = RwSignal::new(String::new());
    let next_id = StoredValue::new(0);
    let input_ref = NodeRef::<html::Input>::new();

    let execute = move |command: String| {
        let output = match commands::run(&command) {
            Output::Lines(lines) => lines,
            Output::Clear => {
                history.write().clear();
                return;
            }
        };
        let id = next_id.get_value();
        next_id.set_value(id + 1);
        history.write().push(Entry { id, input: command, output });
    };

    let focus_input = move || {
        if let Some(el) = input_ref.get() {
            let _ = el.focus();
        }
    };

    // 出力が増えたら入力欄が見える位置までスクロールする
    Effect::new(move || {
        history.track();
        if let Some(el) = input_ref.get() {
            el.scroll_into_view_with_bool(false);
        }
    });

    // 何を打てばいいか分かるように、最初に help を実行しておく
    execute("help".to_string());

    view! {
        <div class="terminal" on:click=move |_| focus_input()>
            <For
                each=move || history.get()
                key=|entry| entry.id
                children=move |entry| {
                    view! {
                        <div class="entry">
                            <p class="line">
                                <span class="prompt">{PROMPT}</span>
                                <span>{entry.input}</span>
                            </p>
                            {entry
                                .output
                                .into_iter()
                                .map(|line| render_line(line, execute))
                                .collect_view()}
                        </div>
                    }
                }
            />
            <form
                class="line"
                on:submit=move |ev| {
                    ev.prevent_default();
                    let command = input.get();
                    input.set(String::new());
                    execute(command);
                }
            >
                <label class="prompt" for="command">
                    {PROMPT}
                </label>
                <input
                    id="command"
                    class="input"
                    node_ref=input_ref
                    bind:value=input
                    autocomplete="off"
                    autocapitalize="off"
                    spellcheck="false"
                    autofocus
                />
            </form>
        </div>
    }
}

fn render_line(line: Line, execute: impl Fn(String) + Copy + 'static) -> AnyView {
    match line {
        Line::Text(text) => view! { <p class="line">{text}</p> }.into_any(),
        Line::Command { name, description } => {
            view! {
                <p class="line">
                    <button
                        class="command"
                        type="button"
                        on:click=move |_| execute(name.to_string())
                    >
                        {name}
                    </button>
                    <span class="description">{description}</span>
                </p>
            }
                .into_any()
        }
    }
}
