use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_time() -> u128 {
    // Returns the time in Unix Time (the number of milliseconds since 1970)
    let time: u128 = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis();

    //Return the current time
    time
}

pub fn out_of_bounds(x: f32, y: f32, w: f32, h: f32, world_width: f32, world_height: f32,) -> bool {
    //Basically, if the rectangle is out of bounds, it returns true, if not it'll return false
    {
        x + w >= world_width ||
        x <= 0.0 ||
        y +h >= world_height ||
        y <= 0.0
    }

}
