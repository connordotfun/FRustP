mod lib;

struct Node<T, V>{
    function: Fn(T) -> V, // Think about if T is a tuple of signals
    input: Signal<T>,
    output: Signal<V>,
}


