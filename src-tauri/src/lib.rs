use std::{
    collections::VecDeque,
    sync::{Arc, LazyLock},
};

use reqwest::header::ACCEPT;
use tokio::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn get_dad_joke() -> Result<String, String> {
    Ok(JOKES.take().await.unwrap_or("Couldn't get dad joke".into()))
}

struct JokeQueue<const N: usize> {
    client: reqwest::Client,
    queue: Arc<Mutex<VecDeque<String>>>,
}

impl<const N: usize> JokeQueue<N> {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            queue: Arc::new(Mutex::new(VecDeque::with_capacity(N))),
        }
    }

    pub fn from(s: impl Into<[String; N]>) -> Self {
        let client = reqwest::Client::new();
        let queue = Arc::new(Mutex::new(VecDeque::from(s.into())));
        Self { client, queue }
    }

    pub fn from_vec(mut v: Vec<String>) -> Self {
        if v.len() > N {
            v.truncate(N);
        }

        while v.len() < N {
            v.push("Couldn't fetch a dad joke, sorry...".to_string());
        }

        let client = reqwest::Client::new();
        let queue = Arc::new(Mutex::new(VecDeque::from(v)));

        Self { client, queue }
    }

    pub async fn fill(&self, s: impl Into<String>) {
        let s = s.into();
        let mut q = self.queue.lock().await;
        if q.len() >= N {
            q.push_back(s);
            q.pop_front();
        } else {
            q.push_back(s);
        }
    }

    pub async fn take(&self) -> Option<String> {
        let queue = self.queue.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            if let Some(new_joke) = request_dad_joke(&client).await {
                let mut q = queue.lock().await;
                q.push_back(new_joke);
            }
        });

        let mut g = self.queue.lock().await;
        g.pop_front()
    }
}

const URL: &str = "https://icanhazdadjoke.com/";
async fn request_dad_joke(c: &reqwest::Client) -> Option<String> {
    c.get(URL)
        .header(ACCEPT, "text/plain")
        .send()
        .await
        .ok()?
        .text()
        .await
        .ok()
}

static JOKES: LazyLock<JokeQueue<10>> = LazyLock::new(|| JokeQueue::new());

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_| {
            tauri::async_runtime::spawn(async {
                for _ in 0..10 {
                    tokio::spawn(async {
                        let joke = request_dad_joke(&JOKES.client)
                            .await
                            .unwrap_or("Failed to fetch dad joke".into());
                        JOKES.fill(joke).await;
                    });
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_dad_joke])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
