use axum::{response::IntoResponse, routing::get, Router};
use bb8_redis::{bb8, RedisConnectionManager};
use redis::AsyncCommands;
use std::{env, sync::Arc};
use token::TokenClient;
use tokio::{net::TcpListener, sync::RwLock};
use tokio_cron_scheduler::{Job, JobScheduler};

mod token;

type RedisPool = bb8::Pool<RedisConnectionManager>;

#[derive(Clone)]
struct SharedState {
    token: Arc<RwLock<String>>,
    redis_pool: Arc<RedisPool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let redis_pool = create_redis_pool().await?;

    let shared_state = SharedState {
        token: Arc::new(RwLock::new(String::new())),
        redis_pool: Arc::new(redis_pool.clone()),
    };

    // Fetch and cache the token immediately
    let token_client = TokenClient::new();
    fetch_and_cache_token(&shared_state, &token_client).await?;

    let routes = Router::new()
        .route("/token", get(get_cached_token))
        .route("/health", get(|| async { "OK" }))
        .with_state(shared_state.clone());

    let job_scheduler = JobScheduler::new().await?;
    let job = Job::new_async("0 */30 * * * *", move |_uuid, _l| {
        let shared_state = shared_state.clone();
        let token_client = TokenClient::new();
        Box::pin(async move {
            if let Err(e) = fetch_and_cache_token(&shared_state, &token_client).await {
                eprintln!("Error fetching and caching token: {:?}", e);
            }
        })
    })?;

    job_scheduler.add(job).await?;
    job_scheduler.start().await?;

    let listener = TcpListener::bind("0.0.0.0:6300").await?;
    println!("Token Cacher listening on 6300");

    axum::serve(listener, routes.into_make_service()).await?;

    Ok(())
}

async fn create_redis_pool() -> Result<RedisPool, Box<dyn std::error::Error>> {
    let redis_url = env::var("TOKEN_CACHE_URL").expect("TOKEN_CACHE_URL must be set");
    let manager = RedisConnectionManager::new(redis_url)?;
    Ok(bb8::Pool::builder().build(manager).await?)
}

async fn get_cached_token(state: axum::extract::State<SharedState>) -> impl IntoResponse {
    println!("Get token request made");
    state.token.read().await.clone()
}

async fn fetch_and_cache_token(
    shared_state: &SharedState,
    token_client: &TokenClient,
) -> Result<(), Box<dyn std::error::Error>> {
    let token = token_client.fetch_token().await?;
    println!("Got new token");

    let mut redis_conn = shared_state.redis_pool.get().await?;
    redis_conn
        .set::<_, _, ()>("access_token", token.as_str())
        .await?;
    redis_conn.expire("access_token", 3600).await?;

    *shared_state.token.write().await = token;
    Ok(())
}
