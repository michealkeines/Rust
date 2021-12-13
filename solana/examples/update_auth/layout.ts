import * as BufferLayout from 'buffer-layout';

/**
 * Layout for a public key
 */
export const publicKey = (property: string = 'publicKey'): Object => {
  return BufferLayout.blob(32, property);
};

/**
 * Layout for a 64bit unsigned value
 */
export const uint64 = (property: string = 'uint64'): Object => {
  return BufferLayout.blob(8, property);
};

export const rustBool = (property: string = 'boolean'): Object => {
    return BufferLayout.blob(1, property);
};
// pub struct Creator {
//     pub address: Pubkey,
//     pub verified: bool,
//     // In percentages, NOT basis points ;) Watch out!
//     pub share: u8,
// }
// pub struct Data {
//     /// The name of the asset
//     pub name: String,
//     /// The symbol for the asset
//     pub symbol: String,
//     /// URI pointing to JSON representing the asset
//     pub uri: String,
//     /// Royalty basis points that goes to creators in secondary sales (0-10000)
//     pub seller_fee_basis_points: u16,
//     /// Array of creators, optional
//     pub creators: Option<Vec<Creator>>,
// }

export const data = (property: string = 'data') => {
    const d = BufferLayout.struct(
        [
            rustString("name"),
            rustString("symbol"),
            rustString("uri"),
            BufferLayout.u16("test"),
            BufferLayout.struct(
                [[
                    publicKey("address"),
                    rustBool("verified"),
                    BufferLayout.u8("share")
                ],
                "creator"]
            )
        ],
        "Data"
    );
};

/**
 * Layout for a Rust String type
 */
export const rustString = (property: string = 'string'): Object => {
  const rsl = BufferLayout.struct(
    [
      BufferLayout.u32('length'),
      BufferLayout.u32('lengthPadding'),
      BufferLayout.blob(BufferLayout.offset(BufferLayout.u32(), -8), 'chars'),
    ],
    property,
  );
  const _decode = rsl.decode.bind(rsl);
  const _encode = rsl.encode.bind(rsl);

  rsl.decode = (buffer: Buffer, offset: number) => {
    const data = _decode(buffer, offset);
    return data.chars.toString('utf8');
  };

  rsl.encode = (str: string, buffer: Buffer, offset: number) => {
    const data = {
      chars: Buffer.from(str, 'utf8'),
    };
    return _encode(data, buffer, offset);
  };

  return rsl;
};
