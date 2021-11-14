use anyhow;
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::fetch::{FetchService, FetchTask, Request, Response},
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
struct SlackUser {
    real_name: String,
    profile_url: String,
    phone: String,
    title: String
}

enum Msg {
    GetUserNames,
    SetUserNames(Vec<SlackUser>)
}

struct Faces {
    link: ComponentLink<Self>,
    users: Vec<SlackUser>,
    fetch_task: Option<FetchTask>
}

impl Faces {
    fn view_user(&self, user: &SlackUser) -> Html {
        html! {
            <div class=classes!("bg-white", "rounded-md")>
                <img class=classes!("object-cover", "rounded-t-md", "w-full", "h-40", "xl:h-48") src={user.profile_url.clone()} />
                <div class=classes!("text-sm", "font-sans", "p-4")>
                    <div class=classes!("font-bold", "text-gray-800")>{ user.real_name.clone() }</div>
                    <div class=classes!("text-gray-400", "overflow-hidden")>{ user.title.clone() }</div>
                    <a class=classes!("text-blue-400", "underline") href={format!("tel:{}", user.phone.clone())}>{ user.phone.clone() }</a>
                </div>
            </div>
        }
    }
}

impl Component for Faces {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetUserNames);
        Self {
            link,
            users: Vec::new(),
            fetch_task: None
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::GetUserNames => {
                let request = Request::get("/users.json")
                    .body(Nothing)
                    .expect("Could not build request");
                let callback = self.link
                        .callback(|response: Response<Json<Result<Vec<SlackUser>, anyhow::Error>>>| {
                            let Json(data) = response.into_body();
                            Msg::SetUserNames(data.expect("could not get users"))
                        });
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                true
            }
            Msg::SetUserNames(users) => {
                self.users = users;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("bg-gray-100")>
                <div class=classes!("container", "mx-auto", "grid-cols-3", "grid", "gap-4", "p-10", "lg:grid-cols-6")>
                    { for self.users.iter().map(|user| self.view_user(user)) }
                </div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Faces>();
}
