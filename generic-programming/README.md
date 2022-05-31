## Rust - Generic arguments three use cases

* Deferred binding of data structures with generic parameters.

```rust
// define Service data structure with generic argument Store(default value is Memtable)
pub struct Service<Store = Memtable> {
    inner: Arc<ServiceInner<Store>>
}

impl<Store> Service<Store> {
    pub fn new(store: Store) -> Self { ... }
}

impl<Store: Storage> Service<Store> {
    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse { ... }
}

// in the generic function can use impl Storage or <Store: Storage> to restrict the argument
pub fn dispatch(cmd: CommandRequest, store: &impl Storage) -> CommandResponse { ... }

pub fn dispatch<Store: Storage>(cmd: CommandRequest, store: &Store) -> CommandResponse { ... }

```


* Use generic parameters and PhantomData to declare types that are not directly used in the data structure, but need to be used in the implementation process.

* Use generic parameters to allow the same data structure to have different implementations of the same trait.