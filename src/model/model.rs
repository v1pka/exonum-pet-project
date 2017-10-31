


encoding_struct! {
    struct Contractor {
        const SIZE = 25;
        field id:                      &[u8]       [00 => 08]
        field node_id:                 &[u8]       [08 => 16]
        field status:                  u8          [16 => 17]
        field serialized_contractor:   &str        [17 => 25]
    }
}
