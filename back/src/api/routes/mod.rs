use warp::filters::BoxedFilter;
use warp::Filter;

pub async fn hello() -> BoxedFilter<()> {
    warp::path("hello")
        .boxed()
}

pub async fn user() -> BoxedFilter<(String,)> {
    warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        .boxed()

}
