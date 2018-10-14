
pub struct Message {
    pub title: &'static str,
    pub subtitle: &'static str
}

const WELCOME_MESSAGE: Message = Message {
    title: "Welcome to rockets!",
    subtitle: "Press Space to continue"
};

const GAMEOVER_MESSAGE: Message = Message {
    title: "Game Over!",
    subtitle: "Press Space to continue"
};