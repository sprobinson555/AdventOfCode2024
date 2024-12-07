pub struct Position {
    pub y :usize,
    pub x :usize,
    y_max : Option<usize>,
    x_max : Option<usize>,
}

impl Position {
    pub fn apply_velocity(&mut self, vel :&Velocity) {
        let new_x = (self.x as isize) + vel.x;
        let new_y = (self.y as isize) + vel.y;

        let mut can_apply = true;
        
        if let Some(y_max) = self.y_max {
            if new_y >= y_max as isize {
                can_apply = false;
            }
        }
        if let Some(x_max) = self.x_max {
            if new_x >= x_max as isize {
                can_apply = false;
            }
        }
        if new_y < 0 || new_x < 0 {
            can_apply = false;
        }
        
        if can_apply {
            self.y = new_y as usize;
            self.x = new_x as usize;
        }
    }


    pub fn new_default() -> Self {
        let pos = Position {
            y : 0,
            x : 0,
            y_max : None,
            x_max : None,
        };
        return pos
    }
    pub fn new_without_max(y :usize, x :usize) -> Self{
        let pos = Position {
            y : y,
            x : x,
            y_max : None,
            x_max : None,
        };
        return pos;
    }
    pub fn new(y :usize, x :usize, y_max: Option<usize>, x_max: Option <usize>) -> Self {
        let pos = Position {
            y : y,
            x : x,
            y_max : y_max,
            x_max : x_max,
        };
        return pos;
    }
}

pub struct Velocity {
    pub y :isize,
    pub x :isize,
}

#[non_exhaustive]
pub struct Unit_Velocity;

impl Unit_Velocity {
    pub const UP :Velocity = Velocity {x:0, y:-1};
    pub const RIGHT :Velocity = Velocity {x:1, y:0};
    pub const DOWN :Velocity = Velocity {x:0, y:1};
    pub const LEFT :Velocity = Velocity {x:-1, y:0};
    pub const UP_RIGHT :Velocity = Velocity {x:1, y:-1};
    pub const DOWN_RIGHT :Velocity = Velocity {x:1, y:1};
    pub const DOWN_LEFT :Velocity = Velocity {x:-1, y:1};
    pub const UP_LEFT :Velocity = Velocity {x:-1, y:-1};
}