


fn main() {
    //Variables
    /* Se pueden typar definiendo el tipo de dato y el espacio en memoria */
    //integer
    let integer_eight:i8 = 21;
    let integer_sixteen:i16 = 31231;
    let integer_thirtytwo:i32 =731023812;

    //Enteros sin signo
    //unsigned
    let positive_integer:u8 = 23;

    //en rust por defecto las variables son inmutables
    let string : &str = "hola";

    //aqui se daclara que la variable es mutable
    let mut mutable_string:&str = "hola";
    mutable_string = "adios";





    println!("positive_int: {}", positive_integer);

    temperature();
}



fn temperature() -> (){
    let max_temperature : i8 =30;
    let min_temperature : i8 =-15;

    println!("temperatura máxima:{} , temeperatura mínima:{}", max_temperature, min_temperature);
}
