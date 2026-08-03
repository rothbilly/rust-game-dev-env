
# Semana junio 1

Se hizo una propuesta de arquitectura general de la aplicación.

![](arch_01.png)

La app base es una lista con los juego, aplicaciones o widgets activos.

```Rust
/// # The App Base Module
#[derive(Default)]
pub struct App
{
    sub_apps: Vec<Box<dyn GameLoop>>,
}
```

Para poder estar en la lista, se debe cumplir que cada juego implemente el trait `GameLoop` 

```rust
pub trait  GameLoop{
    fn _ready(&mut self){}
    fn _process_input(&mut self){}
    fn _update(&mut self, _delta: f32){}
    fn _render(&self){}
}
```

extra: hay traits que obligan a implementar algunos campos comunes en juegos, como pausable o que exista un paren o owner.

```rust
/// Enforce the struct to have a composition base_app field; 
pub trait IParentable{
    fn get_base_app(&self) -> Option<&Box<dyn IParentable>>;
}

pub trait IPausable {
    fn is_active(&self) -> bool;
    fn set_active(&mut self, _value: bool);
}
```

### Screenshot del editor 

![](main_02.png)

Ahora de base hay una app base que no tiene nada y actua como punto de partida, de todas formas actualmente se pueden cambiar los juegos en tiempo de compilacion.
## Rust

### Mutabilidad interior

Hay situaciones en las que hay que tener multiples referencias a un mismo objeto, mutarlo y luego liberarlo.
![](arch_02.png)

`RefCell`, permite tener multiples dueños y checkear ownership en runtime con `borrow()` o `borrow_mut()`

Si necesitas varios **owners** para tu `RefCell` sin tamaño asignada en el **heap**, deberías sustituir `Box` por **`Rc`** (con recuento de referencias) o **`Arc`** (con recuento atómico de referencias).

> ⚠️ **Note:** `RefCell` is strictly single-threaded. If you use `Arc` for multi-threading, you must pair it with a `Mutex` or `RwLock` instead of a `RefCell`.

```rust
use std::rc::Rc;
use std::cell::RefCell;

// Shared, heap-allocated, interior-mutable trait object
let shared_node: Rc<RefCell<dyn CustomWidget>> = Rc::new(RefCell::new(Button));

// Clone the pointer (creates a new owner pointing to the same heap memory)
let cloned_node = Rc::clone(&shared_node);
```

### Sistema de Eventos

Evaluar si **macroquad** tiene algún sistema de eventos internos o implementar uno propio o de librerías externas.

```rust
struct EventDispatcher {
    // We store closures that take a reference to the event
    listeners: Vec<Box<dyn Fn(&Event)>>,
}
```

Se puede crear un sistema que guarda **Closures** con una lista de metodos `ec<Box<dyn Fn(&Event)>>` y luego invocarlos.

```rust
impl EventDispatcher {
    fn new() -> Self {
        Self { listeners: Vec::new() }
    }
    // Register a new listener
    fn subscribe(&mut self, listener: impl Fn(&Event) + 'static) {
        self.listeners.push(Box::new(listener));
    }
    // Dispatch an event to all listeners
    fn dispatch(&self, event: &Event) {
        for listener in &self.listeners {
            listener(event);
        }
    }
}
```


### Trabajos futuros

- Hacer un juego de prueba con tamaño de ventana 256 x 256 pixeles
- Sistema de eventos con delegates.
- Crear algún macro, idealmente partir con inlines macros.


# Semana 2

Diseño del sistema de escenas y nodos del motor

**Opcion 1**: Comparten la misma raiz, pero el juego esta detro de una base `editor`

![](arch_03.png)

**Opcion 2**:  Editor y Game en arboles separados

![](arch_04.png)

**Opcion 3** : Editor y game comparten el arbol

![](arch_05.png)
 
## 1. When to use Mutex vs. RwLock

`Mutex` or `RwLock` lets those threads safely _modify_ the data one at a time.

| Feature         | `Arc<Mutex<T>>`                                                      | `Arc<RwLock<T>>`                                                 |
| --------------- | -------------------------------------------------------------------- | ---------------------------------------------------------------- |
| **Best For**    | Frequent writes, or a mix of reads and writes.                       | Frequent reads, rare writes.                                     |
| **Concurrency** | Only **one** thread can access the data at a time (reads or writes). | **Multiple** readers can access concurrently, OR **one** writer. |
| **Overhead**    | Lower locking overhead.                                              | Higher overhead (needs to track reader counts).                  |
### Using Mutex

The `Mutex` class in Rust provides mutual exclusion, allowing only one thread to access the data at a time. It is a fundamental synchronization primitive useful for protecting shared data from being simultaneously accessed by multiple threads.

The `Mutex` class can be used to create a thread-safe wrapper around a piece of data. Here is a basic example:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// Output: Result: 10
```

### Using RwLock

The `RwLock` class in Rust provides a reader-writer lock, which allows multiple readers or a single writer to access the data at a time. It is useful for scenarios where read-heavy workloads need concurrent access, but write operations require exclusive access.

The `RwLock` class can be used to create a thread-safe wrapper around a piece of data, allowing for multiple readers or a single writer. Here is a basic example:

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let lock = Arc::new(RwLock::new(5));
    let mut handles = vec![];

    for _ in 0..10 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let read_guard = lock.read().unwrap();
            println!("Read value: {}", *read_guard);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
// Output (order may vary):
// Read value: 5
// Read value: 5
// ...
```
#### Considerations

- `RwLock` can cause deadlocks if not used carefully. Ensure that locks are released appropriately.
- A `RwLock` can become poisoned if a thread panics while holding the lock. Accessing a poisoned `RwLock` will result in a `PoisonError`.
- Use `Arc` (Atomic Reference Counting) to share a `RwLock` across multiple threads.

# Semana 3 Junio

## Rust reflection metadata y linkers

| Crate     | Function                             | Example usage              |
| --------- | ------------------------------------ | -------------------------- |
| Linkme    | https://github.com/dtolnay/linkme    | #[distributed_slice]       |
| inventory | https://github.com/dtolnay/inventory | inventory::collect!(Flag); |
| ctor      |                                      |                            |

# Semana 2 Julio

Algunas idea de features para esta semana
- Seleccionar juego por default en los settings para facilitar el profiling
- permitir a los juegos dar un panic error pero dejar que el editor siga funcionando.
- Intergrar un crate de profiling `Puffin` es buen candidato.

# Semana julio 3

Features ideas
- Mejorar el sistema de settings. para que tome las secciones en formato toml
- ventana de profiling
	- grafico de frame_time
	- draw calls
	- uso de memoria de cada juego
- extender funciones de dibujado
- Serializar la configuracion de las ventanas de editor en .app/editor_cache.json
- Inspector de assets: texturas, shaders,meshes 
- exit dialog on esc, double esc to exit and save