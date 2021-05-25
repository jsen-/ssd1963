use packed_struct::derive::PackedStruct;

#[derive(PackedStruct, Debug, PartialEq)]
#[packed_struct(bit_numbering = "msb0", endian = "msb")]
pub struct SetColumnAddress {
    pub start: u16,
    pub end: u16,
}
