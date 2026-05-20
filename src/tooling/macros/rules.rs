
macro_rules! decoup_fields_mut {
    ($self:ident, $($field:ident),* $(,)?) => {
        $(let $field = &mut $self.$field;)*
    };
}