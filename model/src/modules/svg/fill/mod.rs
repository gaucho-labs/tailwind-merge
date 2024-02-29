use super::*;

#[doc=include_str!("readme.md")]
#[derive(Debug, Clone)]
pub struct TailwindFillColor {
    color: TailwindColor,
}

crate::macros::sealed::color_instance!(TailwindFillColor);

impl Display for TailwindFillColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "fill-{}", self.color)
    }
}

impl TailwindInstance for TailwindFillColor {
    fn collision_id(&self) -> String {
        "fill-color".into()
    }

    fn get_collisions(&self) -> Vec<String> {
        vec![self.collision_id()]
    }
}