

//! Macro helpers for decoupling a struct field into a shared, interior-mutable wrapper.

#[macro_export]
macro_rules! decouple_struct_field {
    (
        $(#[$meta:meta])* 
        $vis:vis struct $name:ident {
            $field:ident : $ty:ty
        }
    ) => {
        $(#[$meta])*
        $vis struct $name {
            $field: ::std::rc::Rc<::std::cell::RefCell<$ty>>,
        }

        impl $name {
            pub fn new(value: $ty) -> Self {
                Self {
                    $field: ::std::rc::Rc::new(::std::cell::RefCell::new(value)),
                }
            }

            pub fn $field(&self) -> ::std::cell::Ref<'_, $ty> {
                self.$field.borrow()
            }

            pub fn set_$field(&self, value: $ty) {
                *self.$field.borrow_mut() = value;
            }

            pub fn share(&self) -> Self {
                Self {
                    $field: self.$field.clone(),
                }
            }
        }
    };
}
