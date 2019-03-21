pub struct RAM {
    memory: [u16; 4096]
}

impl RAM {
     pub fn new() -> RAM {
         let mut ram = RAM {
             memory: [0; 4096]
         };
         ram
    }
}