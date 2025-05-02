use futures::stream::{self, Stream, StreamExt};
use reqwest::{Client, ClientBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::error::Error;
use std::fmt;
use std::pin::Pin;

#[derive(Debug)]
pub struct BadRequestException {
    message: String,
}

impl BadRequestException {
    pub fn new(message: &str) -> Self {
        BadRequestException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for BadRequestException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad Request: {}", self.message)
    }
}

impl Error for BadRequestException {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostOpts {
    pub url: String,
    pub username: String,
    pub password: String,
}

pub struct HttpTransportDriver;

impl HttpTransportDriver {
    pub fn new() -> Self {
        HttpTransportDriver {}
    }

    async fn get_chunks<'a>(
        &'a self,
        client_id: &'a str,
        opts: &'a PostOpts,
    ) -> Pin<Box<dyn Stream<Item = Result<Vec<Value>, Box<dyn Error + Send + Sync>>> + 'a>> {
        let debug = std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

        Box::pin(stream::unfold(
            (0, 0, client_id.to_string(), opts.clone()),
            move |(mut start, mut items, client_id, opts)| async move {
                let url = opts.url.clone();
                let username = opts.username.clone();
                let password = opts.password.clone();

                if username.is_empty() || password.is_empty() {
                    return Some((
                        Err(
                            Box::new(BadRequestException::new("Missing username or password"))
                                as Box<dyn Error + Send + Sync>,
                        ),
                        (start, items, client_id, opts),
                    ));
                }

                let client = match Client::builder().build() {
                    Ok(client) => client,
                    Err(e) => {
                        return Some((
                            Err(Box::new(e) as Box<dyn Error + Send + Sync>),
                            (start, items, client_id, opts),
                        ));
                    }
                };

                let sync_endpoint = format!("{}/app/sync/chunk", url);

                let response = match client
                    .get(&sync_endpoint)
                    .basic_auth(username.clone(), Some(password.clone()))
                    .query(&[("client_id", &client_id), ("start", &start.to_string())])
                    .send()
                    .await
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        return Some((
                            Err(Box::new(e) as Box<dyn Error + Send + Sync>),
                            (start, items, client_id, opts),
                        ));
                    }
                };

                if response.status() != StatusCode::OK {
                    return Some((
                        Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::Other,
                            format!("API error: {}", response.status()),
                        )) as Box<dyn Error + Send + Sync>),
                        (start, items, client_id, opts),
                    ));
                }

                let data = match response.text().await {
                    Ok(text) => match serde_json::from_str::<Value>(&text) {
                        Ok(data) => data,
                        Err(e) => {
                            return Some((
                                Err(Box::new(e) as Box<dyn Error + Send + Sync>),
                                (start, items, client_id, opts),
                            ));
                        }
                    },
                    Err(e) => {
                        return Some((
                            Err(Box::new(e) as Box<dyn Error + Send + Sync>),
                            (start, items, client_id, opts),
                        ));
                    }
                };

                let messages = data
                    .get("data")
                    .and_then(|d| d.get("messages"))
                    .and_then(|m| m.as_array())
                    .cloned()
                    .unwrap_or_default();

                let size = data
                    .get("data")
                    .and_then(|d| d.get("size"))
                    .and_then(|s| s.as_u64())
                    .unwrap_or(0);

                items += messages.len();

                if debug {
                    println!(
                        "Got Chunk of client_id{} size:{}/{}",
                        client_id, items, size
                    );
                }

                if messages.is_empty() {
                    if debug {
                        println!("Got all chunks of client_id{} - deleting", client_id);
                    }

                    // Delete the chunks
                    let _ = client
                        .delete(&sync_endpoint)
                        .basic_auth(username, Some(password))
                        .query(&[("client_id", &client_id)])
                        .send()
                        .await;

                    if debug {
                        println!("Got all chunks of client_id{} - deleted", client_id);
                    }

                    return None;
                }

                start += messages.len();

                Some((Ok(messages), (start, items, client_id, opts)))
            },
        ))
    }

    // pub async fn post(&self, data: Value, opts: &PostOpts) -> Result<Value, Box<dyn Error>> {
    //     let debug = std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true";

    //     if debug {
    //         println!("Posting to {}", serde_json::to_string_pretty(opts).unwrap());
    //     }

    //     let sync_endpoint = &opts.url;
    //     let username = &opts.username;
    //     let password = &opts.password;

    //     if username.is_empty() || password.is_empty() {
    //         return Err(Box::new(BadRequestException::new("Missing username or password")));
    //     }

    //     let client = ClientBuilder::new()
    //         .build()?;

    //         let data_string = serde_json::to_string(&data)?;

    //         let response = match client
    //         .post(&format!("{}/app/sync", sync_endpoint))
    //         .basic_auth(username, Some(password))
    //         .header("Content-Type", "application/json")
    //         .body(data_string)
    //         .send()
    //         .await {
    //         Ok(resp) => resp,
    //         Err(e) => {
    //             return Err(Box::new(e));
    //         }
    //     };

    //     if response.status() != StatusCode::OK {
    //         return Err(Box::new(std::io::Error::new(
    //             std::io::ErrorKind::Other,
    //             format!("API error: {}", response.status()),
    //         )));
    //     }

    //     let mut result = match response.text().await {
    //         Ok(text) => {
    //             match serde_json::from_str::<Value>(&text) {
    //                 Ok(data) => {
    //                     if let Some(data_obj) = data.get("data") {
    //                         data_obj.clone()
    //                     } else {
    //                         json!({
    //                             "messages": []
    //                         })
    //                     }
    //                 },
    //                 Err(e) => {
    //                     return Err(Box::new(e));
    //                 }
    //             }
    //         },
    //         Err(e) => {
    //             return Err(Box::new(e));
    //         }
    //     };

    //     // Check if incomplete and chunks are needed
    //     if result.get("incomplete").and_then(|v| v.as_bool()).unwrap_or(false) {
    //         if debug {
    //             println!("Chunk transfer requested");
    //         }

    //         let client_id = data.get("client_id")
    //             .and_then(|c| c.as_str())
    //             .unwrap_or("")
    //             .to_string();

    //         let mut messages = result.get("messages")
    //             .and_then(|m| m.as_array())
    //             .cloned()
    //             .unwrap_or_default();

    //             let chunks_stream = self.get_chunks(&client_id, opts).await;
    //             let mut chunks_stream = chunks_stream.boxed_local();

    //         while let Some(chunk_result) = chunks_stream.next().await {
    //             match chunk_result {
    //                 Ok(chunk) => {
    //                     // Transform chunk messages
    //                     let transformed_messages: Vec<Value> = chunk.iter()
    //                         .filter_map(|m| m.get("message").cloned())
    //                         .collect();

    //                     messages.extend(transformed_messages);
    //                 },
    //                 Err(e) => {
    //                     return Err(e);
    //                 }
    //             }
    //         }

    //         if debug {
    //             println!("Chunk transfer done");
    //         }

    //         // Update the messages in the result
    //         result["messages"] = json!(messages);
    //     }

    //     Ok(result)
    // }

    pub async fn post(&self, data: Value, opts: &PostOpts) -> Result<Value, Box<dyn Error>> {
        log::debug!("Posting to {}", serde_json::to_string_pretty(opts).unwrap());

        let sync_endpoint = &opts.url;
        let username = &opts.username;
        let password = &opts.password;

        if username.is_empty() || password.is_empty() {
            return Err(Box::new(BadRequestException::new(
                "Missing username or password",
            )));
        }

        let client = ClientBuilder::new().build()?;

        //log the data here
        //change merkle in data to string instead of json
        //print data here
        let mut data_to_send = data.clone();
        if let Some(merkle) = data.get("merkle") {
            if merkle.is_object() {
                // Convert the merkle object to a string representation
                let merkle_str = serde_json::to_string(merkle).unwrap_or_default();
                data_to_send["merkle"] = json!(merkle_str);
            }
            // If it's already a string, no need to modify
        }

        let response = match client
            .post(&format!("{}/app/sync", sync_endpoint))
            .basic_auth(username, Some(password))
            .header("Content-Type", "application/json")
            .json(&data_to_send)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        if response.status() != StatusCode::OK {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("API error: {}", response.status()),
            )));
        }

        let mut result = match response.text().await {
            Ok(text) => match serde_json::from_str::<Value>(&text) {
                Ok(data) => {
                    if let Some(data_obj) = data.get("data") {
                        data_obj.clone()
                    } else {
                        json!({
                            "messages": []
                        })
                    }
                }
                Err(e) => {
                    return Err(Box::new(e));
                }
            },
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        // Check if incomplete and chunks are needed
        if result
            .get("incomplete")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            log::debug!("Chunk transfer requested");

            let client_id = data
                .get("client_id")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();

            let mut messages = result
                .get("messages")
                .and_then(|m| m.as_array())
                .cloned()
                .unwrap_or_default();

            let mut start = 0;
            let mut items = 0;

            loop {
                // Fetch a chunk
                let chunk_response = match client
                    .get(&format!("{}/app/sync/chunk", sync_endpoint))
                    .basic_auth(username, Some(password))
                    .query(&[("client_id", &client_id), ("start", &start.to_string())])
                    .send()
                    .await
                {
                    Ok(resp) => resp,
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                };

                if chunk_response.status() != StatusCode::OK {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("API error: {}", chunk_response.status()),
                    )));
                }

                let chunk_data = match chunk_response.text().await {
                    Ok(text) => match serde_json::from_str::<Value>(&text) {
                        Ok(data) => data,
                        Err(e) => {
                            return Err(Box::new(e));
                        }
                    },
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                };

                let chunk_messages = chunk_data
                    .get("data")
                    .and_then(|d| d.get("messages"))
                    .and_then(|m| m.as_array())
                    .cloned()
                    .unwrap_or_default();

                let size = chunk_data
                    .get("data")
                    .and_then(|d| d.get("size"))
                    .and_then(|s| s.as_u64())
                    .unwrap_or(0);

                items += chunk_messages.len();

                log::debug!(
                    "Got Chunk of client_id{} size:{}/{}",
                    client_id,
                    items,
                    size
                );

                if chunk_messages.is_empty() {
                    log::debug!("Got all chunks of client_id{} - deleting", client_id);

                    // Delete the chunks
                    let _ = client
                        .delete(&format!("{}/app/sync/chunk", sync_endpoint))
                        .basic_auth(username, Some(password))
                        .query(&[("client_id", &client_id)])
                        .send()
                        .await;

                    log::debug!("Got all chunks of client_id{} - deleted", client_id);

                    break;
                }

                // Transform chunk messages
                let transformed_messages: Vec<Value> = chunk_messages
                    .iter()
                    .filter_map(|m| m.get("message").cloned())
                    .collect();

                messages.extend(transformed_messages);

                start += chunk_messages.len();
            }

            log::debug!("Chunk transfer done");

            // Update the messages in the result
            result["messages"] = json!(messages);
        }

        Ok(result)
    }
}
