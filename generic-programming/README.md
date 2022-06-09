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

```rust
pub struct Ident<T> {
    inner: u64,
    _tag: PhantomData<T>,
}
```

* Use generic parameters to allow the same data structure to have different implementations of the same trait.

```rust
impl Iterator for Equation<Liner> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u32::MAX {
            return None
        }

        Some(self.current)
    }
}

impl Iterator for Equation<Quadratic> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current >= u16::MAX as u32 {
            return None
        }

        Some(self.current * self.current)
    }
}
```

* Generric function

1. Return value is a genric type

```rust
pub trait Storage {

    //
    fn get_iter() -> Result<Box<dyn Iterator<Item = KvPair>>, KvError>
}

pub fn trait_object_as_return(i: u32) -> Box<dyn Iterator<Item = u32>> {
    Box::new(std::iter::once(i))
}
```

2. Handler complex genric arguments.

```rust
pub fn consume_iterator<F, Iter, T>(mut f: F)
where
    F: FnMut(i32) -> iter,
    Iter: Iterator<Item = T>,
    T: std::fmt::debug,
{
    for item in f(10) {
        println("{:?}", item);
    }
}
``'