use graph;

struct Signal<T>{
    inner: T
}

// pub trait Functor {
//     fn fmap<T, B> (
//         signal: Signal<T>, 
//         f: Fn(T) -> B )
//         -> Signal<B>;
// }
// todo: sort out a more complicated generic using where clause that disallows Signal<Signal> types.
impl<T> Signal<T>{ 

    fn fmap<B, F: Fn(T) -> B> (signal : &Signal<T>, f: F) -> &Signal<B> {
        
    }

}

impl<u32> Singal<T>

// pub trait Functor<A> {
//     trait<T> inner_object;

//     fn fmap<T, G> (
//         obj : impl inner_object<T>,
//         f : Fn(T) -> G )
//         -> impl inner_object<G>;
// }