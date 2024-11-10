use glam::{
    bool as g_bool,
    u8   as g_u8,  u16 as g_u16, u32 as g_u32, u64 as g_u64,
    i8   as g_i8,  i16 as g_i16, i32 as g_i32, i64 as g_i64,
    f32  as g_f32, f64 as g_f64
};
use toml_edit as toml;

use super::serializable::Serializable;


impl Serializable for g_bool::BVec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x, self.y]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_bool::bvec2(x.as_bool()?, y.as_bool()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_bool::BVec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x, self.y, self.z]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_bool::bvec3(x.as_bool()?, y.as_bool()?, z.as_bool()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_bool::BVec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x, self.y, self.z, self.w]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_bool::bvec4(x.as_bool()?, y.as_bool()?, z.as_bool()?, w.as_bool()?));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_u8::U8Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u8::u8vec2(x.as_integer()? as u8, y.as_integer()? as u8));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u8::U8Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u8::u8vec3(x.as_integer()? as u8, y.as_integer()? as u8, z.as_integer()? as u8));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u8::U8Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u8::u8vec4(x.as_integer()? as u8, y.as_integer()? as u8, z.as_integer()? as u8, w.as_integer()? as u8));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_u16::U16Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u16::u16vec2(x.as_integer()? as u16, y.as_integer()? as u16));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u16::U16Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u16::u16vec3(x.as_integer()? as u16, y.as_integer()? as u16, z.as_integer()? as u16));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u16::U16Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u16::u16vec4(x.as_integer()? as u16, y.as_integer()? as u16, z.as_integer()? as u16, w.as_integer()? as u16));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_u32::UVec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u32::uvec2(x.as_integer()? as u32, y.as_integer()? as u32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u32::UVec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u32::uvec3(x.as_integer()? as u32, y.as_integer()? as u32, z.as_integer()? as u32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u32::UVec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u32::uvec4(x.as_integer()? as u32, y.as_integer()? as u32, z.as_integer()? as u32, w.as_integer()? as u32));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_u64::U64Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u64::u64vec2(x.as_integer()? as u64, y.as_integer()? as u64));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u64::U64Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u64::u64vec3(x.as_integer()? as u64, y.as_integer()? as u64, z.as_integer()? as u64));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_u64::U64Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_u64::u64vec4(x.as_integer()? as u64, y.as_integer()? as u64, z.as_integer()? as u64, w.as_integer()? as u64));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_i8::I8Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i8::i8vec2(x.as_integer()? as i8, y.as_integer()? as i8));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i8::I8Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i8::i8vec3(x.as_integer()? as i8, y.as_integer()? as i8, z.as_integer()? as i8));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i8::I8Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i8::i8vec4(x.as_integer()? as i8, y.as_integer()? as i8, z.as_integer()? as i8, w.as_integer()? as i8));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_i16::I16Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i16::i16vec2(x.as_integer()? as i16, y.as_integer()? as i16));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i16::I16Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i16::i16vec3(x.as_integer()? as i16, y.as_integer()? as i16, z.as_integer()? as i16));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i16::I16Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i16::i16vec4(x.as_integer()? as i16, y.as_integer()? as i16, z.as_integer()? as i16, w.as_integer()? as i16));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_i32::IVec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i32::ivec2(x.as_integer()? as i32, y.as_integer()? as i32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i32::IVec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i32::ivec3(x.as_integer()? as i32, y.as_integer()? as i32, z.as_integer()? as i32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i32::IVec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as i64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i32::ivec4(x.as_integer()? as i32, y.as_integer()? as i32, z.as_integer()? as i32, w.as_integer()? as i32));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_i64::I64Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i64::i64vec2(x.as_integer()?, y.as_integer()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i64::I64Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i64::i64vec3(x.as_integer()?, y.as_integer()?, z.as_integer()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_i64::I64Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_i64::i64vec4(x.as_integer()?, y.as_integer()?, z.as_integer()?, w.as_integer()?));
                    }
                    None
            },
            _ => None
        }
    }
}

