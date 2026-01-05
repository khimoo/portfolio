use yew::prelude::*;
use yew_router::prelude::*;
use crate::styles::Theme;
use crate::home::routes::Route;
use crate::home::article::ArticleStyles;

#[function_component(ArticleLoadingView)]
pub fn article_loading_view() -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={ArticleStyles::index_container()}>
                <div style="margin-bottom: 20px;">
                    <Link<Route> to={Route::Home}>
                        <button style={Theme::button_style()}>
                            {"← Back to Home"}
                        </button>
                    </Link<Route>>
                </div>
                <h1>{"Loading article..."}</h1>
                <div style="margin-top: 20px;">
                    <div style={Theme::loading_spinner_style()}></div>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ArticleErrorViewProps {
    pub error_message: String,
}

#[function_component(ArticleErrorView)]
pub fn article_error_view(props: &ArticleErrorViewProps) -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={ArticleStyles::index_container()}>
                <div style="margin-bottom: 20px;">
                    <Link<Route> to={Route::Home}>
                        <button style={Theme::button_style()}>
                            {"← Back to Home"}
                        </button>
                    </Link<Route>>
                </div>
                <h1>{"Article Not Found"}</h1>
                <p style={format!("color: {};", Theme::ERROR_COLOR)}>
                    {format!("Error: {}", props.error_message)}
                </p>
                <p>{"The article you're looking for doesn't exist or couldn't be loaded."}</p>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContentLoadingViewProps {
    pub title: String,
}

#[function_component(ContentLoadingView)]
pub fn content_loading_view(props: &ContentLoadingViewProps) -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={ArticleStyles::index_container()}>
                <h1>{&props.title}</h1>
                <div style="margin-top: 20px;">
                    <div style={Theme::loading_spinner_style()}></div>
                </div>
                <p>{"Loading content..."}</p>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ContentErrorViewProps {
    pub title: String,
    pub error_message: String,
}

#[function_component(ContentErrorView)]
pub fn content_error_view(props: &ContentErrorViewProps) -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={ArticleStyles::index_container()}>
                <h1>{&props.title}</h1>
                <p style={format!("color: {};", Theme::ERROR_COLOR)}>
                    {format!("Failed to load content: {}", props.error_message)}
                </p>
            </div>
        </>
    }
}