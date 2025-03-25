use state_oop::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    println!("Попробуем добавить текст не в драфте");
    post.add_text("I ate a salad for lunch today");
    // щикарно
    assert_eq!("", post.content());

    post.reject();
    // попали обратно в драфт
    post.add_text("I ate a salad for lunch today");
    // получилось добавить проверим в конце
    post.request_review();

    post.approve();
    // тут паничка
    post.approve();
    // тут нет -> значит всё чики пенсил
    assert_eq!("I ate a salad for lunch todayI ate a salad for lunch today", post.content());
    // снова добавился текст значит режект сработал как надо
}
