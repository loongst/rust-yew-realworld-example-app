use yew::prelude::*;

use crate::error::Error;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub error: Option<Error>,
}

#[function_component]
pub fn ListErrors(props: &Props) -> Html {
    if let Some(error) = &props.error {
        html! {
            <ul class="error-messages">
                {
                    match error {
                        Error::UnprocessableEntity(error_info) => {
                            html! {
                                <>
                                {for error_info.errors.iter().map(|(key, value)| {
                                    html! {
                                        <li>
                                        { key }
                                        {for value.iter().map(|e| {
                                            html! {
                                                <>{" "} {e}</>
                                            }
                                        })}
                                        </li>
                                    }
                                })}
                                </>
                            }
                        }
                        _ => {
                            html! {
                                <li>{error.to_string()}</li>
                            }
                        }

                    }
                }
            </ul>
        }
    } else {
        html! {}
    }
}
