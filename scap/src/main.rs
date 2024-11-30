use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::Frame,
};

fn main() {
    // Check if the platform is supported
    if !scap::is_supported() {
        println!("❌ Platform not supported");
        return;
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        println!("❌ Permission not granted. Requesting permission...");
        if !scap::request_permission() {
            println!("❌ Permission denied");
            return;
        }
    }

    // Get recording targets
    let targets = scap::get_all_targets();
    println!("Targets: {:?}", targets);

    // All your displays and windows are targets
    // You can filter this and capture the one you need.

    // Create Options
    let options = Options {
        fps: 60,
        target: None, // None captures the primary display
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        source_rect: Some(Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 2000.0,
                height: 1000.0,
            },
        }),
        ..Default::default()
    };

    // Create Capturer
    let mut capturer = Capturer::new(options);

    // Start Capture
    capturer.start_capture();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Stop Capture
    capturer.stop_capture();
}
