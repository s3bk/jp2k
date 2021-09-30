fn main() {
    let bytes = include_bytes!("rust-logo-512x512-blk.jp2");

    let codec = jp2k::Codec::jp2();
    let stream = jp2k::Stream::from_bytes(bytes).unwrap();
    // let stream = jp2k::Stream::from_file("/mnt/c/projects/iiif-server/cache/remote/322930.jp2").unwrap();

    let jp2k::ImageBuffer { buffer, width, height, num_bands } = jp2k::ImageBuffer::build(
        codec,
        stream,
        jp2k::DecodeParams::default(),
    )
    .unwrap();
    assert_eq!(num_bands, 4);

    let img = image::RgbaImage::from_raw(
        width,
        height,
        buffer,
    )
    .unwrap();

    img.save("examples/output/test.png").unwrap();
}
