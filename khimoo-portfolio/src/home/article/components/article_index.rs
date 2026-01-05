use yew::prelude::*;
use yew_router::prelude::*;
use crate::home::data_loader::use_lightweight_articles;
use crate::home::routes::Route;
use crate::home::article::ArticleStyles;
use crate::styles::Theme;

#[function_component(ArticleIndex)]
pub fn article_index() -> Html {
    let (articles, loading, error) = use_lightweight_articles();

    if *loading {
        return html! {
            <>
                <style>{Theme::base_styles()}</style>
                <div style={ArticleStyles::index_container()}>
                    <h1>{"Articles"}</h1>
                    <p>{"Loading articles..."}</p>
                </div>
            </>
        };
    }

    if let Some(err) = error.as_ref() {
        return html! {
            <>
                <style>{Theme::base_styles()}</style>
                <div style={ArticleStyles::index_container()}>
                    <h1>{"Articles"}</h1>
                    <p style={format!("color: {};", Theme::ERROR_COLOR)}>
                        {format!("Error loading articles: {}", err)}
                    </p>
                </div>
            </>
        };
    }

    html! {
        <>
            <style>
                {Theme::base_styles()}
                {format!(
                    ".article-index-container a {{ color: {}; text-decoration: none; }}",
                    Theme::ACCENT_BLUE
                )}
                {format!(
                    ".article-meta {{ font-size: 12px; color: {}; }}",
                    Theme::TEXT_SECONDARY
                )}
            </style>
            <div class="article-index-container" style={ArticleStyles::index_container()}>
                <h1>{"記事一覧"}</h1>
                <BackToHomeButton />
                <ArticleList articles={articles} />
            </div>
        </>
    }
}

#[function_component(BackToHomeButton)]
fn back_to_home_button() -> Html {
    html! {
        <div style="margin-bottom: 20px;">
            <Link<Route> to={Route::Home}>
                <button style={Theme::button_style()}>
                    {"← Back to Home"}
                </button>
            </Link<Route>>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct ArticleListProps {
    articles: UseStateHandle<Option<Vec<crate::home::data_loader::LightweightArticle>>>,
}

#[function_component(ArticleList)]
fn article_list(props: &ArticleListProps) -> Html {
    if let Some(articles_list) = props.articles.as_ref() {
        html! {
            <ul style="list-style: none; padding: 0;">
                {
                    articles_list.iter().map(|article| {
                        html! {
                            <ArticleListItem key={article.slug.clone()} article={article.clone()} />
                        }
                    }).collect::<Html>()
                }
            </ul>
        }
    } else {
        html! { <p>{"No articles found."}</p> }
    }
}

#[derive(Properties, PartialEq)]
struct ArticleListItemProps {
    article: crate::home::data_loader::LightweightArticle,
}

#[function_component(ArticleListItem)]
fn article_list_item(props: &ArticleListItemProps) -> Html {
    let article = &props.article;
    
    html! {
        <li style={ArticleStyles::list_item()}>
            <h3 style="margin: 0 0 8px 0;">
                <Link<Route> to={Route::ArticleShow { slug: article.slug.clone() }}>
                    {&article.title}
                </Link<Route>>
            </h3>
            <ArticleSummary summary={article.summary.clone()} />
            <ArticleMetaInfo article={article.clone()} />
        </li>
    }
}

#[derive(Properties, PartialEq)]
struct ArticleSummaryProps {
    summary: Option<String>,
}

#[function_component(ArticleSummary)]
fn article_summary(props: &ArticleSummaryProps) -> Html {
    if let Some(summary) = &props.summary {
        html! { <p style="margin: 8px 0;">{summary}</p> }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct ArticleMetaInfoProps {
    article: crate::home::data_loader::LightweightArticle,
}

#[function_component(ArticleMetaInfo)]
fn article_meta_info(props: &ArticleMetaInfoProps) -> Html {
    let article = &props.article;
    
    html! {
        <div class="article-meta">
            <CategoryMeta category={article.metadata.category.clone()} />
            <span>{"Links: "}{article.inbound_links.len()}</span>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryMetaProps {
    category: Option<String>,
}

#[function_component(CategoryMeta)]
fn category_meta(props: &CategoryMetaProps) -> Html {
    if let Some(category) = &props.category {
        html! { <span style="margin-right: 16px;">{"Category: "}{category}</span> }
    } else {
        html! {}
    }
}