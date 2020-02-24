//! Based on: https://android.googlesource.com/platform/external/bluetooth/bluez/+/master/src/eir.c

const HCI_MAX_EIR_LENGTH: usize = 240;

#[repr(u8)]
#[allow(unused)]
enum EirByte {
    Flags = 0x01,
    Uuid16Some = 0x02,
    Uuid16All = 0x03,
    Uuid32Some = 0x04,
    Uuid32All = 0x05,
    Uuid128Some = 0x06,
    Uuid128All = 0x07,
    NameShort = 0x08,
    NameComplete = 0x09,
    TxPower = 0x0A,
    DeviceId = 0x10,
}

/// EIR data object.
pub struct EirData {
    // GSList *services;
    pub flags: u8,
    pub name: Option<String>,
    pub name_complete: bool,
}

/// Parse given EIR data.
pub fn parse(mut eir_data: &[u8]) -> Option<EirData> {
    // No EIR data
    if eir_data.is_empty() {
        return None;
    }

    let mut flags = 1u8;
    let mut name = None;
    let mut name_complete = false;

    let mut len = 0usize;
    let mut _uuid16_count: usize = 0;
    let mut _uuid32_count: usize = 0;
    let mut _uuid128_count: usize = 0;
    // let *uuid16 = NULL;
    // let *uuid32 = NULL;
    // let *uuid128 = NULL;
    // uuid_t service;
    // char *uuid_str;
    // unsigned int i;

    while !eir_data.is_empty() && len < HCI_MAX_EIR_LENGTH - 1 {
        let field_len = eir_data[0] as usize;

        // Check for EIR end
        if field_len == 0 {
            break;
        }

        match eir_data[1] {
            x if x == EirByte::Uuid16Some as u8 || x == EirByte::Uuid16All as u8 => {
                _uuid16_count = field_len as usize / 2;
                // uuid16 = &eir_data[2];
            }
            x if x == EirByte::Uuid32Some as u8 || x == EirByte::Uuid32All as u8 => {
                _uuid32_count = field_len as usize / 4;
                // uuid32 = &eir_data[2];
            }
            x if x == EirByte::Uuid128Some as u8 || x == EirByte::Uuid128All as u8 => {
                _uuid128_count = field_len as usize / 16;
                // uuid128 = &eir_data[2];
            }
            x if x == EirByte::Flags as u8 => {
                flags = eir_data[2];
            }
            x if x == EirByte::NameShort as u8 || x == EirByte::NameComplete as u8 => {
                dbg!(&eir_data[2..field_len - 1]);
                dbg!(&eir_data[2..field_len]);
                dbg!(&eir_data[2..=field_len]);

                name = String::from_utf8(eir_data[2..=field_len].to_vec()).ok();
                name_complete = x == EirByte::NameComplete as u8;
            }
            _ => {}
        }

        len += field_len as usize + 1;
        eir_data = &eir_data[field_len as usize + 1..];
    }

    // Stop if invalid length
    if len > HCI_MAX_EIR_LENGTH {
        return None;
    }

    // Return data
    Some(EirData {
        flags,
        name,
        name_complete,
    })

    // // Skip if no UUIDs were parsed
    // let total = uuid16_count + _uuid32_count + _uuid128_count;
    // if total == 0 {
    //     return 0;
    // }

    // /* Generate uuids in SDP format (EIR data is Little Endian) */
    // service.type = SDP_UUID16;
    // for (i = 0; i < uuid16_count; i++) {
    //         uint16_t val16 = uuid16[1];
    //         val16 = (val16 << 8) + uuid16[0];
    //         service.value.uuid16 = val16;
    //         uuid_str = bt_uuid2string(&service);
    //         eir->services = g_slist_append(eir->services, uuid_str);
    //         uuid16 += 2;
    // }
    // service.type = SDP_UUID32;
    // for (i = uuid16_count; i < uuid32_count + uuid16_count; i++) {
    //         uint32_t val32 = uuid32[3];
    //         int k;
    //         for (k = 2; k >= 0; k--)
    //                 val32 = (val32 << 8) + uuid32[k];
    //         service.value.uuid32 = val32;
    //         uuid_str = bt_uuid2string(&service);
    //         eir->services = g_slist_append(eir->services, uuid_str);
    //         uuid32 += 4;
    // }
    // service.type = SDP_UUID128;
    // for (i = uuid32_count + uuid16_count; i < total; i++) {
    //         int k;
    //         for (k = 0; k < 16; k++)
    //                 service.value.uuid128.data[k] = uuid128[16 - k - 1];
    //         uuid_str = bt_uuid2string(&service);
    //         eir->services = g_slist_append(eir->services, uuid_str);
    //         uuid128 += 16;
    // }
}
