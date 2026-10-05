const MAX: usize = 200;

struct Vectores {
    //Definir atributos del objeto.
    dimension: usize,
    elemento: [u64;MAX],
}

impl Vectores {
    //Constructor
    fn new() -> Vectores {
        Vectores {
            dimension: 0,
            elemento: [0;MAX],
        }
    }

    fn dim(&self) -> usize {
        self.dimension
    }

    fn add_element(&mut self, e:u64){
        if self.dimension < MAX {
            self.elemento[self.dimension] = e;
            self.dimension += 1;
        }
    }

    fn mostrar(&self) {
        for i in 0..self.dimension { //Cuando hago uso de for, siempre va hasta el ultimo elemento - 1.
            println!(" Elemento[{}] = {}", i, self.elemento[i]);
        }

    }
}

fn main() {
    let mut v = Vectores::new(); //Instancia

    v.add_element(3);
    v.add_element(1);
    v.add_element(5);
    v.add_element(7);
    v.add_element(0);
    v.add_element(13);
    v.add_element(18);
    v.add_element(2);
    v.add_element(4);

    println!("-------------------");
    v.mostrar();
    println!("-------------------");

    println!("La cantidad de elementos es (Dimension): {}", v.dim());
}
