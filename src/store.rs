use crate::generate;
use crate::generate::GenData;

/// //This could be implemented by Vec type object, or tree or hashmap,
/// depending on how full you expect it to be
pub trait EcsStore<T> {
    fn add(&mut self, g_data: GenData, t: T);
    fn get(&self, g_data: GenData) -> Option<&T>;
    fn get_mut(&mut self, g_data: GenData) -> Option<&mut T>;
    fn drop(&mut self, g_data: GenData);
    
    // Function pointers - optional but helpful for iterating
    // FnMut implements FnOnce
    fn for_each<F: FnMut(GenData, &T)> (&self, f: F); // can be used as a counter
    fn for_each_mut<F: FnMut(GenData, &mut T)> (&mut self, f: F);
}

// vec collection of items
pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>
}

impl<T> VecStore<T>{
    pub fn new() -> Self {
        VecStore { items: vec![] }
    }
}

impl<T> EcsStore<T> for VecStore<T>  {
    // Copy GenData to items vec.
    fn add(&mut self, g_data: GenData, t: T) {
        while g_data.pos >= self.items.len() {
            self.items.push(None);
        }
        self.items[g_data.pos] = Some((g_data.gen, t));
    }
    
    fn get(&self, g_data: GenData) -> Option<&T> {
        if let Some(Some((inner_gen, data))) = self.items.get(g_data.pos) {
            if *inner_gen == g_data.gen {
                return Some(data); // found! immediately exit function
            }
        }
        None
    }
    
    fn get_mut(&mut self, g_data: GenData) -> Option<&mut T> {
        // Some(Some((a, b))) because vec returns an Option and so does get()
        if let Some(Some((inner_gen, data))) = self.items.get_mut(g_data.pos) {
            if *inner_gen == g_data.gen {
                return Some(data);
            }
        }
        None
    }
    
    fn drop(&mut self, g_data: GenData) {
        if let Some(Some((inner_gen, _))) = self.items.get(g_data.pos) {
            if *inner_gen == g_data.gen {
                self.items[g_data.pos] = None;
            }
        }
    }
    
    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (i, x) in self.items.iter().enumerate() {
            if let Some((gen, data)) = x {
                f(GenData {
                    pos: i,
                    gen: *gen
                }, data);
            }
        }
    }
    
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (i, x) in self.items.iter_mut().enumerate() {
            if let Some((gen, data)) = x {
                f(GenData {
                    pos: i,
                    gen: *gen
                }, data);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::generate::{GenData, GenManager};
    
    use super::*;
    
    #[test]
    fn test_store_can_drop() {
        let mut gm = GenManager::new();
        let mut vs = VecStore::new();
        
        vs.add(gm.next(), 1.61803);
        vs.add(gm.next(), 89.0/55.0);
        vs.add(gm.next(), 144.0/89.0);
        
        let gen4 = gm.next();
        vs.add(gen4, 223.0/144.0);
        
        vs.for_each_mut(|_gen, data| (*data += 0.000000000000000001));
        for (i, x)  in vs.items.iter().enumerate() {
            println!("[{}] -> item: ({:?} - {:?})", i, x.unwrap().0, x.unwrap().1);
        }
        println!("-----------After drop()-----------\n");
        vs.drop(gen4);
        for (i, x)  in vs.items.iter().enumerate() {
            println!("[{}] -> item: ({:?} - {:?})", i, x.unwrap().0, x.unwrap().1);
        }
    }
}