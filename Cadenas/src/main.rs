use std::io::{self, Write};

const N: usize = 100;

struct Cadena {
    longitud: usize,
    caracteres: [char; N], //Arreglo
}

impl Cadena {
    fn new() -> Cadena { //Constructor
        Cadena {
            longitud: 0,
            caracteres: ['\0'; N],
        }
    }

    //Obtener la longitud. Ej: "Hola que tal" -> Longitud = 12.
    fn obtener_longitud(&self) -> usize {
        self.longitud
    }

    //Metodo para adicionar caracteres.
    //En un arreglo, la primera posicion siempre inicia en el indice 0.
    fn add_char(&mut self, c:char) {
        if self.longitud < N {
            self.caracteres[self.longitud] = c;
            self.longitud += 1;
        }
    }

    //Metodo para obtener un caracter dada una posicion.
    fn obtener_char(&self, pos: usize) -> char {
        if pos > 0 && pos <= self.longitud {
            self.caracteres[pos-1]
        } else {
            '\0'
            }
    }

    //Contar la cantidad de apariciones de un caracter dado por el usuario. Hola como estas
    //El usuario envia: a
    //resultado = 2
    fn cant_apariciones(&self, c: char) -> i32 {
        let mut contador = 0;
        for i in 0..self.longitud {
            if self.caracteres[i] == c {
                contador += 1;
            }
        }
        contador
    }

    //Metodo para mostrar el caracter que mas veces se repite.
    fn mas_repetido(&self) -> char {
        let mut maxchar: char = self.caracteres[0];
        let mut contador: u32 = 0;
        for i in 0..self.longitud {
            let c = self.caracteres[i];
            let mut cont = 0;
            for j in 0..self.longitud{
                if self.caracteres[j] == c {
                    cont += 1;
                }
            }
            if cont > contador {
                contador = cont;
                maxchar = c;
            }
        }
        maxchar
    }

    // limpia la cadena para poder ingresar una nueva
    fn limpiar(&mut self) {
        self.longitud = 0;
        self.caracteres = ['\0'; N];
    }

    // muestra la cadena completa carácter por carácter
    fn mostrar(&self) {
        for i in 0..self.longitud {
            print!("{}", self.caracteres[i]);
        }
        println!();
    }
}

// ── helpers de entrada ──────────────────────────────────────────────
fn leer_linea() -> String {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).expect("Error al leer");
    entrada.trim().to_string()
}

fn leer_numero() -> Option<usize> {
    leer_linea().parse::<usize>().ok()
}

// ── menú ────────────────────────────────────────────────────────────
fn mostrar_menu(c: &Cadena) {
    // construimos la cadena actual para mostrarla en el encabezado
    let mut preview = String::new();
    for i in 0..c.longitud {
        preview.push(c.caracteres[i]);
    }
    if preview.is_empty() {
        preview = String::from("(vacía)");
    }

    println!("\n╔══════════════════════════════════╗");
    println!("║   CADENA: {:>22}  ║", preview);
    println!("╠══════════════════════════════════╣");
    println!("║  1. Ingresar nueva cadena        ║");
    println!("║  2. Mostrar cadena               ║");
    println!("║  3. Longitud                     ║");
    println!("║  4. Obtener carácter (posición)  ║");
    println!("║  5. Cantidad apariciones (c)     ║");
    println!("║  6. Caracter + repetido          ║");
    println!("╠══════════════════════════════════╣");
    println!("║  Q. Salir                        ║");
    println!("╚══════════════════════════════════╝");
    print!("   Opción: ");
    io::stdout().flush().expect("Error al mostrar menú");
}

fn main() {
    println!("════════════════════════════════════");
    println!("  Cadenas - POO — Programación I   ");
    println!("════════════════════════════════════");

    let mut c: Cadena = Cadena::new(); //Instancia de la clase (objeto)

    loop {
        mostrar_menu(&c);
        let opcion = leer_linea();

        match opcion.as_str() {
            "1" => {
                println!("  Ingresa la cadena:");
                let entrada = leer_linea();

                c.limpiar();

                for ch in entrada.chars() {
                    c.add_char(ch);
                }

                println!("  ✓ Cadena cargada ({} caracteres)", c.obtener_longitud());
            }

            "2" => {
                print!("  Cadena: ");
                c.mostrar();
            }

            "3" => println!("  Longitud: → {}", c.obtener_longitud()),

            "4" => {
                println!("  Ingresa la posición (1 = izquierda):");
                match leer_numero() {
                    Some(pos) if pos >= 1 && pos <= c.obtener_longitud() => {
                        println!("  Carácter en posición {}: → '{}'", pos, c.obtener_char(pos));
                    }
                    Some(_) => println!("  Posición fuera de rango (1 a {}).", c.obtener_longitud()),
                    None    => println!("  Posición inválida."),
                }
            }

            "5" => {
                println!("  Ingresa el caracter:");
                let entrada = leer_linea();
                match entrada.chars().next() {
                    Some(car)  => {
                        let cantidad = c.cant_apariciones(car);
                        println!(" El caracter {}, aparece {} vez/veces", car, cantidad);
                    }
                    None    => println!("  No ingresaste ningun caracter."),
                }
            }

            "6" => {
                if c.obtener_longitud() == 0 {
                    println!("La cadena esta vacia choquito.");
                } else {
                    let resultado = c.mas_repetido();
                    println!("El caracter que mas se repite es: {}", resultado);
                }
            }

            "q" | "Q" => { println!("\n  Hasta luego.\n"); break; }
            _          => println!("  Opción no válida."),
        }
    }
}