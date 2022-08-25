use std::collections::HashMap;
use share::article::article_base::ArticleHttp;
use share::article::article_statistics::ArticleStatisticsHttp;
use share::tag::tag_base::TagHttp;
use crate::article::article_statistics::sql::access::list_article_statistics;
use crate::tag::sql::access::list_tag_sql;
use crate::tag::sql::model::TagDB;
use crate::tag::tag_relation::sql::access::list_tag_relation_sql;

pub struct ArticleService<'a> {
    article_list: &'a mut Vec<ArticleHttp>,
}

impl <'a> ArticleService <'a> {
    pub fn new(article_list: &mut Vec<ArticleHttp>) -> ArticleService {
        ArticleService {
            article_list
        }
    }

    pub fn each_set_with_tag_list(&mut self) {
        let entity_id_list = self.article_list.iter()
            .map(|article| article.id.unwrap())
            .collect();

        let tag_relation_list =
            if let Some(tag_relation_list) = list_tag_relation_sql(
                entity_id_list, 1) {
                tag_relation_list
            } else {
                Vec::new()
            };

        let tag_id_list = tag_relation_list
            .iter()
            .map(|r| r.tag_id)
            .collect();

        let tag_list =
            if let Some(tag_list) = list_tag_sql(tag_id_list) {
                tag_list
            } else {
                Vec::new()
            };

        let relation_map = tag_relation_list.iter()
            .fold(HashMap::new(), |mut map: HashMap<i64, Vec<i64>>, relation| {
                if map.contains_key(&relation.entity_id) {
                    map.get_mut(&relation.entity_id).unwrap().push(relation.tag_id);
                } else {
                    let init = vec![relation.tag_id];
                    map.insert(relation.entity_id, init);
                }

                map
            });

        let tag_map = tag_list.iter()
            .fold(HashMap::new(), |mut map, tag| {
                map.insert(tag.id.unwrap(), tag);

                map
            });

        for article in self.article_list.iter_mut() {
            let tag_id_list = relation_map.get(&article.id.unwrap()).unwrap();

            let tag_list = tag_id_list.iter()
                .map(|id| tag_map.get(id).unwrap())
                .map(|tag| <TagDB as Into<TagHttp>>::into((*tag).clone()))
                .collect();

            article.tag_list = Some(tag_list);
        }
    }

    pub fn each_set_with_statistics(&mut self) {
        let article_id_list = self.article_list.iter()
            .map(|article| article.id.unwrap())
            .collect();

        if let Some(data_list) = list_article_statistics(article_id_list) {
            let map: HashMap<i64, ArticleStatisticsHttp> = data_list.into_iter()
                .map(|data| data.into())
                .collect::<Vec<ArticleStatisticsHttp>>().into_iter()
                .fold(HashMap::new(), |mut map: HashMap<i64, ArticleStatisticsHttp>, data: ArticleStatisticsHttp| {
                    map.insert(data.article_id, data);

                    map
                });

            self.article_list.iter_mut()
                .for_each(|article| {
                    let statistics = map.get(&article.id.unwrap()).unwrap();

                    article.statistics = Some((*statistics).clone());
                })
        }
    }
}