pub mod keyboard {

    pub struct Key {
        pub x: i32,
        pub y: i32
    }

    /* 
    * Grid des Keyboards. (eigentlich const, will rust aber nicht).
    */
    pub fn init() -> Vec<Key> {
        let mut grid = Vec::new();

        // ranges 0 - 22
        for index in 1..22 {
            grid.push(Key { x: index, y: 0 });
        }

        // 24 - 46
        for index in 24..46 {
            grid.push(Key { x: index, y: 1 });
        }

        // 48 - 70
        for index in 48..70 {
            grid.push(Key { x: index, y: 2 });
        }

        // 72 - 94

        // 96 - 118

        // 120 - 142

        // 144 - 166

        return grid;
    }
}
