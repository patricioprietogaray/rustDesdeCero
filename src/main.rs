mod main02;

fn main() {
    // Declare a variable
    let a_number;
    //println!("(vacio) The number is {}.", a_number);

    // Declare a second variable and bind the value
    let a_word = "Ten";

    // Bind a value to the first variable
    a_number = 10;
    println!("(le asigno el primer valor) The number is {}.", a_number);
    let a_number = 32;
    println!("(Shadowing) The number is {}.", a_number);

    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);

    // Addition, Subtraction, and Multiplication
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );

    // Integer and Floating point division
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // Declare variable to store result of "greater than" test, Is 1 > 4? -- false
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    let letra='A';
    let _nombre="Pepe"; //memoria cache
    let _nombre_largo= "Pepe Argento".to_string(); //heap memoria grande

    let _texto="hola";
    let mut _texto_2="hola";
    let mut _texto_3="h";
    println!("Texto {}", letra);

    let emoji='😃';
    println!("Emoji {}", emoji);

    let tupla_multi_celular=("primero","2".to_string(),true,4.5,1);
    println!("{:?}", tupla_multi_celular);

    println!("Mostrar elemento 0 {}", tupla_multi_celular.0);
    println!("Mostrar elemento 1 {}", tupla_multi_celular.1);
    println!("Mostrar elemento 2 {}", tupla_multi_celular.2);
    println!("Mostrar elemento 3 {}", tupla_multi_celular.3);
    println!("Mostrar elemento 4 {}", tupla_multi_celular.4);

    //Estructura con nombre y tipo (sin punto y coma al final)
    struct Estructura01 {
        nombre: String,
        edad: u8,
        tipo_doc: char,
        nro_doc: u32
    }

    let estru01 = Estructura01 {nombre: String::from("Patricio"), edad: 45, tipo_doc: '1', nro_doc: 25240291};

    println!("Nombre: {}; Edad: {}; Tipo Doc: {};  Nro Doc: {}", estru01.nombre, estru01.edad, estru01.tipo_doc, estru01.nro_doc);


    //Estructura con tipo 
    struct Estructura02 (
        String,
        u8,
        char,
        u32
    );

    let estru02 = Estructura02(String::from("Patricio"), 45, '1', 25240291);
    println!("Nombre: {}; Edad: {}; Tipo Doc: {};  Nro Doc: {}", estru02.0, estru02.1, estru02.2, estru02.3);



    //Estructura sin definir
    // struct Estructura03;

    // let estru03=Estructura03(String::from("Matias"), 12);


    //**************************************************** */

    //Definir una tupla Estructura
    #[derive(Debug)]
    struct KeyPress (
        String, 
        char
    );

    //Definir una estructura clasica
    #[derive(Debug)]
    struct MouseClick{
        x:i64, 
        y:i64
    }

    //definir un enum variado usando los datos que preseden
    //Actualizar y leer segun el estado de un booleano
    #[derive(Debug)]
    enum WebEvent {
        WELoad (bool),
        WEClick (MouseClick),
        WEKeys(KeyPress)
    }

    //Instanciar MouseClick y asignar coordenadas con ; al final
    let _clic = MouseClick {
        x: 100,
        y: 250
    }; 
   //mostrar el punto del clic 
   println!("El pixel cliqueado es en la coordenada x:{}, y:{}", _clic.x, _clic.y);
   
   //Instanciar la tupla KeyPress y pasarle Ctrl + N
   let _keys = KeyPress (String::from("Ctrl + "), 'N');
   println!("\nTecla presionada: {} {}", _keys.0, _keys.1);
   //las tuplas tienen elementos indexados.

   //ahora a instanciar el enum WebEvent
   let _we_load=WebEvent::WELoad(true); //varor booleano (simple)
   let _we_clic=WebEvent::WEClick(_clic);
   let _we_keys=WebEvent::WEKeys(_keys);

   println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", _we_load, _we_clic, _we_keys);



    // let _leer = WebEvent::WELoad(true);

    // //ojo al piojo, definir primero mouseclic
    // let _lugar_clic = MouseClick{x: 20, y: 40};
    // //luego el clic entre PARENTESIS NO {}
    // let _clic = WebEvent::WEClick(_lugar_clic);
    // println!("Mouse click location: {}, {}", _clic.x, _clic.y);

    // //que tecla presione
    // let _tecla_presionada = KeyPress(String::from("Pato"),'P');
    // let _tecla = WebEvent::WEKeys(_tecla_presionada);


    //Funciones
    //llamar a una funcion
    let mensaje="Pepe";
    saludar(mensaje);


    let numero=0;
    let resultado=divide_por_5(numero);
    println!("{} / 5 = {}", numero, resultado);


    ////////////////////////////////////////////////////////////////
    //Ejercicio////////////////////////////////

    

    let _fabrica_un_auto = (String::from("Silver"), transmision::Automatica, true);
    println!("Auto 1: {}, transmisión {}, convertible: {}, kilometraje: {}", 
    _fabrica_un_auto.color, 
    _fabrica_un_auto.transmision, 
    _fabrica_un_auto.convertible, 
    _fabrica_un_auto.kilometraje
);

}


fn saludar(saludo: &str) {
    println!("Hola {}", saludo);
}

fn divide_por_5(num: i16) -> i16 {
    if num == 0 {
        return 0;
    }
    num / 5
    //el ultimo no hace falta que lleve ; porque es el valor que retorna
}

