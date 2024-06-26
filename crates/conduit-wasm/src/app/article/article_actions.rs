use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;
use crate::services::articles::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
    pub can_modify: bool,
}

/// Article actions component to edit or delete an article.
#[function_component]
pub fn ArticleActions(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let article_delete = {
        let slug = props.slug.clone();
        use_async(async move { del(slug).await })
    };
    let onclick = {
        let article_delete = article_delete.clone();
        Callback::from(move |_| {
            article_delete.run();
        })
    };

    use_effect_with(article_delete, move |article_delete| {
        if article_delete.data.is_some() {
            navigator.push(&AppRoute::Home);
        }
        || ()
    });

    if props.can_modify {
        html! {
            <span>
                <Link<AppRoute> to={AppRoute::Editor { slug: props.slug.clone() }} classes="btn btn-outline-secondary btn-sm" >
                    { "Edit Article" }
                </Link<AppRoute>>
                { " " }
                <button class="btn btn-outline-danger btn-sm" {onclick} >
                    <i class="ion-trash-a"></i> { "Delete Article" }
                </button>
            </span>
        }
    } else {
        html! {
            <span>
            </span>
        }
    }
}
