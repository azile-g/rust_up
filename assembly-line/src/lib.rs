// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let speed = speed as f64;
    let rate = match speed {
        s if s <= 4.0 => 1.0,
        s if 5.0 <= s && s <= 8.0 => 0.9, 
        s if s == 9.0 || s == 10.0 => 0.77,
        _ => 1.0
    };
    let prod_rate = rate*speed*221.0 as f64;
    return prod_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let prod_rate = production_rate_per_hour(speed);
    let items_num = (prod_rate/60.0) as u32; 
    return items_num
}
