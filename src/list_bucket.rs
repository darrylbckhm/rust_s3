//Author: Darryl Beckham

extern crate rusoto;

use std::default::Default;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::s3::ListObjectsRequest;
use rusoto::s3::S3Client;
use rusoto::default_tls_client;

fn list_bucket_objects(){
    
    let bare_s3 = S3Client::new(
        default_tls_client().unwrap(),
        DefaultCredentialsProvider::new().unwrap(),
        Region::UsEast1
    );

    let mut list_request = ListObjectsRequest::default();
    list_request.bucket = "dabeckham.first.bucket".to_string();
    list_request.prefix = Some("list1000".to_string());
    let result = bare_s3.list_objects(&list_request).unwrap();

    //loop {
        for obj in &result.contents.expect("Contents not found!"){
            println!("{:?}", obj.key.clone().unwrap());
        }

        //if (&result).is_truncated.is_none(){
        //    break;
        //}
    //}
}

fn main() {
    println!("Listing objects!");
    list_bucket_objects();
}
