#![recursion_limit = "512"]

mod services;
use crate::services::task_client::TaskClient;
use anyhow::Error;
use serde_derive::{Deserialize, Serialize};
use std::collections::BTreeMap;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, ToString};
use todo_models::{Entry, TaskRequest, TaskResponse};
use wasm_bindgen::prelude::*;
use yew::events::KeyboardEvent;
use yew::services::fetch::FetchTask;
use yew::services::ConsoleService;
use yew::web_sys::HtmlInputElement as InputElement;
use yew::{html, Component, ComponentLink, Href, Html, InputData, NodeRef, ShouldRender};

pub struct Model {
    link: ComponentLink<Self>,
    state: State,
    focus_ref: NodeRef,
    update_tasks: BTreeMap<u64, FetchTask>,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Entry>,
    filter: Filter,
    value: String,
    edit_value: String,
    request_counter: u64,
}

pub enum Msg {
    Add,
    Edit(usize),
    Update(String),
    UpdateEdit(String),
    Remove(usize),
    SetFilter(Filter),
    ToggleAll,
    ToggleEdit(usize),
    Toggle(usize),
    ClearCompleted,
    Focus,
    Nope,
    GetTasks,
    UpdateTasks,
    TasksReceived(u64, Result<TaskResponse, Error>),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let entries = Vec::new();

        let state = State {
            entries,
            filter: Filter::All,
            value: "".into(),
            edit_value: "".into(),
            request_counter: 0,
        };
        let focus_ref = NodeRef::default();

        // Load task data on load
        link.send_message(Msg::GetTasks);

