#![recursion_limit = "256"]
#[warn(dead_code)]

use std::str::FromStr;
use yew::prelude::*;


pub struct Model {
    link: ComponentLink<Self>,
    state: State,
}

pub struct State {
    tasks: Vec<Task>,
    new_task_name: String,
    new_task_assignee: String,
    new_task_mandays: u32,
}

pub struct Task {
    name: String,
    assignee: String,
    mandays: u32,
    status: u32,
}

pub enum Msg {
    IncreaseStatus(usize),
    DecreaseStatus(usize),
    UpdateNewTaskName(String),
    UpdateNewTaskAssignee(yew::html::ChangeData),
    UpdateNewTaskMandays(String),
    NewTask,
}

impl State {
    fn add_new_task(&mut self, name: String, assignee: String, mandays: u32) {
        self.tasks.push(Task { name, assignee, mandays, status: 1 });
    }
    fn increase_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status < 3).map(|e| e.status = e.status + 1);
    }
    fn decrease_status(&mut self, idx: usize) {
        self.tasks.get_mut(idx).filter(|e| e.status > 1).map(|e| e.status = e.status - 1);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            state: State {
                tasks: vec![
                    Task { name: "Task 1".to_string(), assignee: "🐱".to_string(), mandays: 3, status: 1 },
                    Task { name: "Task 2".to_string(), assignee: "🐶".to_string(), mandays: 2, status: 1 },
                    Task { name: "Task 3".to_string(), assignee: "🐱".to_string(), mandays: 1, status: 2 },
                    Task { name: "Task 4".to_string(), assignee: "🐹".to_string(), mandays: 3, status: 3 },
                ],
                new_task_name: "".to_string(),
                new_task_assignee: "".to_string(),
                new_task_mandays: 0,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateNewTaskName(val) => {
                self.state.new_task_name = val;
            }
            Msg::UpdateNewTaskAssignee(val) => {
                if let yew::html::ChangeData::Select(v) = &val {
                    self.state.new_task_assignee = v.raw_value();
                }
            }
            Msg::UpdateNewTaskMandays(val) => {
                if let Ok(v) = u32::from_str(&val) {
                    self.state.new_task_mandays = v;
                }
            }
            Msg::NewTask => {
                self.state.add_new_task(self.state.new_task_name.clone(), self.state.new_task_assignee.clone(), self.state.new_task_mandays);
            }
            Msg::IncreaseStatus(idx) => {
                self.state.increase_status(idx);
            }
            Msg::DecreaseStatus(idx) => {
                self.state.decrease_status(idx);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <section class="section", id="board",>
            { self.view_header(&self.state)}
                <div class="container",>
                    <div class="columns",>
                        { self.view_column(1, "未対応", &self.state.tasks) }
                        { self.view_column(2, "処理中", &self.state.tasks) }
                        { self.view_column(3, "完了"  , &self.state.tasks) }
                    </div>
                </div>
             </section>
        }
    }
}

impl Model {
    fn view_column(&self, status: u32, status_text: &str, tasks: &Vec<Task>) -> Html {
        html! {
            <div class=format!("column status-{}", status),>
                <div class="tags has-addons",>
                    <span class="tag",>{ status_text }</span>
                    <span class="tag is-dark",>{ tasks.iter().filter(|e| e.status == status).count() }</span>
                </div>
                { for tasks.iter().enumerate().filter(|e| e.1.status == status).map(|t| self.view_task(t)) }
            </div>
        }
    }

    fn view_task(&self, (idx, task): (usize, &Task)) -> Html {
        html! {
            <div class="card",>
                <div class="card-content",>
                    { &task.name }
                </div>
                <footer class="card-footer",>
                    <div class="card-footer-item",>
                        { &task.assignee }
                    </div>
                    <div class="card-footer-item",>
                        { format!("{} 人日", &task.mandays) }
                    </div>
                </footer>
                <footer class="card-footer",>
                  <a class="card-footer-item", onclick=self.link.callback(move |_| Msg::DecreaseStatus(idx))>{ "◀" }</a>
                  <a class="card-footer-item", onclick=self.link.callback(move |_| Msg::IncreaseStatus(idx))>{ "▶︎︎" }</a>
                </footer>
            </div>
        }
    }

    fn view_header(&self, state: &State) -> Html {
        html! {
            <div class="container">
                <input value=&state.new_task_name, oninput=self.link.callback(|e: InputData| Msg::UpdateNewTaskName(e.value))/>
                <select value=&state.new_task_assignee, onchange=self.link.callback(|e| Msg::UpdateNewTaskAssignee(e))>
                    <option value="🐱",>{ "🐱" }</option>
                    <option value="🐶",>{ "🐶" }</option>
                    <option value="🐹",>{ "🐹" }</option>
                </select>
                <input value=&state.new_task_mandays, oninput=self.link.callback(|e: InputData| Msg::UpdateNewTaskMandays(e.value))/>
                <button onclick=self.link.callback(|_| Msg::NewTask)>{ "追加" }</button>
                <hr/>
            </div>
        }
    }
}