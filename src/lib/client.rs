use crate::lib::dispatcher::EventDispatcher;

#[derive(Clone, Debug)]
pub struct Client {
    xoxc_token: String,
    xoxd_token: String,
    event_dispatcher: EventDispatcher
}

impl Client {
    fn get_xoxc(&self) -> String {
        self.xoxc_token.clone()
    }

    fn get_xoxd(&self) -> String {
        self.xoxd_token.clone()
    }

    fn new(xoxc: String, xoxd: String) -> Self {
        Self {
            xoxc_token: xoxc,
            xoxd_token: xoxd,
            event_dispatcher: EventDispatcher::new(4096)
        }
    }

    async fn run(&self) -> ! {
        todo!()
    }
}