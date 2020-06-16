use rusoto_core::Region;
use rusoto_s3::{S3Client, S3};
use seahorse::{Command, Context};
use tokio::runtime::Runtime;

pub(crate) fn command() -> Command {
    Command::new("list")
        .usage("Show S3 buckets list")
        .alias("ls")
        .action(action)
}

fn action(_c: &Context) {
    let mut runtime = Runtime::new().unwrap();
    let client = S3Client::new(Region::ApNortheast1);

    runtime.block_on(async {
        match client.list_buckets().await {
            Ok(out) => {
                if let Some(buckets) = out.buckets {
                    for bucket in buckets.iter() {
                        println!("{}", bucket.name.as_ref().unwrap());
                    }
                }
            }
            Err(e) => eprintln!("{:?}", e),
        }
    });
}
