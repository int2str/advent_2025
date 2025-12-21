#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Joltages {
    data: [u64; 16],
    len: usize,
}

impl Joltages {
    pub fn new(len: usize) -> Self {
        Self {
            data: [0; 16],
            len: std::cmp::min(len, 16),
        }
    }

    pub fn from_u64(button: u64, len: usize) -> Self {
        let mut joltages = Self::new(len);
        for idx in 0..joltages.len {
            if (button & (1 << idx)) != 0 {
                joltages[idx] = 1;
            }
        }
        joltages
    }

    pub fn exceeds(&self, limit: &Self) -> bool {
        (0..self.len)
            .into_iter()
            .any(|idx| self.data[idx] > limit.data[idx])
    }

    pub fn is_zero(&self) -> bool {
        self.data.iter().all(|&n| n == 0)
    }

    pub fn gcd(&self) -> u64 {
        std::cmp::max(
            self.data
                .iter()
                .skip(1)
                .fold(self.data[0], |g, &n| gcd(g, n)),
            1,
        )
    }

    pub fn iter(&self) -> impl Iterator<Item = &u64> {
        self.data[0..self.len].iter()
    }
}

impl std::ops::Index<usize> for Joltages {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl std::ops::IndexMut<usize> for Joltages {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl std::ops::Add for Joltages {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let len = std::cmp::min(16, std::cmp::max(self.len, rhs.len));
        let mut data = [0u64; 16];
        for (idx, item) in data.iter_mut().enumerate().take(len) {
            *item = self.data[idx].wrapping_add(rhs.data[idx]);
        }
        Self { data, len }
    }
}

impl std::ops::Sub for Joltages {
    type Output = Option<Self>;

    fn sub(self, rhs: Self) -> Self::Output {
        let len = std::cmp::min(16, std::cmp::max(self.len, rhs.len));
        let mut data = [0u64; 16];
        for (idx, item) in data.iter_mut().enumerate().take(len) {
            if self.data[idx] < rhs.data[idx] {
                return None; // Cannot subtract, would go negative
            }
            *item = self.data[idx] - rhs.data[idx];
        }
        Some(Self { data, len })
    }
}

impl std::ops::Div<u64> for Joltages {
    type Output = Self;

    fn div(self, divisor: u64) -> Self::Output {
        let mut data = [0u64; 16];
        for (idx, item) in data.iter_mut().enumerate().take(self.len) {
            *item = self.data[idx] / divisor;
        }
        Self {
            data,
            len: self.len,
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}
