
macro_rules! fields_mut {
    ($self:ident, $($field:ident),* $(,)?) => {
        $(let $field = &mut $self.$field;)*
    };
}