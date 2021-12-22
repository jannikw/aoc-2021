#[macro_use]
extern crate lazy_static;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let toggles = get_input();

    let core = count_on(
        &toggles
            .iter()
            .filter(|CuboidToggle(_, cuboid)| cuboid.is_in(&CORE))
            .copied()
            .collect(),
    );
    let all = count_on(&toggles);

    println!("task 1: number of cubes on in core = {}", core);
    println!("task 2: total number of cubes on = {}", all);
}

fn count_on(toggles: &Vec<CuboidToggle>) -> i64 {
    let mut count: i64 = 0;
    for (z_start, z_end) in toggles
        .iter()
        .flat_map(|CuboidToggle(_, cuboid)| {
            vec![
                cuboid.z_start - 1,
                cuboid.z_start,
                cuboid.z_end,
                cuboid.z_end + 1,
            ]
        })
        .sorted()
        .dedup()
        .tuple_windows()
    {
        let zs: Vec<&CuboidToggle> = toggles
            .iter()
            .filter(|CuboidToggle(_, cuboid)| cuboid.z_start <= z_start && z_start <= cuboid.z_end)
            .collect();
        for (y_start, y_end) in zs
            .iter()
            .flat_map(|CuboidToggle(_, cuboid)| {
                vec![
                    cuboid.y_start - 1,
                    cuboid.y_start,
                    cuboid.y_end,
                    cuboid.y_end + 1,
                ]
            })
            .sorted()
            .dedup()
            .tuple_windows()
        {
            let zys: Vec<&&CuboidToggle> = zs
                .iter()
                .filter(|CuboidToggle(_, cuboid)| {
                    cuboid.y_start <= y_start && y_start <= cuboid.y_end
                })
                .collect();
            for (x_start, x_end) in zys
                .iter()
                .flat_map(|CuboidToggle(_, cuboid)| {
                    vec![
                        cuboid.x_start - 1,
                        cuboid.x_start,
                        cuboid.x_end,
                        cuboid.x_end + 1,
                    ]
                })
                .sorted()
                .dedup()
                .tuple_windows()
            {
                if let Some(CuboidToggle(true, _)) = zys
                    .iter()
                    .filter(|CuboidToggle(_, cuboid)| {
                        cuboid.x_start <= x_start && x_start <= cuboid.x_end
                    })
                    .last()
                {
                    count += ((z_start - z_end).abs())
                        * ((y_start - y_end).abs())
                        * ((x_start - x_end).abs());
                }
            }
        }
    }

    count
}

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();
}

#[derive(Clone, Copy)]
struct Cuboid {
    x_start: i64,
    x_end: i64,
    y_start: i64,
    y_end: i64,
    z_start: i64,
    z_end: i64,
}

impl Cuboid {
    fn is_in(&self, other: &Cuboid) -> bool {
        other.x_start <= self.x_start
            && self.x_end <= other.x_end
            && other.y_start <= self.y_start
            && self.y_end <= other.y_end
            && other.z_start <= self.z_start
            && self.z_end <= other.z_end
    }
}

static CORE: Cuboid = Cuboid {
    x_start: -50,
    x_end: 50,
    y_start: -50,
    y_end: 50,
    z_start: -50,
    z_end: 50,
};

#[derive(Clone, Copy)]
struct CuboidToggle(bool, Cuboid);

impl CuboidToggle {
    fn parse(text: &str) -> Option<Self> {
        let captures = REGEX.captures(text)?;
        let toggle = captures.get(1)?.as_str() == "on";
        let x_start = captures.get(2)?.as_str().parse().ok()?;
        let x_end = captures.get(3)?.as_str().parse().ok()?;
        let y_start = captures.get(4)?.as_str().parse().ok()?;
        let y_end = captures.get(5)?.as_str().parse().ok()?;
        let z_start = captures.get(6)?.as_str().parse().ok()?;
        let z_end = captures.get(7)?.as_str().parse().ok()?;

        Some(CuboidToggle(
            toggle,
            Cuboid {
                x_start,
                x_end,
                y_start,
                y_end,
                z_start,
                z_end,
            },
        ))
    }
}

fn get_input() -> Vec<CuboidToggle> {
    include_str!("../../inputs/day22.txt")
        .trim()
        .split("\n")
        .map(|l| CuboidToggle::parse(l).unwrap())
        .collect()
}

#[test]
fn test_small_example() {
    let toggles: Vec<CuboidToggle> = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10"
        .trim()
        .split("\n")
        .map(|l| CuboidToggle::parse(l).unwrap())
        .filter(|CuboidToggle(_, cuboid)| cuboid.is_in(&CORE))
        .collect();
    assert_eq!(count_on(&toggles), 39);
}

#[test]
fn test_larger_example() {
    let toggles: Vec<CuboidToggle> = "on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682"
        .trim()
        .split("\n")
        .map(|l| CuboidToggle::parse(l).unwrap())
        .filter(|CuboidToggle(_, cuboid)| cuboid.is_in(&CORE))
        .collect();
    assert_eq!(count_on(&toggles), 590784);
}

#[test]
fn test_all() {
    let toggles: Vec<CuboidToggle> = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507"
        .trim()
        .split("\n")
        .map(|l| CuboidToggle::parse(l).unwrap())
        .collect();
    assert_eq!(count_on(&toggles), 2758514936282235);
}
