use yew::prelude::*;
use yew_router::prelude::*;
use crate::styles::{Theme, CommonStyles};
use crate::home::routes::Route;

#[function_component(LoadingView)]
pub fn loading_view() -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={CommonStyles::loading_container()}>
                <div style={CommonStyles::centered_text()}>
                    <h2>{"記事データを読み込み中..."}</h2>
                    <div style="margin-top: 20px;">
                        <div style={Theme::loading_spinner_style()}></div>
                    </div>
                </div>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ErrorViewProps {
    pub error_message: String,
}

#[function_component(ErrorView)]
pub fn error_view(props: &ErrorViewProps) -> Html {
    html! {
        <>
            <style>{Theme::base_styles()}</style>
            <div style={CommonStyles::error_container()}>
                <div style={CommonStyles::centered_text()}>
                    <h2>{"データの読み込みに失敗しました"}</h2>
                    <p>{format!("エラー: {}", props.error_message)}</p>
                    <div style="margin-top: 20px;">
                        <Link<Route> to={Route::Home}>
                            <button style={Theme::button_style()}>
                                {"← ホームに戻る"}
                            </button>
                        </Link<Route>>
                    </div>
                </div>
            </div>
        </>
    }
}