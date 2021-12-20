use std::fmt;

#[derive(Debug, Clone)]
struct ImageEnhancement {
    enhancement: Vec<bool>,
}

impl ImageEnhancement {
    fn get(&self, index: usize) -> bool {
        self.enhancement[index]
    }
}

impl From<Vec<bool>> for ImageEnhancement {
    fn from(input: Vec<bool>) -> Self {
        Self { enhancement: input }
    }
}

#[derive(Debug, Default, Clone)]
struct Image {
    image: Vec<Vec<bool>>,
}

impl Image {
    fn width(&self) -> usize {
        self.image[0].len()
    }

    fn height(&self) -> usize {
        self.image.len()
    }

    fn lit_pixel_count(&self) -> usize {
        self.image
            .iter()
            .map(|x| x.iter().filter(|&v| *v))
            .flatten()
            .count()
    }

    fn pixel(&self, row: isize, col: isize) -> Option<char> {
        if row < 0 || col < 0 {
            return None;
        }

        Some(
            if self.image.get(row as usize)?.get(col as usize).copied()? {
                '1'
            } else {
                '0'
            },
        )
    }

    fn pixel_value(&self, row: isize, col: isize) -> char {
        self.pixel(row, col).unwrap_or('0')
    }

    // TODO: this works for the example, not the actual input
    // I think because the algorithm's 0 turns to a 1
    // and this might be processing too much of the "infinite" image for that
    fn enhance(&self, enhancement: &ImageEnhancement) -> Self {
        // enhance 3x the image size
        let mut image = vec![vec![false; self.width() * 3]; self.height() * 3];

        let ystart = -(self.height() as isize);
        let yend = (self.height() * 2) as isize;
        let xstart = -(self.width() as isize);
        let xend = (self.width() * 2) as isize;

        for y in ystart..yend {
            for x in xstart..xend {
                let mut index = String::with_capacity(9);
                index.push(self.pixel_value(y - 1, x - 1));
                index.push(self.pixel_value(y - 1, x));
                index.push(self.pixel_value(y - 1, x + 1));
                index.push(self.pixel_value(y, x - 1));
                index.push(self.pixel_value(y, x));
                index.push(self.pixel_value(y, x + 1));
                index.push(self.pixel_value(y + 1, x - 1));
                index.push(self.pixel_value(y + 1, x));
                index.push(self.pixel_value(y + 1, x + 1));

                let index = usize::from_str_radix(&index, 2).unwrap();

                let xidx = (x + self.width() as isize) as usize;
                let yidx = (y + self.height() as isize) as usize;
                image[yidx][xidx] = enhancement.get(index);
            }
        }

        Self { image }
    }
}

impl From<Vec<Vec<bool>>> for Image {
    fn from(input: Vec<Vec<bool>>) -> Self {
        Self { image: input }
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.image {
            for col in row {
                write!(f, "{}", if *col { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(image: Image, enhancement: ImageEnhancement) {
    println!("{}", image);
    println!();

    let mut image = image;
    for _ in 0..2 {
        image = image.enhance(&enhancement);
    }

    println!("enhanced:");
    println!();
    println!("{}", image);

    let lit_count = image.lit_pixel_count();
    println!("Enhanced image has {} lit pixels", lit_count);
}

fn main() {
    let input = include_str!("../sample.txt").trim();

    let enhancement: Vec<bool> = input
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(|ch| ch == '#')
        .collect();

    let enhancement: ImageEnhancement = enhancement.into();

    let image: Vec<Vec<bool>> = input
        .lines()
        .skip(1)
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.chars().map(|ch| ch == '#').collect())
        })
        .collect();

    let image: Image = image.into();

    part1(image, enhancement);
}
