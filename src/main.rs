mod demo;

use demo::Demo;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cornell_box = demo::CornellBox::new();

    cornell_box
        .render(
            &args[1],
            args[2].parse::<u32>().unwrap(),
            args[3].parse::<u32>().unwrap(),
            args.get(4)
                .unwrap_or(&String::from("10"))
                .parse::<u32>()
                .unwrap(),
            1.0,
        )
        .unwrap();
}
