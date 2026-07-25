use texture_decoder_rs::astc::{decode_ldr_4_4, decode_ldr_6_6};

const ASTC_HEADER_LEN: usize = 16;
const ASTC_MAGIC: [u8; 4] = [0x13, 0xab, 0xa1, 0x5c];

fn parse_2d_header(data: &[u8]) -> (usize, usize, usize, &[u8]) {
    assert!(data.len() >= ASTC_HEADER_LEN);
    assert_eq!(&data[..4], ASTC_MAGIC);
    assert_eq!(data[6], 1, "only 2D ASTC test assets are supported");

    let width = usize::from(data[7]) | (usize::from(data[8]) << 8) | (usize::from(data[9]) << 16);
    let height =
        usize::from(data[10]) | (usize::from(data[11]) << 8) | (usize::from(data[12]) << 16);
    assert_eq!([data[4], data[5]], [4, 4]);
    (
        usize::from(data[4]),
        width,
        height,
        &data[ASTC_HEADER_LEN..],
    )
}

#[test]
fn decodes_reference_compressed_asset_to_its_rgba_golden() {
    let astc = include_bytes!("data/astcenc-tiles-ldr-complex-4x4.astc");
    let expected = include_bytes!("data/astcenc-tiles-ldr-complex-4x4.rgba");
    let (footprint_x, width, height, payload) = parse_2d_header(astc);
    assert_eq!(footprint_x, 4);

    let mut output = vec![0_u8; width * height * 4];
    decode_ldr_4_4(payload, width, height, &mut output).unwrap();
    assert_eq!(&output, expected);
}

#[test]
fn decodes_copied_tiles_asset() {
    let astc = include_bytes!("data/astcenc-tiles-ldr-4x4.astc");
    let (footprint_x, width, height, payload) = parse_2d_header(astc);
    assert_eq!((footprint_x, width, height), (4, 8, 8));

    let mut output = vec![0_u8; width * height * 4];
    decode_ldr_4_4(payload, width, height, &mut output).unwrap();
    assert_eq!(&output[..4], &[0, 0, 0, 255]);
    assert_eq!(&output[4 * 4..4 * 5], &[255, 0, 0, 255]);
    assert_eq!(
        &output[4 * width * 4..4 * width * 4 + 4],
        &[255, 255, 255, 255]
    );
    assert_eq!(
        &output[4 * (width * 4 + 4)..4 * (width * 4 + 5)],
        &[64, 192, 0, 221]
    );
}

#[test]
fn decodes_copied_6x6_asset_with_image_edge_clipping() {
    let astc = include_bytes!("data/astcenc-ldr-a-1x1-6x6.astc");
    assert_eq!(&astc[..4], ASTC_MAGIC);
    assert_eq!([astc[4], astc[5], astc[6]], [6, 6, 1]);
    let width = usize::from(astc[7]) | (usize::from(astc[8]) << 8) | (usize::from(astc[9]) << 16);
    let height =
        usize::from(astc[10]) | (usize::from(astc[11]) << 8) | (usize::from(astc[12]) << 16);
    assert_eq!((width, height), (1, 1));

    let mut output = [0_u8; 4];
    decode_ldr_6_6(&astc[ASTC_HEADER_LEN..], width, height, &mut output).unwrap();
    assert_eq!(output, [43, 173, 0, 255]);
}
