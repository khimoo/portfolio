use yew::prelude::*;
use crate::home::data_loader::{use_article_content, DataLoader};
use crate::home::article::components::{
    ArticleLoadingView, ArticleErrorView, ContentLoadingView,
    ContentErrorView, ArticleContent
};

#[derive(Properties, PartialEq)]
pub struct ArticleViewProps {
    pub slug: String,
}

#[function_component(ArticleView)]
pub fn article_view(props: &ArticleViewProps) -> Html {
    // Load article metadata
    let (article, loading, error) = use_article_content(Some(props.slug.clone()));

    // Load article content
    let article_content = use_state(|| None::<String>);
    let content_loading = use_state(|| false);
    let content_error = use_state(|| None::<String>);

    // Load content when article metadata is available
    {
        let article = article.clone();
        let article_content = article_content.clone();
        let content_loading = content_loading.clone();
        let content_error = content_error.clone();

        use_effect_with(article.clone(), move |article| {
            if let Some(article_data) = article.as_ref() {
                let file_path = article_data.file_path.clone();
                let article_content = article_content.clone();
                let content_loading = content_loading.clone();
                let content_error = content_error.clone();

                content_loading.set(true);
                content_error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let loader = DataLoader::new();
                    match loader.load_article_content_only(&file_path).await {
                        Ok(content) => {
                            article_content.set(Some(content));
                            content_error.set(None);
                        }
                        Err(e) => {
                            content_error.set(Some(format!("{}", e)));
                        }
                    }
                    content_loading.set(false);
                });
            }
            || {}
        });
    }

    // Handle loading and error states
    if *loading {
        return html! { <ArticleLoadingView /> };
    }

    if let Some(err) = error.as_ref() {
        return html! { <ArticleErrorView error_message={err.to_string()} /> };
    }

    if let Some(article_data) = article.as_ref() {
        if *content_loading {
            return html! { <ContentLoadingView title={article_data.title.clone()} /> };
        }

        if let Some(err) = content_error.as_ref() {
            return html! {
                <ContentErrorView
                    title={article_data.title.clone()}
                    error_message={err.clone()}
                />
            };
        }

        if let Some(content) = article_content.as_ref() {
            // Simplified: single component call with integrated header and content
            return html! {
                <ArticleContent 
                    article={article_data.clone()} 
                    content={content.clone()} 
                />
            };
        }
    }

    // Fallback
    html! { <ArticleErrorView error_message={"Article not found".to_string()} /> }
}
