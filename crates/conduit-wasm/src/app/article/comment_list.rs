use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use super::comment::Comment;
use super::comment_input::CommentInput;
use crate::app::AppRoute;
use crate::hooks::use_user_context;
use crate::services::comments::*;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: String,
}

/// A comment list component of an article.
#[function_component]
pub fn CommentList(props: &Props) -> Html {
    let user_ctx = use_user_context();
    let comment_list = {
        let slug = props.slug.clone();
        use_async_with_options(
            async move { for_article(slug).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let callback_added = {
        let comment_list = comment_list.clone();
        Callback::from(move |comment_info| {
            if let Some(mut list) = comment_list.data.clone() {
                list.comments.insert(0, comment_info);
                comment_list.update(list);
            }
        })
    };
    let callback_deleted = {
        let comment_list = comment_list.clone();
        Callback::from(move |comment_id| {
            if let Some(mut list) = comment_list.data.clone() {
                list.comments.retain(|c| c.id != comment_id);
                comment_list.update(list);
            }
        })
    };

    if let Some(comment_list) = &comment_list.data {
        html! {
            <div class="col-xs-12 col-md-8 offset-md-2">
                {
                    if user_ctx.is_authenticated() {
                        html! {
                            <div>
                                <CommentInput
                                    slug={props.slug.clone()}
                                    callback={callback_added} />
                            </div>
                        }
                    } else {
                        html! {
                            <p>
                                <Link<AppRoute> to={AppRoute::Login} classes="nav-link">
                                    { "Sign in" }
                                </Link<AppRoute>>
                                { " or " }
                                <Link<AppRoute> to={AppRoute::Register} classes="nav-link">
                                    { "sign up" }
                                </Link<AppRoute>>
                                { " to add comments on this article." }
                            </p>
                        }
                    }
                }
                <div>
                    {for comment_list.comments.iter().map(|comment| {
                        html! {
                            <Comment
                                slug={props.slug.clone()}
                                comment={comment.clone()}
                                callback={callback_deleted.clone()} />
                        }
                    })}
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
