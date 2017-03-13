//Author: Darryl Beckham

extern crate rusoto;

use std::default::Default;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::ListObjectsV2Request;
use rusoto::s3::S3Client;
use rusoto::default_tls_client;

fn list_bucket_objects(){
    
    let bare_s3 = S3Client::new(
        default_tls_client().unwrap(),
        DefaultCredentialsProvider::new().unwrap(),
        Region::UsEast1
    );

    let mut list_request = ListObjectsV2Request::default();
    list_request.bucket = "dabeckham.first.bucket".to_string();
    list_request.prefix = Some("list1000".to_string());
    let mut continuation_token = "";

    loop {
        if continuation_token != ""{
            list_request.continuation_token = Some(continuation_token.to_string());
        }

        let result = bare_s3.list_objects_v2(&list_request).unwrap();

        //I think result goes out of scope after the first iteration of the loop
        //result needs to update on each iteration
        for obj in &result.contents.expect("Contents not found!"){
            println!("{:?}", obj.key.clone().unwrap());
        }

        if (result).is_truncated.is_none(){
            break;
        }

        continuation_token = &result.continuation_token.expect("No token!").as_ref();
    }
}

fn main() {
    println!("Listing objects!");
    list_bucket_objects();
}
