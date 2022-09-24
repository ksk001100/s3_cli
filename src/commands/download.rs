use rusoto_core::Region;
use rusoto_s3::{GetObjectRequest, ListObjectsRequest, S3Client, S3};
use seahorse::{Command, Context, Flag, FlagType};
use std::io::{stdout, Write};
use std::path::Path;
use tokio::fs::{create_dir, File};
use tokio::io;
use tokio::runtime::Runtime;

pub(crate) fn command() -> Command {
    Command::new("downlaod")
        .usage("Download the S3 bucket")
        .alias("dl")
        .alias("d")
        .flag(
            Flag::new("output", FlagType::String)
                .alias("o")
                .description("Specify output directory"),
        )
        .action(action)
}

fn action(c: &Context) {
    let mut runtime = Runtime::new().unwrap();
    let client = S3Client::new(Region::ApNortheast1);
    let bucket = &c.args[0];

    runtime.block_on(async {
        let bucket = &bucket;
        let output_dir = match c.string_flag("output") {
            Ok(o) => o,
            Err(_) => (*bucket).clone(),
        };

        if !Path::new(&output_dir).exists() {
            create_dir(&output_dir).await.unwrap();
        }

        println!("[Bucket] : {}", bucket);
        println!("[Output dir] : {}", output_dir);

        let req = ListObjectsRequest {
            bucket: bucket.to_string(),
            ..Default::default()
        };

        if let Ok(out) = client.list_objects(req).await {
            let objects = out.contents.unwrap();
            let keys = objects.iter().flat_map(|obj| &obj.key).collect::<Vec<_>>();
            let total_count = keys.len();

            for (i, key) in keys.iter().enumerate() {
                let req = GetObjectRequest {
                    bucket: bucket.to_string(),
                    key: (*key).clone(),
                    ..Default::default()
                };
                match client.get_object(req).await {
                    Ok(mut out) => {
                        let body = out.body.take().unwrap();
                        let mut body = body.into_async_read();
                        let filename = key.split("/").last().unwrap();
                        let path = format!("{}/{}", output_dir, filename);
                        let per = (i + 1) as f32 / total_count as f32 * 100.0;
                        if let Ok(mut file) = File::create(path).await {
                            io::copy(&mut body, &mut file).await.unwrap();
                            print!("\r[Downloading] : {}/{}({:.1}%)", i + 1, total_count, per);
                            stdout().flush().unwrap();
                        }
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
    });
}
