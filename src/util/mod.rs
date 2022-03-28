use druid::Screen;

pub fn get_position(width: f64, height: f64) -> (f64, f64) {
    let mut x = 0.0;
    let mut y = 0.0;
    for m in Screen::get_monitors().iter() {
        if m.is_primary() {
            let rect = m.virtual_work_rect();
            x = rect.x1 / 2.0 - width / 2.0 + rect.x0;
            if x < 0.0 {
                x = 0.0
            }
            y = rect.y1 / 2.0 - height / 2.0 + rect.y0;
            if y < 0.0 {
                y = 0.0
            }
            break;
        }
    }
    (x, y)
}
