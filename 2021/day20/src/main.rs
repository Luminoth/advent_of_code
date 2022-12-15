use std::fmt;

#[derive(Debug, Clone)]
struct ImageEnhancement {
    enhancement: Vec<bool>,
}

impl ImageEnhancement {
    fn get(&self, index: usize) -> bool {
        self.enhancement[index]
    }

    fn first(&self) -> bool {
        self.enhancement.first().copied().unwrap()
    }

    fn last(&self) -> bool {
        self.enhancement.last().copied().unwrap()
    }

    fn flipped(&self) -> bool {
        self.first() && self.first() != self.last()
    }
}

impl From<Vec<bool>> for ImageEnhancement {
    fn from(input: Vec<bool>) -> Self {
        // can't have both the first and last lookup flipped
        assert!(!input.first().copied().unwrap() || !input.last().copied().unwrap());

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

    fn flipped(&self, enhancement: &ImageEnhancement, iteration: usize) -> bool {
        if enhancement.flipped() {
            iteration % 2 != 0
        } else {
            false
        }
    }

    fn lit_pixel_count(&self) -> usize {
        self.image
            .iter()
            .flat_map(|x| x.iter().filter(|&v| *v))
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

    fn pixel_value(
        &self,
        row: isize,
        col: isize,
        enhancement: &ImageEnhancement,
        iteration: usize,
    ) -> char {
        self.pixel(row, col)
            .unwrap_or(if self.flipped(enhancement, iteration) {
                '1'
            } else {
                '0'
            })
    }

    fn enhance(&self, enhancement: &ImageEnhancement, iteration: usize) -> Self {
        let offset = 2;

        let mut image = vec![
            vec![false; self.width() + (offset as usize * 2)];
            self.height() + (offset as usize * 2)
        ];

        let ystart = -offset;
        let yend = (self.height() as isize) + offset;
        let xstart = -offset;
        let xend = (self.width() as isize) + offset;

        for y in ystart..yend {
            for x in xstart..xend {
                let mut index = String::with_capacity(9);
                index.push(self.pixel_value(y - 1, x - 1, enhancement, iteration));
                index.push(self.pixel_value(y - 1, x, enhancement, iteration));
                index.push(self.pixel_value(y - 1, x + 1, enhancement, iteration));
                index.push(self.pixel_value(y, x - 1, enhancement, iteration));
                index.push(self.pixel_value(y, x, enhancement, iteration));
                index.push(self.pixel_value(y, x + 1, enhancement, iteration));
                index.push(self.pixel_value(y + 1, x - 1, enhancement, iteration));
                index.push(self.pixel_value(y + 1, x, enhancement, iteration));
                index.push(self.pixel_value(y + 1, x + 1, enhancement, iteration));

                let index = usize::from_str_radix(&index, 2).unwrap();

                let xidx = (x + offset) as usize;
                let yidx = (y + offset) as usize;

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

fn run(image: &Image, enhancement: &ImageEnhancement, iterations: usize) -> usize {
    let mut image = image.clone();
    for i in 0..iterations {
        image = image.enhance(enhancement, i);
    }
    image.lit_pixel_count()
}

fn main() {
    let input = include_str!("../input.txt").trim();

    let enhancement: Vec<bool> = input
        .lines()
        .next()
        .unwrap() // enhance 3x the image size
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

    let lit_count = run(&image, &enhancement, 2);
    assert!(lit_count == 5179);
    println!("Enhanced image has {} lit pixels", lit_count);

    let lit_count = run(&image, &enhancement, 50);
    assert!(lit_count == 16112);
    println!("More Enhanced image has {} lit pixels", lit_count);
}
