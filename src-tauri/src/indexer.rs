use std::error::Error;
use std::fs;
use std::time::Instant;
use tantivy::directory::MmapDirectory;
use tantivy::schema::{Schema, Value, STORED, TEXT};
use tantivy::{Index, IndexWriter, TantivyDocument};

pub  fn index_all_files() -> Result<(),Box<dyn Error>>{
    let path = "./index";
    fs::create_dir_all(path).unwrap();
    let directory = MmapDirectory::open(path)?;
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("name", TEXT | STORED);
    schema_builder.add_text_field("full_path", TEXT | STORED);
    
    if Index::exists(&directory)? { println!("index already exists. skip indexing"); return Ok(()); }
    
    let schema = schema_builder.build();
    let index = Index::open_or_create(directory, schema.clone())?;
    let mut index_writer:IndexWriter = index.writer(50_000_000)?;
    
    let name = schema.get_field("name")?;
    let full_path = schema.get_field("full_path")?;

    let mut stack = vec![String::from("C:\\")];
    
    while let indexing_target_path = stack.pop() {
        let indexing_target_path = match indexing_target_path {
            Some(path) => path,
            None => break
        };
        println!("Indexing: {}", indexing_target_path);
        let paths = match std::fs::read_dir(indexing_target_path){
            Ok(paths) => paths,
            Err(_e) => continue
        };
        for item in paths {
            let item = item?;
            let metadata = item.metadata()?;
            if metadata.is_dir() {
                stack.push(item.path().to_str().unwrap().to_string());
            } else {
                let mut doc = TantivyDocument::new();
                doc.add_text(name, item.file_name().to_str().unwrap());
                doc.add_text(full_path, item.path().to_str().unwrap());
                index_writer.add_document(doc)?;
            }
        }
    }
    
    index_writer.commit()?;
    Ok(())
}

pub fn search_files(query:&str) -> Result<Vec<String>,Box<dyn Error>> {
    let index = Index::open_in_dir("./index")?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    let name = schema.get_field("name")?;
    let full_path = schema.get_field("full_path")?;
    
    let query_parser = tantivy::query::QueryParser::for_index(&index, vec![name, full_path]);
    let query = query_parser.parse_query(query)?;
    //map searchresult to full_path lists
    let top_docs = searcher.search(&query, &tantivy::collector::TopDocs::with_limit(100))?;
    let mut result_vec:Vec<String> = Vec::new();
    
    let start = Instant::now();
    for (_score, doc_address) in top_docs {
        let retrieved_doc:TantivyDocument = searcher.doc(doc_address)?;
        let full_path = match retrieved_doc.get_first(full_path){
            Some(s) => s.as_str().unwrap().to_string(),
            None => continue
        };
        println!("Found: {}", &full_path);
        result_vec.push(full_path);
    }
    
    println!("search took: {:?}", start.elapsed());
    
    Ok(result_vec)
}