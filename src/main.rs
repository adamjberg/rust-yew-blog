mod post;
use serde_derive::Deserialize;
use yew::prelude::*;

enum Msg {

}

struct Model {
    posts: Vec<Post>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub title: String,
    pub url: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            posts: vec![
                Post { title: "Building My First Command Line Interface (CLI) with Rust".to_string(), url:  "https://devtails.medium.com/building-my-first-command-line-interface-cli-with-rust-b6beb9c284e0".to_string() },
                Post { title: "Bundling Your Node.js Express App with esbuild".to_string(), url:  "https://devtails.medium.com/bundling-your-node-js-express-app-with-esbuild-5aecc36c5047".to_string() },
                Post { title: "The Night Technology Betrayed Me: A Halloween Horror Story".to_string(), url:  "https://devtails.medium.com/the-night-technology-betrayed-me-a-halloween-horror-story-c85b91750835".to_string() },
                Post { title: "Hacking Together 4 Apps in 24 Hours".to_string(), url:  "https://devtails.medium.com/hacking-together-4-apps-in-24-hours-69972792af72".to_string() },
                Post { title: "How to Setup a Django Admin Page For MongoDB".to_string(), url:  "https://devtails.medium.com/how-to-setup-a-django-admin-page-for-mongodb-fcc893523756".to_string() },
            ]
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <nav class="navbar navbar-dark bg-dark">
                    <div class="container">
                        <a class="navbar-brand" href="#">{ "devtails" }</a>
                    </div>
                </nav>
                <post::post_list::PostList posts=self.posts.clone()/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}