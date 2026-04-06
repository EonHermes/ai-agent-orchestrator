use std::path::Path;
use tantivy::{
    schema::{Schema, TEXT, FAST, STRING},
    document::Document,
    index::{Index, IndexReader, IndexWriter, ReloadPolicy},
    query::QueryParser,
    collector::TopDocs,
    Result as TantivyResult,
};
use crate::models::Snippet;

pub struct SearchIndex {
    index: Index,
    writer: IndexWriter,
}

impl SearchIndex {
    pub async fn new<P: AsRef<Path>>(path: P) -> TantivyResult<Self> {
        let mut schema = Schema::new();
        schema.add_text_field("title", STRING | FAST);
        schema.add_text_field("code", TEXT);
        schema.add_text_field("language", STRING);
        schema.add_text_field("tags", TEXT);
        schema.add_text_field("id", STRING | FAST);

        let index = Index::create_in_dir(path, schema)?;
        let writer = index.writer_with_num_threads(1, 50_000_000)?;

        Ok(SearchIndex { index, writer })
    }

    pub fn index_snippet(&self, snippet: &Snippet) -> TantivyResult<()> {
        let schema = self.index.schema();
        let title_field = schema.get_field("title").unwrap();
        let code_field = schema.get_field("code").unwrap();
        let language_field = schema.get_field("language").unwrap();
        let tags_field = schema.get_field("tags").unwrap();
        let id_field = schema.get_field("id").unwrap();

        let mut doc = Document::new();
        doc.add_text(title_field, &snippet.title);
        doc.add_text(code_field, &snippet.code);
        doc.add_text(language_field, &snippet.language);
        doc.add_text(tags_field, snippet.tags.as_deref().unwrap_or(""));
        doc.add_text(id_field, &snippet.id);

        self.writer.add_document(doc)?;
        self.writer.commit()?;
        Ok(())
    }

    pub fn remove_snippet(&self, snippet_id: &str) -> TantivyResult<()> {
        let id_field = self.index.schema().get_field("id").unwrap();
        let query = format!("id:{}", snippet_id);
        let query_parser = QueryParser::for_index(&self.index, vec![id_field]);
        let query = query_parser.parse_query(&query)?;

        self.writer.delete_query(query)?;
        self.writer.commit()?;
        Ok(())
    }

    pub fn search(&self, query: &str, top_k: usize) -> TantivyResult<Vec<(f32, String)>> {
        let reader: IndexReader = self.index.reader_builder().reload_policy(ReloadPolicy::OnCommit).try_into()?;
        let searcher = reader.searcher();

        let title_field = self.index.schema().get_field("title").unwrap();
        let code_field = self.index.schema().get_field("code").unwrap();
        let tags_field = self.index.schema().get_field("tags").unwrap();
        let query_parser = QueryParser::for_index(&self.index, vec![title_field, code_field, tags_field]);

        let query = query_parser.parse_query(query)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(top_k))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;
            let id = retrieved_doc
                .get_first(self.index.schema().get_field("id").unwrap())
                .and_then(|value| value.as_str())
                .map(String::from)
                .unwrap_or_default();

            results.push((score, id));
        }

        Ok(results)
    }
}