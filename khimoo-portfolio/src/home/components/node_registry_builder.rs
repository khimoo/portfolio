use std::collections::HashMap;
use crate::home::data_loader::{ArticlesData, ProcessedArticle};
use crate::home::types::{NodeRegistry, NodeId, Position, NodeContent, ContainerBound};
use crate::config::get_config;

/// Builder for creating NodeRegistry from articles data
pub struct NodeRegistryBuilder;

impl NodeRegistryBuilder {
    /// Create NodeRegistry from articles data with proper positioning
    pub fn build_from_articles(
        articles_data: &ArticlesData,
        container_bound: &ContainerBound,
    ) -> (NodeRegistry, HashMap<NodeId, String>) {
        let mut registry = NodeRegistry::new();
        let mut slug_to_id = HashMap::new();
        let mut id_to_slug = HashMap::new();
        let mut next_id = 1u32;

        let center_x = container_bound.width / 2.0;
        let center_y = container_bound.height / 2.0;

        #[cfg(target_arch = "wasm32")]
        {
            web_sys::console::log_1(
                &format!(
                    "Building registry with container: {:?}, center: ({}, {})",
                    container_bound, center_x, center_y
                ).into(),
            );
        }

        let home_articles: Vec<_> = articles_data
            .articles
            .iter()
            .filter(|article| article.metadata.home_display)
            .collect();

        if home_articles.is_empty() {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::warn_1(&"No home articles found! Creating fallback".into());
            
            return Self::create_fallback_registry(center_x, center_y);
        }

        let radius = (container_bound.width.min(container_bound.height) * 0.3).max(150.0);
        let angle_step = 2.0 * std::f32::consts::PI / home_articles.len() as f32;

        for (index, article) in home_articles.iter().enumerate() {
            let node_id = NodeId(next_id);
            let content = Self::determine_node_content(article);
            let (position, base_radius) = Self::calculate_node_position(
                article, index, center_x, center_y, radius, angle_step
            );

            registry.add_node(node_id, position, base_radius, content);
            registry.set_node_importance(node_id, article.metadata.importance.unwrap_or(3));
            registry.set_node_inbound_count(node_id, article.inbound_links.len());

            slug_to_id.insert(article.slug.clone(), node_id);
            id_to_slug.insert(node_id, article.slug.clone());
            next_id += 1;
        }

        Self::add_article_links(&mut registry, &home_articles, &slug_to_id);

        (registry, id_to_slug)
    }

    /// Determine node content based on article metadata
    fn determine_node_content(article: &ProcessedArticle) -> NodeContent {
        if let Some(image_url) = &article.metadata.author_image {
            let optimized_image_url = Self::get_optimized_image_url(image_url);
            
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(
                &format!(
                    "Creating author node for '{}' with image: '{}'",
                    article.title, optimized_image_url
                ).into(),
            );

            NodeContent::Author {
                name: article.title.clone(),
                image_url: optimized_image_url,
                bio: None,
            }
        } else {
            NodeContent::Text(article.title.clone())
        }
    }

    /// Get optimized image URL for author images
    fn get_optimized_image_url(image_url: &str) -> String {
        if image_url.starts_with("articles/") || image_url.starts_with("/articles/") {
            let optimized_path = image_url
                .replace("articles/img/author_img.png", "articles/img/author_img_medium.png")
                .replace("/articles/img/author_img.png", "/articles/img/author_img_medium.png");
            get_config().get_url(&optimized_path)
        } else {
            get_config().get_url(image_url)
        }
    }

    /// Calculate node position based on whether it's an author node or regular article
    fn calculate_node_position(
        article: &ProcessedArticle,
        index: usize,
        center_x: f32,
        center_y: f32,
        radius: f32,
        angle_step: f32,
    ) -> (Position, i32) {
        if article.metadata.author_image.is_some() {
            // Author node at center with larger size
            (Position { x: center_x, y: center_y }, 60)
        } else {
            // Regular articles in circular arrangement
            let angle = index as f32 * angle_step;
            let x = center_x + radius * angle.cos();
            let y = center_y + radius * angle.sin();
            (Position { x, y }, 30)
        }
    }

    /// Add links between articles
    fn add_article_links(
        registry: &mut NodeRegistry,
        home_articles: &[&ProcessedArticle],
        slug_to_id: &HashMap<String, NodeId>,
    ) {
        for article in home_articles {
            if let Some(&from_id) = slug_to_id.get(&article.slug) {
                for link in &article.outbound_links {
                    if let Some(&to_id) = slug_to_id.get(&link.target_slug) {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(
                            &format!(
                                "Adding edge: {} -> {} (IDs: {} -> {})",
                                article.slug, link.target_slug, from_id.0, to_id.0
                            ).into(),
                        );
                        registry.add_edge(from_id, to_id);
                    }
                }
            }
        }
    }

    /// Create fallback registry when no articles are found
    fn create_fallback_registry(center_x: f32, center_y: f32) -> (NodeRegistry, HashMap<NodeId, String>) {
        let mut registry = NodeRegistry::new();
        let mut id_to_slug = HashMap::new();

        registry.add_node(
            NodeId(1),
            Position { x: center_x, y: center_y },
            40,
            NodeContent::Text("Author".to_string()),
        );
        
        id_to_slug.insert(NodeId(1), "author".to_string());
        (registry, id_to_slug)
    }
}