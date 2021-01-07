pub struct date_time{
    year: u16,
    month_weekday_day: u16,
    hour_minute: u16,
    millisecond: u16,
}


pub struct power_meter{
    address: String,
    date_and_time: date_time,
    energy:u16, 
    phase_1: f32,
    phase_2: f32,
    phase_3: f32,
    l1_l2_voltage:f32,
    l2_l3_voltage:f32,
    l3_l1_voltage:f32,
    l1_ln_voltage:f32,
    l2_ln_voltage:f32,
    l3_ln_voltage:f32,
    p1_apparent_power:f32,
    p2_apparent_power:f32,
    p3_apparent_power:f32,
    p1_reactive_power:f32,
    p2_reactive_power:f32,
    p3_reactive_power:f32,
    p1_active_power:f32,
    p2_active_power:f32,
    p3_active_power:f32,
}