use tide::{Middleware, Request, Next};

pub struct DebugRefresh;


#[tide::utils::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for DebugRefresh {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> tide::Result {
        let mut res = next.run(req).await;
        if cfg!(debug_assertions) {
            res.insert_header("Cache-Control", "no-cache, no-store, must-revalidate");
            res.insert_header("Pragma", "no-cache");
            res.insert_header("Expires", "0");
        }

        Ok(res)
    }
}
