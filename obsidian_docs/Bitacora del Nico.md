
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

For a single-threaded application, the ultimate combo for an unsized, shared, heap-allocated type is:

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