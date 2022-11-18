use ril::prelude::*;
use std::time::Duration;

const COLORS: [Rgb; 12] = [
    Rgb::new(255, 0, 0),
    Rgb::new(255, 128, 0),
    Rgb::new(255, 255, 0),
    Rgb::new(128, 255, 0),
    Rgb::new(0, 255, 0),
    Rgb::new(0, 255, 128),
    Rgb::new(0, 255, 255),
    Rgb::new(0, 128, 255),
    Rgb::new(0, 0, 255),
    Rgb::new(128, 0, 255),
    Rgb::new(255, 0, 255),
    Rgb::new(255, 0, 128),
];

#[test]
fn test_gif_encode() -> ril::Result<()> {
    let mut seq = ImageSequence::new();

    for color in COLORS.into_iter() {
        seq.push_frame(
            Frame::from_image(Image::new(256, 256, color)).with_delay(Duration::from_millis(100)),
        )
    }

    seq.save_inferred("tests/out/gif_encode_output.gif")?;

    Ok(())
}

#[test]
fn test_gif_decode() -> ril::Result<()> {
    for (frame, ref color) in ImageSequence::<Rgb>::open("tests/sample.gif")?.zip(COLORS) {
        let frame = frame?.into_image();

        assert_eq!(frame.dimensions(), (256, 256));
        assert_eq!(frame.pixel(0, 0), color);
    }

    Ok(())
}

#[test]
fn test_gif_palette_decode() -> ril::Result<()> {
    for (frame, color) in ImageSequence::<PalettedRgb>::open("tests/sample.gif")?.zip(COLORS) {
        let frame = frame?.into_image();
        assert_eq!(frame.pixel(0, 0).color(), color);
    }

    Ok(())
}
