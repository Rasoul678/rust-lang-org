use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.add_text(", again!");

    // post.reject();

    post.approve();
    assert_eq!("I ate a salad for lunch today, again!", post.content());

    // If rejected
    // assert_eq!("", post.content());
}
