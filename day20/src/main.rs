use std::fmt;

fn main() {
    let (enhancement_string, image) = get_input();

    let enhanced_image = enhance(&enhancement_string, &enhance(&enhancement_string, &image));

    println!(
        "task 1: lit pixels after enhancing twice = {}",
        enhanced_image.count_lit()
    );

    let enhanced_image_50 = enhance_n_times(&enhancement_string, &image, 50);
    println!(
        "task 2: lit pixels after enhancing 50 times = {}",
        enhanced_image_50.count_lit()
    );
}

type EnhancementString = Vec<u8>;

fn parse_enhancement_string(text: &str) -> EnhancementString {
    text.chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .collect::<EnhancementString>()
}

#[derive(Clone)]
struct Image {
    width: i32,
    height: i32,
    pixels: Vec<u8>,
    infinity: u8,
}

impl Image {
    fn get(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            self.infinity
        } else {
            self.pixels[(y * self.width + x) as usize]
        }
    }

    fn count_lit(&self) -> usize {
        self.pixels.iter().map(|p| *p as usize).sum()
    }

    fn parse<'a, I>(lines: I) -> Image
    where
        I: IntoIterator<Item = &'a str>,
    {
        let mut height = 0;
        let mut width = None;
        let mut pixels = Vec::new();

        for line in lines {
            let mut line_pixels = line
                .chars()
                .map(|c| if c == '#' { 1 } else { 0 })
                .collect::<Vec<u8>>();
            pixels.append(&mut line_pixels);
            assert_eq!(width.get_or_insert(line.len()), &line.len());
            height += 1;
        }

        Image {
            width: width.unwrap_or(0) as i32,
            height,
            pixels,
            infinity: 0,
        }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", if self.get(x, y) == 1 { "#" } else { "." })?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn enhance(enhancement_string: &Vec<u8>, image: &Image) -> Image {
    let mut enhanced_pixels = vec![0u8; ((image.width + 2) * (image.height + 2)) as usize];
    for y in -1..image.height + 1 {
        for x in -1..image.width + 1 {
            let index = (image.get(x - 1, y - 1) as usize) << 8
                | (image.get(x, y - 1) as usize) << 7
                | (image.get(x + 1, y - 1) as usize) << 6
                | (image.get(x - 1, y) as usize) << 5
                | (image.get(x, y) as usize) << 4
                | (image.get(x + 1, y) as usize) << 3
                | (image.get(x - 1, y + 1) as usize) << 2
                | (image.get(x, y + 1) as usize) << 1
                | (image.get(x + 1, y + 1) as usize);
            enhanced_pixels[((y + 1) * (image.width + 2) + x + 1) as usize] =
                enhancement_string[index];
        }
    }

    let new_infinity = if image.infinity == 1 {
        enhancement_string[511]
    } else {
        enhancement_string[0]
    };

    Image {
        width: image.width + 2,
        height: image.height + 2,
        pixels: enhanced_pixels,
        infinity: new_infinity,
    }
}

fn enhance_n_times(enhancement_string: &Vec<u8>, image: &Image, n: usize) -> Image {
    (0..n).fold(image.clone(), |i, _| enhance(&enhancement_string, &i))
}

fn get_input() -> (Vec<u8>, Image) {
    let mut lines = include_str!("../../inputs/day20.txt").trim().split("\n");
    let enhancement_string = parse_enhancement_string(lines.next().unwrap());

    // Skip empty line
    assert_eq!(lines.next().unwrap(), "");

    let image = Image::parse(lines);

    (enhancement_string, image)
}

#[test]
fn test_example() {
    let enhancement_string = parse_enhancement_string("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#");
    let image = Image::parse(
        "...............
...............
...............
...............
...............
.....#..#......
.....#.........
.....##..#.....
.......#.......
.......###.....
...............
...............
...............
...............
..............."
            .split("\n"),
    );
    let image_1 = enhance(&enhancement_string, &image);
    let image_2 = enhance(&enhancement_string, &image_1);
    let image_50 = enhance_n_times(&enhancement_string, &image, 50);

    assert_eq!(image_2.count_lit(), 35);
    assert_eq!(image_50.count_lit(), 3351);
}
