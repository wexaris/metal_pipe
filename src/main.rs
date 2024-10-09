#[cfg(test)] mod test;

/// Single structure to track usable pipe length, and how a blank pipe has been cut.
#[derive(Debug, PartialEq)]
pub struct PipeBlank {
    pub original_size: u32,
    pub current_size: u32,
    pub cuts: Vec<u32>,
}

impl PipeBlank {
    pub fn new(size: u32) -> Self {
        PipeBlank { original_size: size, current_size: size, cuts: vec![] }
    }

    /// Cut a pipe of a given size from this blank.
    /// Reduces remaining pipe length, and tracks how much was cut.
    ///
    /// NOTE: requested cut length must not exceed available length.
    pub fn cut(&mut self, size: u32) {
        assert!(self.current_size >= size);
        self.cuts.push(size);
        self.current_size -= size;
    }

    pub fn is_unused(&self) -> bool { self.current_size == self.original_size }
}

/// Cuts a set of pipe blanks into the requested sizes,
/// while producing as little waste material as possible.
///
/// Returns a list of objects that describe how long each blank was originally,
/// what length pieces it's been divided into, and how much pipe still remains.
///
/// If a requested length pipe cannot be cut, a message will be logged to the console.
///
/// NOTE: largest possible lengths will be cut first.
///
/// NOTE: blank pipe definition order will not affect results.
pub fn cut_pipes(blanks: &[u32], mut requests: Vec<u32>) -> Vec<PipeBlank> {
    // track original, current, and used length in a single pipe object
    let mut blanks = blanks.iter()
        .map(|len| PipeBlank::new(*len))
        .collect::<Vec<_>>();

    // not strictly necessary!
    // provides consistent results no matter the initial order of the blanks
    // otherwise, different orderings of blanks can result in different usage patterns
    // NOTE: does NOT affect TOTAL amount of leftover material
    blanks.sort_by_key(|b| b.original_size);

    // sort the requests largest to smallest for the greedy algorithm
    requests.sort();
    requests.reverse();

    // go through the requests
    for req in requests {
        // look for the best blank - one that will leave the least waste
        let match_idx = find_blank(&blanks, req, false)
            .or_else(|| find_blank(&blanks, req, true));

        // update parameters after the cut
        if let Some(blank_idx) = match_idx {
            blanks[blank_idx].cut(req);
        }
        else { // no blank could fit the requested size
            eprintln!("pipe length ({req}) could not be cut!");
        }
    }

    blanks
}

fn find_blank(blanks: &[PipeBlank], cut: u32, allow_unused: bool) -> Option<usize> {
    let mut best_idx = None;
    let mut min_waste = u32::MAX;

    for (idx, blank) in blanks.iter().enumerate() {
        if (blank.current_size >= cut) && (!blank.is_unused() || (blank.is_unused() && allow_unused)) {
            let waste = blank.current_size - cut;
            if waste < min_waste {
                min_waste = waste;
                best_idx = Some(idx);
            }
        }
    }

    best_idx
}

fn main() {
    let blanks = vec![100, 120, 150, 200, 250, 300, 120, 150, 400, 500, 350, 100, 180, 220, 200, 175, 210, 300, 280, 450, 500, 160, 230, 270, 190, 310, 220, 240, 100, 90, 80, 150, 140, 350, 180, 200, 220, 400, 500, 500, 480, 410, 150, 250, 300, 350, 100, 90, 60, 70];
    let requests = vec![30, 60, 80, 90, 120, 20, 150, 180, 200, 220, 240, 100, 50, 30, 120, 60, 300, 180, 50, 100, 150, 160, 220, 80, 40, 70, 150, 190, 50, 60, 30, 120, 140, 160, 100, 200, 230, 250, 120, 10, 130, 140, 150, 180, 170, 60, 90, 100, 250, 260, 300, 350, 400, 220, 190, 280, 120, 50, 30, 80, 110, 200, 90, 150, 250, 260, 300, 220, 160, 180, 130, 120];

    let mut blanks = cut_pipes(&blanks, requests);
    consistent_sort(&mut blanks);

    print_blanks(&blanks);
}

pub fn print_blanks(blanks: &[PipeBlank]) {
    println!("Blank usage:");
    for blank in blanks {
        print!("{}:", blank.original_size);
        for cut in &blank.cuts {
            print!(" {}", cut);
        }
        if blank.current_size > 0 {
            print!(" -{}", blank.current_size);
        }
        println!();
    }
}

/// Sort by original pipe size and highest cut size.
pub fn consistent_sort(pipes: &mut Vec<PipeBlank>) {
    pipes.sort_by(|x, y|
        x.original_size.cmp(&y.original_size)
            .then(x.cuts.cmp(&y.cuts))
            .reverse()
    );
}
