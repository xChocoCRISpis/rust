fn main(){
    let (name,country) = form_user();
    println!("Hola {}, de {}",name.trim(),country.trim());
}


fn form_user()->(String,String){
    let mut name : String = String::new();
    let mut country : String = String::new();

    println!("Ingresa tu nombre: ");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Ingresa tu país: ");
    std::io::stdin().read_line(&mut country).unwrap();

    return (name,country);
}


fn print_user()->(){
    let mut name : String = String::new();
    let mut age_string : String = String::new();

    println!("Ingresa tu nombre: ");
    std::io::stdin().read_line(&mut name).unwrap();

    println!("Ingresa tu edad : ");
    std::io::stdin().read_line(&mut age_string).unwrap();
    let age:u8 = age_string.trim().parse::<u8>().unwrap();

    println!("Hola {}, de {} años",name.trim(),age);
}

fn string_types()->(){
    let string : &str = "hola";
    let string_object : String = String::new();
    let string_object_two = String::from("hola");
    let string_object_three = "hola".to_string();

}