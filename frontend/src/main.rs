mod tests;
mod css;
mod article;
mod index;
mod dashboard;

use yew::prelude::*;
use yew_router::{Routable, BrowserRouter};
use yew_router::prelude::*;
use crate::article::Article;
use crate::index::Index;
use crate::dashboard::index::DashboardIndex;
use crate::dashboard::article::manage::DashboardArticleManage;
use crate::dashboard::article::create::DashboardArticleCreate;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Index,

    #[at("/article")]
    Article,

    #[at("/dashboard")]
    Dashboard,

    #[at("/dashboard/article/manage")]
    DashboardArticleManage,

    #[at("/dashboard/article/create")]
    DashboardArticleCreate,
}


fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index => {
            html! {
                <Index />
            }
        },

        Route::Article => {
            html! {
                <Article />
            }
        },

        Route::Dashboard => {
            html! {
                <DashboardIndex />
            }
        },

        Route::DashboardArticleManage => {
            html! {
                <DashboardArticleManage />
            }
        },

        Route::DashboardArticleCreate => {
            html! {
                <DashboardArticleCreate />
            }
        }
    }
}

pub struct App {

}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {

        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <Switch<Route> render={Switch::render(switch)} />
                </BrowserRouter>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
