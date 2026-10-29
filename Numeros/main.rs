struct Numero {
    valor: u64,
}

impl Numero  {
    //contructor
    fn new (valor: u64)->Numero{
        Numero { valor }

    }
    fn espar(&self)->bool{
        self.valor % 2 == 0
    }
    fn cantidig(&self)->u64{
        let mut cantidad = 0;
        let mut numero = self.valor;
        while numero > 0 {
            cantidad += 1;
            numero= numero/10;
        }
        cantidad
    }
    fn esmayor(&self, x:u64)-> bool {
        self.valor > x

    }
    //metodo que devuelva la suma de todos los números naturales del valor. Ej:
    //valor = 5: 1+2+3+4+5= 15
    fn sumanat(&self)->u64 {
        let mut suma = 0;
        for i in 1..=self.valor {
            suma += i;
        }
        suma
    }

    //1.-Método que devuelve potencia propia: valor elevado a valor. ej: valor=2, 2 elevado a 2 = 4
    //2.-Método que devuelve la cantidad de multiplos del valor en su rango, excluyendo el valor.
    //3.-Método que devuelva la cantidad de dígitos impares.

    //hecho por: Menacho Araúz Uriel
    //grupo: SC
    
    //potencia propia
   fn _potenciapropia(&self)->u64 {
         self.valor.pow(self.valor as u32)
   }

   // Cantidad de multiplos del valor en el rango 
   fn esmultiplo(&self)->u64 {
    let mut cantidad = 0;
    for i in 1..self.valor {
        if self.valor % i == 0 {
            cantidad += 1;
        }
    }
    cantidad
   }

   // cantidad de digitos impares
   fn cantimpares(&self)->u64 {
    let mut cantidad = 0;
    let mut n = self.valor;
    while n > 0 {
        let digito = n % 10;
         if digito % 2 != 0 {
            cantidad += 1;
         }
        n /= 10;
    }
    cantidad
   }


   //1.- Invertir los digitos del numero 
   //2.- Es capicua?
   //3.- Obtener la raiz digital: la raiz digital de 2134 es: 2 + 1 + 3 + 4 = 10 (como 10 tiene de digitos...
   // 1 + 0 = 1.. Raiz digital de 2134 es 1)

   //Invertir los digitos del numero
    fn invertir(&self)->u64 {
      let mut n = self.valor;
      let mut inv = 0;
        while 0 < n {
            inv = inv * 10 + n % 10;
            n /= 10; 
        }
        inv
    }

    // Es capicua?
    fn capicua(&self)->bool {
        let mut n = self.valor;
        let mut c = 0;
        while 0 < n {
            c = c * 10 + n % 10;
            n /= 10;

        }
        c == self.valor
    }

    // Raiz Digital
    fn raizdigital(&self)->u64 {
        let mut n = self.valor;
        let mut sumar = 0;
        while n > 0 || sumar > 9 {
            if n == 0 {
                n = sumar;
                sumar = 0;
            }
            sumar += n % 10;
            n /= 10;
        }
        sumar
    }
}

fn main() {
    //la instancia
    let n = Numero::new (101);
    //llamadas a métodos
    println!("El valor actual de la instancia n es: {}", n.valor);
    println!("El valor es par?: {}", n.espar());
    println!("La cantidad de dígitos es: {}", n.cantidig());
    println!("El valor , es mayor que el numero x?: {}", n.esmayor(40));
    println!("La suma de los números naturales es: {}", n.sumanat());
   // println!("La potencia propia del valor es: {}", n.potenciapropia());
    println!("Cantidad de múltiplos del valor en su rango: {}", n.esmultiplo());
    println!("La cantidad de dígitos impares es: {}", n.cantimpares());
    println!("El digito invertido del valor es: {}", n.invertir());
    println!("¿El valor es capicúa?: {}", n.capicua());
    println!("La raíz digital del valor es: {}", n.raizdigital());
}
