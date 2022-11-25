use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

fn main() {
    let mut options = LedMatrixOptions::new();

    options.set_hardware_mapping("adafruit-hat");
    // options.set_rows(64);
    // options.set_cols(64);
    // _ = options.set_brightness(100);
    options.set_chain_length(4);
    options.set_parallel(1);
    // options.set_led_rgb_sequence("RGB");
    // options.set_hardware_pulsing(false);

    match LedMatrix::new(Some(options), None) {
        Ok(matrix) => {
            let mut canvas = matrix.offscreen_canvas();
            for red in 0..255 {
                for green in 0..255 {
                    for blue in 0..255 {
                        canvas.fill(&LedColor { red, green, blue });
                        canvas = matrix.swap(canvas);
                    }
                }
            }
        }
        Err(error_message) => eprintln!("Could not create matrix:{}", error_message),
    };
}
