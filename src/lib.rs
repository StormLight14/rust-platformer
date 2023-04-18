pub mod col {
    use macroquad::prelude::*;

    pub enum RectSide {
        Top,
        Bottom,
        Left,
        Right,
        None,
    }

    pub fn resolve_collision(rect_a: Rect, rect_velocity: Vec2, rect_b: Rect) -> RectSide {
        //RectSide is the side of the PLAYER that is touching the tile

        let velocity = rect_velocity;

        if rect_a.overlaps(&rect_b) {
            if velocity.x < 0f32 {
                return RectSide::Left;
            } else if velocity.x > 0f32 {
                return RectSide::Right;
            }
            if velocity.y < 0f32 {
                return RectSide::Top;
            } else if velocity.y > 0f32 {
                return RectSide::Bottom;
            } else {
                return RectSide::None;
            }
        } else {
            return RectSide::None;
        }
    }
}
