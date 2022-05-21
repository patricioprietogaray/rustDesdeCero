
struct Amigos {
    nombre: String,
    apellido: String,
    tipoDoc: TipoDocumento,
    nroDoc: u32,
    celular: u64,
    domicilio: String,
    localidad: Localidad,
}

#[derive(PartialEq, Debug)]
enum TipoDocumento {
    Dni,
    DniM,
    DniF,
    CUIL,
    CUIT,
}

#[derive(PartialEq, Debug)]
enum Localidad {
    CostaEsmeralda,
    VillaRobles,
    PuntaMédanos,
    NuevaAtlantis,
    MarDeAjó,
    SanBernardoDelTuyú,
    CostaAzul,
    LucilaDelMar,
    AguasVerdes,
    CostaDelEste,
    MarDelTuyú,
    SantaTeresita,
    CostaChica,
    LasToninas,
    SanClementeDelTuyú,
}

fn cargarAmigo(nombre: String, apellido: String, tipoDoc: TipoDocumento, nroDoc: u32, celular: u64, domicilio: String, localidad: Localidad) -> Amigos {
                    //instanciar
                    Amigos { nombre: nombre, apellido: apellido, tipoDoc: tipoDoc, nroDoc: nroDoc, celular: celular, domicilio: domicilio, localidad: localidad
                    }
                }

fn main() {
    let amigo01:Amigos=cargarAmigo(String::from("Freddy"), String::from("Castro"),TipoDocumento::Dni, 18547455,2257544558, String::from("Sarmiento 765"), Localidad::MarDeAjó);
    println!("Mi amigo se llama {} {}, quien acredita identidad con {:?} N° {}, con domicilio en {} de la ciudad de {:?}", 
    amigo01.nombre, amigo01.apellido, amigo01.tipoDoc, amigo01.nroDoc, amigo01.domicilio, amigo01.localidad);


    let amigo02:Amigos=cargarAmigo(String::from("Uriel"), String::from("Todisco"),TipoDocumento::Dni, 52654789, 2257454788,String::from("Santiago del Estero 788"), Localidad::PuntaMédanos);
    println!("Mi amigo se llama {} {}, quien acredita identidad con {:?} N° {}, con domicilio en {} de la ciudad de {:?}", 
    amigo02.nombre, amigo02.apellido, amigo02.tipoDoc, amigo02.nroDoc, amigo02.domicilio, amigo02.localidad);


    println!("Mi amigo se llama {} {}, quien acredita identidad con {:?} N° {}, con domicilio en {} de la ciudad de {:?}", 
    amigo01.nombre, amigo01.apellido, amigo01.tipoDoc, amigo01.nroDoc, amigo01.domicilio, amigo01.localidad);


    #[warn(unused_doc_comments)]
    /// arrays
    //declarar, inicializar y cargar datos, el compilador se adapta a la longitud (7).
    let dias=["Domingo", "Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado"];

    println!("El día es {}", dias[5]);

    

}