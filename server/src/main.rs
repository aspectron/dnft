//#[cfg(not(target_arch = "wasm32"))]
//use workflow_http::error::Error;

#[cfg(not(target_arch = "wasm32"))]
#[async_std::main]
async fn main() -> Result<(), dnft::client::error::Error> {
    use std::{
        fs,
        io::Error as IoError,
        io::Read,
        path::{Path, PathBuf},
        sync::Arc,
    };

    use async_std::{fs::OpenOptions, io};
    use multipart::server::Multipart;
    use rand::distributions::{Alphanumeric, DistString};
    use tide::{prelude::*, Request};

    let files_path = std::env::current_dir()?.join("files");
    if !files_path.exists() {
        fs::create_dir(files_path)?;
    }

    #[derive(Clone, Debug)]
    struct DataDirState {
        dir: Arc<PathBuf>,
    }

    impl DataDirState {
        fn try_new() -> Result<Self, IoError> {
            let cwd = std::env::current_dir()?.join("files");
            Ok(Self { dir: Arc::new(cwd) })
        }

        fn path(&self) -> &Path {
            self.dir.as_path()
        }
    }

    tide::log::start();

    let tide_secret: &[u8] = &(0..64).map(|_| rand::random::<u8>()).collect::<Vec<u8>>();

    let mut app = tide::with_state(DataDirState::try_new()?);
    app.with(tide::log::LogMiddleware::new());

    // let cwd = std::env::current_dir().unwrap();
    // let file = cwd.as_path().join(".auth");
    // if file.exists() {
    //     let memory_store = stores::from_hjson_file(file.as_path())?;
    //     let authenticator = BasicAuthenticator::new(memory_store);
    //     authenticator.init(&mut app);
    // }

    app.with(tide::sessions::SessionMiddleware::new(
        tide::sessions::MemoryStore::new(),
        tide_secret,
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

    app.at("/").serve_dir("../wasm/web/")?;
    app.at("/file").serve_dir("files/")?;

    app.at("upload/file")
        .post(|mut req: Request<DataDirState>| async move {
            let mut file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
            //let mut file_name = "xyz".to_string();
            let content_type = req.content_type().unwrap();
            let boundary = content_type.param("boundary").unwrap();
            let req_bytes = std::io::Cursor::new(req.body_bytes().await?); //Req::from_request(req).await?;
            let mut multipart_req = Multipart::with_body(req_bytes, boundary.as_str());
            let mut file_bytes = vec![];
            multipart_req.foreach_entry(|mut field| {
                let name = &field.headers.name;
                if !name.eq(&"file".into()) {
                    return;
                }

                let mime = match field.headers.content_type {
                    Some(value) => value,
                    None => return,
                };
                match (mime.type_(), mime.subtype()) {
                    (mime::IMAGE, mime::JPEG) => {
                        file_name.push_str(".jpg");
                    }
                    (mime::IMAGE, mime::PNG) => {
                        file_name.push_str(".png");
                    }
                    (mime::IMAGE, mime::SVG) => {
                        file_name.push_str(".svg");
                    }
                    (mime::IMAGE, mime::BMP) => {
                        file_name.push_str(".bmp");
                    }
                    _ => {}
                }
                let _res = field.data.read_to_end(&mut file_bytes);
            })?;

            if file_bytes.is_empty() {
                return Ok(json!({"error": "Unable to read file. Please try again." }));
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