impl Serializable for g_f32::Vec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as f64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f32::vec2(x.as_float()? as f32, y.as_float()? as f32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Vec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as f64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f32::vec3(x.as_float()? as f32, y.as_float()? as f32, z.as_float()? as f32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Vec3A {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as f64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f32::vec3a(x.as_float()? as f32, y.as_float()? as f32, z.as_float()? as f32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Vec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array().map(|x| x as f64)))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f32::vec4(x.as_float()? as f32, y.as_float()? as f32, z.as_float()? as f32, w.as_float()? as f32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Mat2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f32::mat2(g_f32::Vec2::from_value(x_axis.clone())?, g_f32::Vec2::from_value(y_axis.clone())?));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Mat3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value(), self.z_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis, z_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f32::mat3(g_f32::Vec3::from_value(x_axis.clone())?, g_f32::Vec3::from_value(y_axis.clone())?, g_f32::Vec3::from_value(z_axis.clone())?));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Mat3A {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value(), self.z_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis, z_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f32::mat3a(g_f32::Vec3A::from_value(x_axis.clone())?, g_f32::Vec3A::from_value(y_axis.clone())?, g_f32::Vec3A::from_value(z_axis.clone())?));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Mat4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value(), self.z_axis.to_value(), self.w_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis, z_axis, w_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f32::mat4(
                            g_f32::Vec4::from_value(x_axis.clone())?, g_f32::Vec4::from_value(y_axis.clone())?,
                            g_f32::Vec4::from_value(z_axis.clone())?, g_f32::Vec4::from_value(w_axis.clone())?
                    ));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Quat {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x as f64, self.y as f64, self.z as f64, self.w as f64]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f32::quat(x.as_float()? as f32, y.as_float()? as f32, z.as_float()? as f32, w.as_float()? as f32));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f32::Affine2 {
    fn to_value(&self) -> toml_edit::Value {
        let mut table: toml::InlineTable = toml::InlineTable::new();
                table.insert("matrix2",     self.matrix2.to_value());
                table.insert("translation", self.translation.to_value());

        table.into()
    }

    fn from_value(mut value: toml::Value) -> Option<Self> where Self: Sized {
        let table: &mut toml::InlineTable = value.as_inline_table_mut()?;

        Some(g_f32::Affine2::from_mat2_translation(
            g_f32::Mat2::from_value(table.remove("matrix2")?)?,
            g_f32::Vec2::from_value(table.remove("translation")?)?
        ))
    }
}
impl Serializable for g_f32::Affine3A {
    fn to_value(&self) -> toml_edit::Value {
        let mut table: toml::InlineTable = toml::InlineTable::new();
                table.insert("matrix3",     self.matrix3.to_value());
                table.insert("translation", self.translation.to_value());

        table.into()
    }

    fn from_value(mut value: toml::Value) -> Option<Self> where Self: Sized {
        let table: &mut toml::InlineTable = value.as_inline_table_mut()?;

        Some(g_f32::Affine3A::from_mat3_translation(
            g_f32::Mat3A::from_value(table.remove("matrix3")?)?.into(),
            g_f32::Vec3A::from_value(table.remove("translation")?)?.into()
        ))
    }
}

impl Serializable for g_f64::DVec2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f64::dvec2(x.as_float()?, y.as_float()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DVec3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f64::dvec3(x.as_float()?, y.as_float()?, z.as_float()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DVec4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(self.to_array()))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f64::dvec4(x.as_float()?, y.as_float()?, z.as_float()?, w.as_float()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DMat2 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f64::dmat2(g_f64::DVec2::from_value(x_axis.clone())?, g_f64::DVec2::from_value(y_axis.clone())?));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DMat3 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value(), self.z_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis, z_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f64::dmat3(g_f64::DVec3::from_value(x_axis.clone())?, g_f64::DVec3::from_value(y_axis.clone())?, g_f64::DVec3::from_value(z_axis.clone())?));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DMat4 {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x_axis.to_value(), self.y_axis.to_value(), self.z_axis.to_value(), self.w_axis.to_value()]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                if let [x_axis, y_axis, z_axis, w_axis] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                    return Some(g_f64::dmat4(
                            g_f64::DVec4::from_value(x_axis.clone())?, g_f64::DVec4::from_value(y_axis.clone())?,
                            g_f64::DVec4::from_value(z_axis.clone())?, g_f64::DVec4::from_value(w_axis.clone())?
                    ));
                }
                None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DQuat {
    fn to_value(&self) -> toml::Value {
        toml::Value::Array(toml::Array::from_iter(vec![self.x, self.y, self.z, self.w]))
    }

    fn from_value(value: toml::Value) -> Option<Self> where Self: Sized {
        match value {
            toml::Value::Array(arr) => {
                    if let [x, y, z, w] = arr.into_iter().collect::<Vec<_>>().as_slice() {
                        return Some(g_f64::dquat(x.as_float()?, y.as_float()?, z.as_float()?, w.as_float()?));
                    }
                    None
            },
            _ => None
        }
    }
}
impl Serializable for g_f64::DAffine2 {
    fn to_value(&self) -> toml_edit::Value {
        let mut table: toml::InlineTable = toml::InlineTable::new();
                table.insert("matrix2",     self.matrix2.to_value());
                table.insert("translation", self.translation.to_value());

        table.into()
    }

    fn from_value(mut value: toml::Value) -> Option<Self> where Self: Sized {
        let table: &mut toml::InlineTable = value.as_inline_table_mut()?;

        Some(g_f64::DAffine2::from_mat2_translation(
            g_f64::DMat2::from_value(table.remove("matrix2")?)?,
            g_f64::DVec2::from_value(table.remove("translation")?)?
        ))
    }
}
impl Serializable for g_f64::DAffine3 {
    fn to_value(&self) -> toml_edit::Value {
        let mut table: toml::InlineTable = toml::InlineTable::new();
                table.insert("matrix3",     self.matrix3.to_value());
                table.insert("translation", self.translation.to_value());

        table.into()
    }

    fn from_value(mut value: toml::Value) -> Option<Self> where Self: Sized {
        let table: &mut toml::InlineTable = value.as_inline_table_mut()?;

        Some(g_f64::DAffine3::from_mat3_translation(
            g_f64::DMat3::from_value(table.remove("matrix3")?)?,
            g_f64::DVec3::from_value(table.remove("translation")?)?
        ))
    }
}
