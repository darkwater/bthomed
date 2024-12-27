use bytes::Buf;

#[derive(Debug, PartialEq)]
pub struct Object {
    pub name: &'static str,
    pub value: f32,
}

pub async fn decode(mut data: impl Buf) -> Vec<Object> {
    data.copy_to_bytes(3);

    let mut out = vec![];

    while data.has_remaining() {
        let header = data.get_u8();
        let len = header & 0b11111;
        let ty = header >> 5;
        // println!("len: {}, ty: {}", len, ty);

        let mut data = data.copy_to_bytes(len as usize);
        // println!("{:#02x?}", &data[..]);

        let object_id = data.get_u8();
        let value = match (len, ty) {
            (2, 0) => data.get_u8() as f32,
            (3, 0) => data.get_u16_le() as f32,
            (2, 1) => data.get_i8() as f32,
            (3, 1) => data.get_i16_le() as f32,
            (5, 2) => data.get_f32_le(),
            _ => {
                log::error!("unimplemented: len: {}, ty: {}", len, ty);
                continue;
            }
        };

        let (obj, scale) = match object_id {
            0x01 => ("battery", 1.),
            0x02 => ("temperature", 0.01),
            0x03 => ("humidity", 0.01),
            0x0c => ("voltage", 0.001),
            0x10 => ("power", 1.),
            _ => {
                log::error!("unimplemented: object_id: {:#02x?}", object_id);
                continue;
            }
        };

        out.push(Object {
            name: obj,
            value: value * scale,
        });
    }

    out
}
