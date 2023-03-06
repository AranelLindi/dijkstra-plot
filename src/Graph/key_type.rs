pub mod key_enum {
    #[derive(PartialEq, Eq, Clone, Hash)]
    pub enum KeyType {
        Boolean,
        Int,
        Long,
        Float,
        Double,
        String,
    }
}