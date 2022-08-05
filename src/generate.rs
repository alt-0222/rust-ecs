// Manager for generated data, ID

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64 // each position has its own generation
}

pub struct GenEntity {
    active: bool,
    gen: u64,
}

// where we get new GenerationIDs from
pub struct GenManager {
    items: Vec<GenEntity>,
    dropped: Vec<usize> // list of all dropped entities
}

impl GenManager {
    pub fn new() -> Self {
        GenManager { items: vec![], dropped: vec![] }
    }
    
    pub fn next(&mut self) -> GenData {
        if let Some(idx) = self.dropped.pop() {
            // most recent drop
            let mut active_entity = &mut self.items[idx];
            active_entity.active = true;
            active_entity.gen += 1;
            return GenData {
                pos: idx,
                gen: active_entity.gen
            };
            
        }
    
        // if nothing left in drops..
        self.items.push( GenEntity {
            active: true,
            gen: 0
        });
        // add to end
        return GenData {
            pos: self.items.len() - 1,
            gen: 0
        }
    }
    
    pub fn drop(&mut self, g_data: GenData) {
        if let Some(ae) = self.items.get_mut(g_data.pos) {
            if ae.active && ae.gen == g_data.gen {
                // dont drop newer items than given
                ae.active = false;
                self.dropped.push(g_data.pos);
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
        
        let _g = gm.next();
        // assert_eq!(g, GenData { gen: 0, pos: 0 });
        let g2 = gm.next();
        gm.next();
        gm.next();
        gm.drop(g2);
        let g3 = gm.next();
        assert_eq!(g3, GenData { gen: 1, pos: 1 });
    }
}