        Model {
            link,
            state,
            focus_ref,
            update_tasks: BTreeMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                let description = self.state.value.trim();
                if !description.is_empty() {
                    let entry = Entry {
                        _id: "".into(),
                        description: description.to_string(),
                        completed: false,
                        editing: false,
                    };
                    self.state.entries.push(entry);
                }
                self.state.value = "".to_string();

                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::Edit(idx) => {
                let edit_value = self.state.edit_value.trim().to_string();
                self.state.complete_edit(idx, edit_value);
                self.state.edit_value = "".to_string();

                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::Update(val) => {
                println!("Input: {}", val);
                self.state.value = val;
            }
            Msg::UpdateEdit(val) => {
                println!("Input: {}", val);
                self.state.edit_value = val;
            }
            Msg::Remove(idx) => {
                self.state.remove(idx);
                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::SetFilter(filter) => {
                self.state.filter = filter;
            }
            Msg::ToggleEdit(idx) => {
                self.state.edit_value = self.state.entries[idx].description.clone();
                self.state.clear_all_edit();
                self.state.toggle_edit(idx);
            }
            Msg::ToggleAll => {
                let status = !self.state.is_all_completed();
                self.state.toggle_all(status);
                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::Toggle(idx) => {
                self.state.toggle(idx);
                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::ClearCompleted => {
                self.state.clear_completed();
                self.link.send_message(Msg::UpdateTasks);
            }
            Msg::Focus => {
                if let Some(input) = self.focus_ref.cast::<InputElement>() {
                    input.focus().unwrap();
                }
            }
            Msg::Nope => {}
            Msg::GetTasks => {
                let request_id = self.state.request_counter;
                self.state.request_counter += 1;
                self.update_tasks
                    .insert(request_id, TaskClient::get_tasks(&self.link, request_id));
            }
            Msg::TasksReceived(request_id, data) => {
                self.update_tasks.remove(&request_id);
                match data {
                    Ok(response) => self.state.entries = response.tasks,
                    Err(e) => ConsoleService::error(&e.to_string()),
                };
            }
            Msg::UpdateTasks => {
                let request_id = self.state.request_counter;
                self.state.request_counter += 1;
                self.update_tasks.insert(
                    request_id,
                    TaskClient::update_tasks(
                        &self.link,
                        request_id,
                        &TaskRequest {
                            tasks: self.state.entries.clone(),
                        },
                    ),
                );
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hidden_class = if self.state.entries.is_empty() {
            "hidden"
        } else {
            ""
        };
        html! {
            <div class="todomvc-wrapper">
                <section class="todoapp">
                    <header class="header">
                        <h1>{ "todos" }
                        </h1>
                        { self.view_input() }
                    </header>
                    <section class=("main", hidden_class)>
                        <input
                            type="checkbox"
                            class="toggle-all"
                            id="toggle-all"
                            checked=self.state.is_all_completed()
                            onclick=self.link.callback(|_| Msg::ToggleAll) />
                        <label for="toggle-all" />
                        <ul class="todo-list">
                            { for self.state.entries.iter().filter(|e| self.state.filter.fit(e)).enumerate().map(|e| self.view_entry(e)) }
                        </ul>
                    </section>
                    <footer class=("footer", hidden_class)>
                        <span class="todo-count">
                            <strong>{ self.state.total() }</strong>
                            { " item(s) left" }
                        </span>
                        <ul class="filters">
                            { for Filter::iter().map(|flt| self.view_filter(flt)) }
                        </ul>
                        <button class="clear-completed" onclick=self.link.callback(|_| Msg::ClearCompleted)>
                            { format!("Clear completed ({})", self.state.total_completed()) }
                        </button>
                    </footer>
                </section>
                <footer class="info">
                    <p>{ "Double-click to edit a todo" }</p>
                    <button onclick=self.link.callback(|_| Msg::UpdateTasks)>
                        {format!("Save Tasks")}
                    </button>
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_filter(&self, filter: Filter) -> Html {
        let flt = filter.clone();
        html! {
            <li>
                <a class=if self.state.filter == flt { "selected" } else { "not-selected" }
                   href=&flt
                   onclick=self.link.callback(move |_| Msg::SetFilter(flt.clone()))>
                    { filter }
                </a>
            </li>
        }
    }

    fn view_input(&self) -> Html {
        html! {
            <input class="new-todo"
                   placeholder="What needs to be done?"
                   value=&self.state.value
                   oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                   onkeypress=self.link.callback(|e: KeyboardEvent| {
                       if e.key() == "Enter" { Msg::Add } else { Msg::Nope }
                   }) />
        }
    }

    fn view_entry(&self, (idx, entry): (usize, &Entry)) -> Html {
        let mut class = "todo".to_string();
        if entry.editing {
            class.push_str(" editing");
        }
        if entry.completed {
            class.push_str(" completed");
        }
        html! {
            <li class=class>
                <div class="view">
                    <input
                        type="checkbox"
                        class="toggle"
                        checked=entry.completed
                        onclick=self.link.callback(move |_| Msg::Toggle(idx)) />
                    <label ondblclick=self.link.callback(move |_| Msg::ToggleEdit(idx))>{ &entry.description }</label>
                    <button class="destroy" onclick=self.link.callback(move |_| Msg::Remove(idx)) />
                </div>
                { self.view_entry_edit_input((idx, &entry)) }
            </li>
        }
    }

    fn view_entry_edit_input(&self, (idx, entry): (usize, &Entry)) -> Html {
        if entry.editing {
            html! {
                <input class="edit"
                       type="text"
                       ref=self.focus_ref.clone()
                       value=&self.state.edit_value
                       onmouseover=self.link.callback(|_| Msg::Focus)
                       oninput=self.link.callback(|e: InputData| Msg::UpdateEdit(e.value))
                       onblur=self.link.callback(move |_| Msg::Edit(idx))
                       onkeypress=self.link.callback(move |e: KeyboardEvent| {
                          if e.key() == "Enter" { Msg::Edit(idx) } else { Msg::Nope }
                       }) />
            }
        } else {
            html! { <input type="hidden" /> }
        }
    }
}

#[derive(EnumIter, ToString, Clone, PartialEq, Serialize, Deserialize)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl<'a> Into<Href> for &'a Filter {
    fn into(self) -> Href {
        match *self {
            Filter::All => "#/".into(),
            Filter::Active => "#/active".into(),
            Filter::Completed => "#/completed".into(),
        }
    }
}

impl Filter {
    fn fit(&self, entry: &Entry) -> bool {
        match *self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed,
        }
    }
}

impl State {
    fn total(&self) -> usize {
        self.entries.len()
    }

    fn total_completed(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| Filter::Completed.fit(e))
            .count()
    }

    fn is_all_completed(&self) -> bool {
        let mut filtered_iter = self
            .entries
            .iter()
            .filter(|e| self.filter.fit(e))
            .peekable();

        if filtered_iter.peek().is_none() {
            return false;
        }

        filtered_iter.all(|e| e.completed)
    }

    fn toggle_all(&mut self, value: bool) {
        for entry in self.entries.iter_mut() {
            if self.filter.fit(entry) {
                entry.completed = value;
            }
        }
    }

    fn clear_completed(&mut self) {
        let entries = self
            .entries
            .drain(..)
            .filter(|e| Filter::Active.fit(e))
            .collect();
        self.entries = entries;
    }

    fn toggle(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.completed = !entry.completed;
    }

    fn toggle_edit(&mut self, idx: usize) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        let entry = entries.get_mut(idx).unwrap();
        entry.editing = !entry.editing;
    }

    fn clear_all_edit(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.editing = false;
        }
    }

    fn complete_edit(&mut self, idx: usize, val: String) {
        let filter = self.filter.clone();
        let mut entries = self
            .entries
            .iter_mut()
            .filter(|e| filter.fit(e))
            .collect::<Vec<_>>();
        if !val.is_empty() {
            let entry = entries.get_mut(idx).unwrap();
            entry.description = val;
            entry.editing = !entry.editing;
        } else {
            self.remove(idx);
        }
    }

    fn remove(&mut self, idx: usize) {
        let idx = {
            let filter = self.filter.clone();
            let entries = self
                .entries
                .iter()
                .enumerate()
                .filter(|&(_, e)| filter.fit(e))
                .collect::<Vec<_>>();
            let &(idx, _) = entries.get(idx).unwrap();
            idx
        };
        self.entries.remove(idx);
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}
