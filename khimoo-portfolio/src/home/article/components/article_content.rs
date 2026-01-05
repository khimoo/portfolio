use yew::prelude::*;
use yew_router::prelude::*;
use crate::home::data_loader::ProcessedArticle;
use crate::home::routes::Route;
use crate::home::article::{MarkdownProcessor, ArticleStyles};
use crate::config::get_config;

#[derive(Properties, PartialEq)]
pub struct ArticleContentProps {
    pub article: ProcessedArticle,
    pub content: String,
}

#[function_component(ArticleContent)]
pub fn article_content(props: &ArticleContentProps) -> Html {
    let rendered_content = MarkdownProcessor::process_to_html(&props.content);

    html! {
        <>
            <style>
                {ArticleStyles::base_styles()}
            </style>
            <div style={ArticleStyles::unified_article_container()}>
                <article>
                    <ArticleHeaderSection article={props.article.clone()} />
                    <main class="markdown-body">
                        {rendered_content}
                    </main>
                    <RelatedArticlesSection article={props.article.clone()} />
                </article>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ArticleHeaderSectionProps {
    article: ProcessedArticle,
}

#[function_component(ArticleHeaderSection)]
fn article_header_section(props: &ArticleHeaderSectionProps) -> Html {
    let article = &props.article;

    html! {
        <header style={ArticleStyles::integrated_article_header()}>
            <div style="flex: 1;">
                <h1 style={ArticleStyles::integrated_article_title()}>{&article.title}</h1>
                <div style={ArticleStyles::integrated_article_meta()}>
                    <CategoryInfo category={article.metadata.category.clone()} />
                    <ImportanceInfo importance={article.metadata.importance} />
                    <InboundLinksInfo inbound_count={article.inbound_links.len()} />
                    <TagsInfo tags={article.metadata.tags.clone()} />
                </div>
            </div>
            <AuthorImageDisplay author_image={article.metadata.author_image.clone()} />
        </header>
    }
}

#[derive(Properties, PartialEq)]
struct CategoryInfoProps {
    category: Option<String>,
}

#[function_component(CategoryInfo)]
fn category_info(props: &CategoryInfoProps) -> Html {
    if let Some(category) = &props.category {
        html! {
            <span>{"Category: "}<strong>{category}</strong></span>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct ImportanceInfoProps {
    importance: Option<u8>,
}

#[function_component(ImportanceInfo)]
fn importance_info(props: &ImportanceInfoProps) -> Html {
    if let Some(importance) = props.importance {
        html! {
            <span>{"Importance: "}<strong>{importance}{"/5"}</strong></span>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct InboundLinksInfoProps {
    inbound_count: usize,
}

#[function_component(InboundLinksInfo)]
fn inbound_links_info(props: &InboundLinksInfoProps) -> Html {
    html! {
        <span>{"Inbound links: "}<strong>{props.inbound_count}</strong></span>
    }
}

#[derive(Properties, PartialEq)]
struct TagsInfoProps {
    tags: Vec<String>,
}

#[function_component(TagsInfo)]
fn tags_info(props: &TagsInfoProps) -> Html {
    if !props.tags.is_empty() {
        html! {
            <span>
                {"Tags: "}
                {
                    props.tags.iter().enumerate().map(|(i, tag)| {
                        html! {
                            <>
                                {if i > 0 { ", " } else { "" }}
                                <span style={ArticleStyles::tag_style()}>
                                    {tag}
                                </span>
                            </>
                        }
                    }).collect::<Html>()
                }
            </span>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct AuthorImageDisplayProps {
    author_image: Option<String>,
}

#[function_component(AuthorImageDisplay)]
fn author_image_display(props: &AuthorImageDisplayProps) -> Html {
    if let Some(author_image) = &props.author_image {
        let resolved_image_path = get_config().get_url(author_image);
        html! {
            <div style={ArticleStyles::author_image_container()}>
                <img
                    src={resolved_image_path}
                    alt="Author image"
                    style={ArticleStyles::author_image()}
                />
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
struct RelatedArticlesSectionProps {
    article: ProcessedArticle,
}

#[function_component(RelatedArticlesSection)]
fn related_articles_section(props: &RelatedArticlesSectionProps) -> Html {
    if !props.article.outbound_links.is_empty() {
        html! {
            <footer style={ArticleStyles::related_articles_footer()}>
                <h3 style="color: #e0e0e0;">{"Related Articles"}</h3>
                <ul style={ArticleStyles::related_articles_list()}>
                    {
                        props.article.outbound_links.iter().map(|link| {
                            html! {
                                <li key={link.target_slug.clone()} style={ArticleStyles::related_articles_item()}>
                                    <Link<Route> to={Route::ArticleShow { slug: link.target_slug.clone() }}>
                                        {&link.target_slug}
                                    </Link<Route>>
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </footer>
        }
    } else {
        html! {}
    }
}

// Legacy component for backward compatibility (will be removed)
#[derive(Properties, PartialEq)]
struct RelatedArticlesProps {
    article: ProcessedArticle,
}

#[function_component(RelatedArticles)]
fn related_articles(props: &RelatedArticlesProps) -> Html {
    html! { <RelatedArticlesSection article={props.article.clone()} /> }
}