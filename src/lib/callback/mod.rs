pub trait Middleware<S, T, A, R>
where A: Callback<S, A, R> {

}

pub trait Callback<S, T, R> : Creatable {

}

pub trait Creatable {
    fn new() -> Self;
}