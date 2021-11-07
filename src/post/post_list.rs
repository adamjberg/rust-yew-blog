use super::super::{Post};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub posts: Vec<Post>,
}

pub struct PostList {
    props: Props,
}

impl PostList {
    fn render_list(&self, posts: &Vec<Post>) -> Html {
        html! {
            { posts.iter().map(|post| self.view_post(post)).collect::<Html>() }
        }
    }

    fn view_post(&self, post: &Post) -> Html {
        html! {
            <div>
                <a href=post.url.clone()>
                    { &post.title }
                </a>
            </div>
        }
    }
}

pub enum Msg {}

impl Component for PostList {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                { self.render_list(&self.props.posts) }
            </div>
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}
