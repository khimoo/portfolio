use yew::prelude::*;
use super::types::{ArticlesData, ProcessedArticle, LightweightArticle, DataLoadError};
use super::api_client::DataLoader;

/// Hook for using DataLoader in Yew components
#[hook]
pub fn use_data_loader() -> UseStateHandle<Option<DataLoader>> {
    use_state(|| Some(DataLoader::new()))
}

/// Hook for loading articles data
#[hook]
pub fn use_articles_data() -> (UseStateHandle<Option<ArticlesData>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let data = data.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_articles().await {
                    Ok(articles_data) => {
                        data.set(Some(articles_data));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });

            || {}
        });
    }

    (data, loading, error)
}

/// Hook for loading lightweight articles (for list display)
#[hook]
pub fn use_lightweight_articles() -> (UseStateHandle<Option<Vec<LightweightArticle>>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| true);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            let data = data.clone();
            let loading = loading.clone();
            let error = error.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let loader = DataLoader::new();
                match loader.load_lightweight_articles().await {
                    Ok(lightweight_articles) => {
                        data.set(Some(lightweight_articles));
                        error.set(None);
                    }
                    Err(e) => {
                        error.set(Some(e));
                    }
                }
                loading.set(false);
            });

            || {}
        });
    }

    (data, loading, error)
}

/// Hook for loading a specific article by slug (with caching)
#[hook]
pub fn use_article_content(slug: Option<String>) -> (UseStateHandle<Option<ProcessedArticle>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(slug.clone(), move |slug| {
            if let Some(slug) = slug {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("use_article_content: Loading article with slug: {}", slug).into());

                let data = data.clone();
                let loading = loading.clone();
                let error = error.clone();
                let slug = slug.clone();

                loading.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let loader = DataLoader::new();
                    
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"use_article_content: Created DataLoader, calling load_article_by_slug".into());

                    match loader.load_article_by_slug(&slug).await {
                        Ok(article) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("use_article_content: Successfully loaded article: {}", article.title).into());
                            data.set(Some(article));
                            error.set(None);
                        }
                        Err(e) => {
                            #[cfg(target_arch = "wasm32")]
                            web_sys::console::log_1(&format!("use_article_content: Failed to load article: {}", e).into());
                            error.set(Some(e));
                        }
                    }
                    loading.set(false);
                });
            } else {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&"use_article_content: No slug provided".into());
                data.set(None);
                loading.set(false);
                error.set(None);
            }

            || {}
        });
    }

    (data, loading, error)
}

/// Hook for loading article content with caching
#[hook]
pub fn use_cached_article_content(file_path: Option<String>) -> (UseStateHandle<Option<String>>, UseStateHandle<bool>, UseStateHandle<Option<DataLoadError>>) {
    let data = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None);

    {
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();

        use_effect_with(file_path.clone(), move |file_path| {
            if let Some(file_path) = file_path {
                let data = data.clone();
                let loading = loading.clone();
                let error = error.clone();
                let file_path = file_path.clone();

                loading.set(true);
                error.set(None);

                wasm_bindgen_futures::spawn_local(async move {
                    let loader = DataLoader::new();
                    match loader.load_article_content_only(&file_path).await {
                        Ok(content) => {
                            data.set(Some(content));
                            error.set(None);
                        }
                        Err(e) => {
                            error.set(Some(e));
                        }
                    }
                    loading.set(false);
                });
            } else {
                data.set(None);
                loading.set(false);
                error.set(None);
            }

            || {}
        });
    }

    (data, loading, error)
}