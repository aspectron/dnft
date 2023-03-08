#[cfg(not(target_arch = "wasm32"))]
use workflow_http::error::Error;
// #[cfg(not(target_arch = "wasm32"))]
// use async_std::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<(), Error> {
    use workflow_http::{stores, BasicAuthenticator, Router};
    //use duct::cmd;
    use std::{
        collections::HashMap,
        io::Error as IoError,
        path::{PathBuf, Path},
        sync::Arc,
        collections::BTreeMap, fs, io::Read
    };

    use async_std::{fs::OpenOptions, io};
    use tide::{prelude::*, Request};
    use multipart::server::Multipart;
    use rand::distributions::{Alphanumeric, DistString};

    let files_path = std::env::current_dir()?.join("files");
    if !files_path.exists(){
        fs::create_dir(files_path)?;
    }

    #[derive(Clone, Debug)]
    struct DataDirState {
        dir: Arc<PathBuf>,
    }

    impl DataDirState {
        fn try_new() -> Result<Self, IoError> {
            let cwd = std::env::current_dir()?.join("files");
            Ok(Self {
                dir: Arc::new(cwd),
            })
        }

        fn path(&self) -> &Path {
            self.dir.as_path()
        }
    }

    //let args: Vec<String> = std::env::args().collect();

    // ~~~

    /*
    if Path::new("./node_modules").exists() != true {
        println!("\n\nnode_modules folder is absent... running npm install...\n");
        cmd!("npm", "install").run()?;
    }

    if Path::new("./root/workflow").exists() != true {
        println!("\n\nworkflow wasm folder is absent... running wasm-pack...\n");
        cmd!(
            "wasm-pack",
            "build",
            "--target",
            "web",
            "--out-name",
            "workflow",
            "--out-dir",
            "../root/workflow",
            "--features",
            "test"
        )
        .dir("wasm")
        .run()?;
    }
    */

    // ~~~

    tide::log::start();

    let cwd = std::env::current_dir().unwrap();
    let mount_map = HashMap::new();
    let source_map = HashMap::new();
    let overrides = BTreeMap::new();
    /*[
        (
            "eventemitter3".to_string(),
            "/lib/esm/eventemitter3.js".to_string(),
        ),
        (
            "@solana/web3.js".to_string(),
            "/lib/esm/solana.js".to_string(),
        ),
    ]);*/
    /*
    let adapters = fs::read_dir("./node_modules/@solana")
        .map_err(|e| {
            panic!(
                "\n\nError reading './node_modules/@solana'...\n{}\n\nDid you run npm install?\n\n",
                e.to_string()
            );
        })
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<std::result::Result<Vec<_>, io::Error>>()?;
    //println!("adapters: {:#?}", adapters);

    for adapter in adapters {
        let path = adapter.as_path();
        //println!("path: {:?}", path);
        if let Some(s) = path.to_str() {
            if !s.contains(&"wallet-adapter-") {
                continue;
            }
            if let Some(file_name) = path.file_name() {
                if let Some(name) = file_name.to_str() {
                    //println!("name: {}", &name);
                    if name.eq("wallet-adapter-solflare") {
                        overrides
                            .insert(format!("@solana/{}", name), format!("/lib/esm/solflare.js"));
                    } else {
                        overrides.insert(
                            format!("@solana/{}", name),
                            format!("/node_modules/@solana/{}/lib/esm/index.js", name),
                        );
                    }
                }
            }
        }
    }
    */

    let router = Router::new_with_overrides(cwd.clone(), mount_map, source_map, overrides);

    let tide_secret: &[u8] = &(0..64).map(|_| rand::random::<u8>()).collect::<Vec<u8>>();

    let mut app = tide::with_state(DataDirState::try_new()?);
    app.with(tide::log::LogMiddleware::new());

    let file = cwd.as_path().join(".auth");
    if file.exists() {
        let memory_store = stores::from_hjson_file(file.as_path())?;
        let authenticator = BasicAuthenticator::new(memory_store);
        authenticator.init(&mut app);
    }

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        tide_secret
    ));

    app.with(tide::utils::Before(
        |mut request: tide::Request<DataDirState>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));

    app.at("/").serve_file("../wasm/web/index.html")?;

    app.at("/reset")
        .get(|mut req: tide::Request<DataDirState>| async move {
            req.session_mut().destroy();
            Ok(tide::Redirect::new("/"))
        });

    router.init(&mut app);
    app.at("/").serve_dir("../wasm/web/")?;
    app.at("/file").serve_dir("files/")?;
    //app.at("/node_modules").serve_dir("node_modules/")?;

    app.at("upload/file").post(|mut req: Request<DataDirState>| async move {
        let mut file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
        //let mut file_name = "xyz".to_string();
        let content_type = req.content_type().unwrap();
        let boundary = content_type.param("boundary").unwrap();
        let req_bytes = std::io::Cursor::new(req.body_bytes().await?);//Req::from_request(req).await?;
        let mut multipart_req = Multipart::with_body(req_bytes, boundary.as_str());
        let mut file_bytes = vec![];
        multipart_req.foreach_entry(|mut field|{
            let name = &field.headers.name;
            //println!("name: {name:?}");
            // if name.eq(&"name".into()) && field.is_text(){
            //     let mut string = String::new();
            //     let _res = field.data.read_to_string(&mut string);
            //     //let string = String::from_utf8_lossy(&string);
            //     println!("text: {string:?}, res:{_res:?}");
            // }else 
            if !name.eq(&"file".into()){
                return;
            }
            
            // let name = match field.headers.filename{
            //     Some(value)=>value,
            //     None=>return
            // };
            let mime = match field.headers.content_type{
                Some(value)=>value,
                None=>return
            };
            match (mime.type_(), mime.subtype()){
                (mime::IMAGE, mime::JPEG)=>{
                    file_name.push_str(".jpg");
                }
                (mime::IMAGE, mime::PNG)=>{
                    file_name.push_str(".png");
                }
                (mime::IMAGE, mime::SVG)=>{
                    file_name.push_str(".svg");
                }
                (mime::IMAGE, mime::BMP)=>{
                    file_name.push_str(".bmp");
                }
                _=>{

                }
            }
            let _res = field.data.read_to_end(&mut file_bytes);
                
        })?;
        //println!("body###: {multipart_req:?}");
        if file_bytes.is_empty(){
            return Ok(json!({"error": "Unable to read file. Please try again." }))
        }
        let fs_path = req.state().path().join(&file_name);
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(&fs_path)
            .await?;

        let data = async_std::io::Cursor::new(file_bytes);
        io::copy(data, file).await?;

        Ok(json!({"success":true, "file": format!("file/{file_name}") }))
    });

    app.listen("0.0.0.0:8085").await?;

    Ok(())
}

#[cfg(target_arch = "wasm32")]
fn main() -> std::result::Result<(), String> {
    panic!("wasm32 target is not supported");
}
