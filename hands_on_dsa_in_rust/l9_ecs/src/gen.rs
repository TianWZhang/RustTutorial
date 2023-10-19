#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64
}

pub struct EntityActive {
    active: bool,
    gen: u64
}

// where we get new GenerationIDs from
pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>, //list of all dropped entities
}

impl GenManager {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            drops: Vec::new()
        }
    }

    pub fn next(&mut self) -> GenData {
        if let Some(loc) = self.drops.pop() {
            // most recent drop
            let ea = &mut self.items[loc];
            ea.active = true;
            ea.gen += 1;
            return GenData {
                pos: loc,
                gen: ea.gen
            };
        }
        // if nothing left in drops, add on the end
        self.items.push(EntityActive{active: true, gen: 0});
        return GenData {
            pos: self.items.len() - 1,
            gen: 0
        };
    }

    pub fn drop(&mut self, g: GenData) {
        if let Some(ea) = self.items.get_mut(g.pos) {
            if ea.active && ea.gen == g.gen {
                // do not drop newer items than given
                ea.active = false;
                self.drops.push(g.pos);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_items_drop() {
        let mut gm = GenManager::new();
        let g = gm.next();
        assert_eq!(g, GenData {gen: 0, pos: 0});

        let g2 = gm.next();
        gm.next();
        gm.next();
        gm.drop(g2);

        let g3 = gm.next();
        assert_eq!(g3, GenData {gen: 1, pos: 1});
    }
}