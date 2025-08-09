use dioxus::prelude::*;


const FOTOCODIGO: Asset = asset!("/assets/codigos/codewars.png");
const CODIGO: Asset = asset!("/assets/codigos/codigo.txt");

const IMAGEN_1_PNG: Asset = asset!("/assets/wallpapers/1.png");
const IMAGEN_2_PNG: Asset = asset!("/assets/wallpapers/2.png");
const IMAGEN_3_PNG: Asset = asset!("/assets/wallpapers/3.png");
const IMAGEN_4_PNG: Asset = asset!("/assets/wallpapers/4.png");
const IMAGEN_5_PNG: Asset = asset!("/assets/wallpapers/5.png");
const IMAGEN_6_PNG: Asset = asset!("/assets/wallpapers/6.png");
const IMAGEN_7_PNG: Asset = asset!("/assets/wallpapers/7.png");
const IMAGEN_8_PNG: Asset = asset!("/assets/wallpapers/8.png");
const IMAGEN_9_PNG: Asset = asset!("/assets/wallpapers/9.png");
const IMAGEN_10_PNG: Asset = asset!("/assets/wallpapers/10.png");
const IMAGEN_11_PNG: Asset = asset!("/assets/wallpapers/11.png");
const IMAGEN_12_PNG: Asset = asset!("/assets/wallpapers/12.png");
const IMAGEN_13_PNG: Asset = asset!("/assets/wallpapers/13.png");
//fotos
const MICARA: Asset = asset!("/assets/fotosrandom/yopp.jpg");

/*
pub fn Codigo() -> Element {
    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);
    rsx!{
        h1 {"ACA PONDRE CODIGO QUE ME COMPETE XD"}
        img {src: FOTOCODIGO} 
        a {href: "https://www.codewars.com/kata/5899642f6e1b25935d000161/train/rust", p{"Ejercicio en cuestion"}} 
        a {href: CODIGO, p{"codigo"}}

    }

}
*/
pub fn Wallpapers() -> Element {
    rsx!{
        div { class: "wallpap",id: "wallpapers",

        h1 {"Wallpapers que tengo en el pc :p"}
            
            a{target:"_blank",href: IMAGEN_1_PNG,img {id:"1",src: IMAGEN_1_PNG}}  
            a{target:"_blank",href: IMAGEN_2_PNG,img {id:"2",src: IMAGEN_2_PNG}}  
            a{target:"_blank",href: IMAGEN_3_PNG,img {id:"3",src: IMAGEN_3_PNG}}  
            a{target:"_blank",href: IMAGEN_4_PNG,img {id:"4",src: IMAGEN_4_PNG}}  
            a{target:"_blank",href: IMAGEN_5_PNG,img {id:"5",src: IMAGEN_5_PNG}}  
            a{target:"_blank",href: IMAGEN_6_PNG,img {id:"6",src: IMAGEN_6_PNG}}  
            a{target:"_blank",href: IMAGEN_7_PNG,img {id:"7",src: IMAGEN_7_PNG}}  
            a{target:"_blank",href: IMAGEN_8_PNG,img {id:"8",src: IMAGEN_8_PNG}}  
            a{target:"_blank",href: IMAGEN_9_PNG,img {id:"9",src: IMAGEN_9_PNG}}  
            a{target:"_blank",href: IMAGEN_10_PNG,img {id:"10",src: IMAGEN_10_PNG}}  
            a{target:"_blank",href: IMAGEN_11_PNG,img {id:"11",src: IMAGEN_11_PNG}}  
            a{target:"_blank",href: IMAGEN_12_PNG,img {id:"12",src: IMAGEN_12_PNG}}  
            a{target:"_blank",href: IMAGEN_13_PNG,img {id:"13",src: IMAGEN_13_PNG}}  
        } 
     
    }

}

#[component]
pub fn Presentacion() -> Element {
    rsx!{
        div {class:"principal", id: "rey",
         
            img { src: "https://avatars.githubusercontent.com/u/136939439?v=4"}, 
            
            h1 {"E.C WEB. Es mi web personal :p, dejare esto como ", a {target:"_blank",href: "https://github.com/realECoficial/realECoficial.github.io", {"repositorio"}}}
            h1 {"Lo estare usando para trackear en donde voy en mis estudios:"}
            ol {
                li { h2 {"Rust."} }
                li { h2 {"Filosofia."} }
                li { h2 {"Matematicas."} }
            } 
        }
    
    }
}

#[component]
pub fn About_me() -> Element {
    rsx!{img {id:"MICARA", src: MICARA }
        div {class:"sobremi", id: "about",
                 
         
            
            h1 {"Hola, soy Emilio .C, me gusta: Leer, programar, indagar: cosas de linux, tecnologia, y esop :p"}
            h2 {"Mis Lenguajes Based actuales son:", 
                ol {
                    li { "Rust" } 
                },
                "los antiguos:",
                ol {
                    li { "C/C++" } 
                }
            }
            h2 {"Mis Lenguajes no tan basados son:", 
                ol {
                    li { "JavaScript" } 
                    li { "c#" } 
                    li { "Java" } 
                    li { "Python" } 
                    li { "Php" } 
                },
            }
        }
    
    }
}
// ._.
#[component]
pub fn Indice() -> Element {
    rsx!{
        
        br {}         
        div { id: "indice",
            h1 {"ìndice"}
        
            a {href:"#estudios",       p{"Estudios Generales"}} 
            a {href:"#libros",       p{"Libros que recomiendo"}} 
            a {href:"#linux",       p{"Secciòn linux general"}} 
            a {href:"#Seccion_util", p{"Secciòn util"}} 
            a {href:"#musica",       p{"Musica"}} 
            a {href:"#futuro",       p{"Futuro"}} 
        }
        br {}         
        br {}         
    }
}


const ESTUDIO1: Asset = asset!("/assets/libros/Think. A Compelling Introduction To Philosophy.pdf");
const ESTUDIO2: Asset = asset!("/assets/libros/Why Math_ (Undergraduate Texts in Mathematics).pdf");
#[component]
pub fn Seccion_estudio_rust() -> Element {
    rsx! {
        br {id: "estudios"}         
        //hr { id: "lineas_separar"} 
        div {class:"noseya", id: "Texto",  

            h1 {"Seccion Estudios Generales"}
            a {target:"_blank",href: "https://www.rust-lang.org/", 
                h2 {  "Rust: Chapter 10.2." }
                ol {
                    li {a{target:"_blank", href:"https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax","Trait Bound Syntax"}} 
                } 
            }
            a {target:"_blank",href: "https://www.codewars.com/users/realECoficial", 
                h2{"Codewars: Ejercicios hechos."}
                 
                ol {
                    li {a{target:"_blank", href:"https://www.codewars.com/kata/546f922b54af40e1e90001da","Replace With Alphabet Position (6 kyu)"}} 
                    li {a{target:"_blank", href:"https://www.codewars.com/kata/513e08acc600c94f01000001","RGB To Hex Conversion (5 kyu PB Fri Aug 8 01:19:43 AM 2025)"}} 
                    li {a{target:"_blank", href:"https://www.codewars.com/kata/5259b20d6021e9e14c0010d4","Reverse words (7 kyu)"}} 
                    li {a{target:"_blank", href:"https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0","Remove First and Last Character (8 kyu)"}} 
                    li {a{target:"_blank", href:"https://www.codewars.com/kata/589478160c0f8a40870000bc","Area of an arrow (7 kyu)"}} 
                } 
            } 
            a {target:"_blank",href: "https://dioxuslabs.com/", 
                h2 {"Dioxus: " }
                ol {
                    li {a{target:"_blank", href:"https://realec-web.xyz/","Esta web"}} 
                } 
            } 
            a {target:"_blank",href: "https://www.susanrigetti.com/philosophy" , 
                h2 {  "Filosofia: Think (cap 180)" }
            } 
                ol {
                    li {a{target:"_blank", href: ESTUDIO1,"Think. A Compelling Introduction To Philosophy"}} 
                } 
        
            a {target:"_blank",href: "https://www.susanrigetti.com/math" , 
                h2 {  "Matematicas." }
            } 
                ol {
                    li {a{target:"_blank", href: "https://www.khanacademy.org/math/get-ready-for-algebra-i/x127ac35e11aba30e:get-ready-for-equations-inequalities","Khan Academy"}} 
                    li {a{target:"_blank", href: ESTUDIO2,"Why Math? (i solve just 1 problem)"}} 
                } 
            h1 {"Electivos"}
            a {target:"_blank",href: "https://www.raylib.com/cheatsheet/cheatsheet.html" , 
                h2 {  "Raylib" }
            } 
                ol {
                    li {a{target:"_blank", href: "https://www.raylib.com/examples.html","Raylib in c"}} 
                } 
        
        }
    }
}


//libros
const MARCUS: Asset = asset!("/assets/libros/Marcus-Aurelius-Meditations.pdf");
const LIBRO1: Asset = asset!("/assets/libros/25 Roberto Bolaño - Los detectives salvajes.pdf");
const LIBRO2: Asset = asset!("/assets/libros/Indigno-de-ser-humano.pdf");
const LIBRO3: Asset = asset!("/assets/libros/Nueva gramática básica de la lengua española.pdf");

#[component]
pub fn Seccion_libros() -> Element {
    rsx! {
        br {id: "libros"}         
        //hr { id: "lineas_separar"} 
        div {class:"libros", id: "Texto",  

            h1 {"Repositorio de libros."}
            a {target:"_blank",href: LIBRO1, 
                h2 {  "Roberto Bolaño - Los detectives salvajes" }
            } 
             
            a {target:"_blank",href: MARCUS , 
                h2 {  "Marcus Aurelius - Meditations." }
            } 
             
                 
            a {target:"_blank",href: LIBRO2, 
                h2 {  "Osamu Dazai - Indigno de ser humano" }
            } 
            a {target:"_blank",href: "https://archive.org/details/wall0000haus/page/4/mode/1up", 
                h2 {  "The wall - Haushofer, Marlen, 1920-1970" }
            } 
            a {target:"_blank",href: LIBRO3, 
                h2 {  "Nueva gramática básica de la lengua española" }
            } 
        }
    }
}

#[component]
pub fn Seccion_linux_general() -> Element {
    rsx! {
        br {id: "linux"}         
        //hr { id: "lineas_separar"} 
        div {class:"linux", id: "Texto",  

            h1 {"Seccion linux general"}
            a {target:"_blank",href: "https://github.com/realECoficial/dotfiles", 
                h2 {  "Dotfiles personales" }
            } 
            a {target:"_blank",href: "https://nathan.rs/posts/dioxus-rust/#why-rust-for-front-end-development", 
                h2 {  "Frontend Rust" }
            } 
            
                 
        }
    }
}



#[component]
pub fn Seccion_util() -> Element {
    rsx! {
        br {id: "Seccion_util"}         
        //hr { id: "lineas_separar"} 
        div {class:"util", id: "Texto",  

            h1 {"Seccion util"}
            a {target:"_blank",href: "https://www.susanrigetti.com/philosophy", 
                h2 {  "Como aprender filosofia (Susan Rigetti)" }
            } 
            a {target:"_blank",href: "https://www.susanrigetti.com/physics", 
                h2 {  "Como aprender fisica (Susan Rigetti)" }
            } 
        
            a {target:"_blank",href: "https://missing.csail.mit.edu/", 
                h2 {  "Clases/lectures de M.I.T de herramientas y curiosidades." }
            } 
        
            a {target:"_blank",href: "https://github.com/ivangabriele/mistralai-client-rs/", 
                h2 {  "Mistral ia en rust :v." }
            } 

            a {target:"_blank",href: "https://odysee.com/@Luke:7/top-5-reasons-my-email-system-is:c", 
                h2 {  "MIL RAZONES POR TENER UN EMAIL SERVER." }
            } 
        }
        
    }
}

//musica
const MUSIC1: Asset = asset!("/assets/musica/Ocean girl _ perfect world.mp3");
const MUSIC2: Asset = asset!("/assets/musica/65_saves.mp3");
const MUSIC3: Asset = asset!("/assets/musica/tasty_trugictra.mp3");
const MUSIC4: Asset = asset!("/assets/musica/Substance - Them Phibez.mp3");
const MUSIC5: Asset = asset!("/assets/musica/NoRedeemingQualities.mp3");
const MUSIC6: Asset = asset!("/assets/musica/YOU.mp3");
const MUSIC7: Asset = asset!("/assets/musica/Madwreck-Ride.mp3");
const MUSIC8: Asset = asset!("/assets/musica/海神-Watazumi-.mp3");


#[component]
pub fn Seccion_musica() -> Element {


    rsx! {
        br {id:"musica"}         
        //hr { id: "lineas_separar"} 

        div {class:"musica", id: "Texto",  

            h2 {"DISCLAIMER: quisiera que no utilicen plataformas de streaming (pagadas o no) de mùsica. No se merecen tu tiempo u dinero."}
            h3 {"Si quieren escuchar musica les recomiendo que la descarguen o se la compren,"}
            h3 {"le da valor agregado."} 
            h3 {"(Esta primera parte se puede escuchar aca mismo, despues es todo youtube.)"} 
            br {}         
                
            a {target:"_blank", id: "yamete",href: "https://www.music-map.de/", 
            h2 {  "PAGINA PARA VER MAPA HISTORICO DE LAS BANDAS QUE ESCUCHAS." }
            } 
            br {}         
            ol { class:"musica-local",
                li{a {target:"_blank",href: MUSIC1,"Ocean_girl - perfect world."}}
            br {}         
                li{a {target:"_blank",href: MUSIC2,"65 save"}}
            br {}         
                li{a {target:"_blank",href: MUSIC3,"Trugictra - Tasty (Fastracker)"}}
            br {}         
                li{a {target:"_blank",href: MUSIC4,"Substance - Them Phibez"}}
            br {}         
                li{a {target:"_blank",href: MUSIC5,"40k! - NoRedeemingQualities"}}
            br {}         
                li{a {target:"_blank",href: MUSIC6,"Harito - YOU"}}
            br {}         
                li{a {target:"_blank",href: MUSIC7,"Madwreck - Ride (Fastracker)"}}
            
            br {}         
                li{a {target:"_blank",href: MUSIC8,"海神 - Watazumi"}}
            } 
             
           // seccion youtube 
             
            h2 {"Esto para abajo es de software propietario."} 
            
            a {target:"_blank", id: "yamete",href: "https://y2mate.as/en-NO0b/", 
                h3 {  "Descargalas como mp3." }
            } 
            br {}         
            ol {class:"musica-youtube",
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=-NtjNCM0Kn4&list=RD-NtjNCM0Kn4&start_radio=1","Lamp - 君が泣くなら"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=FGryJ9YTQzE&list=RDFGryJ9YTQzE&start_radio=1","Lamp - A都市の秋"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=AJdTBPuZkHU&list=RDAJdTBPuZkHU&start_radio=1","Lamp - Yume Utsutsu"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=pXuZGo60sq8&list=RDpXuZGo60sq8&start_radio=1","the pillows - Bran-new Lovesong "}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=xB2K-riHfSc&list=RDxB2K-riHfSc&start_radio=1","the pillows - LAST DINOSAUR"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=p-SWkpGKdP8&list=RDp-SWkpGKdP8&start_radio=1","American Football - Never Meant"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=DMjMuWkAnPc","Trugictra - haparanda.mod (527 kb) (Official Video)"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=aoTZmnF5Wg8","Trugictra - real_eyez.mod (346 kb) (Official Video)"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=1Mf8dn1dtK8","Trugictra - acid32.mod (234 kb) (Official Video)"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=XNYpjr4lxqU","Trugictra - shot_provoking.mod (362 kb) (Official Video)"}}
             
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=_LaxE0mxT7I","Trugictra - lenheetii.mod (413 kb) (Official Video)"}}

            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=Kh3L7u7yuyA","Susquatch - awakening at daybreak"}}
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=Kh3L7u7yuyA","save file 2"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=1Q0Fd66kgZM","Alaska - The Vortex / Invisible"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=a_6quQ994JI","Sōtaisei Riron ( 相対性理论)-Synchroniciteen (Full Album)"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=ODysC7SM_Yk","相対性理論 - 気になるあの娘"}}

            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=XOgFYjwEopo","rosenbridge"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=Q6YV_rpQ4Jk&list=PLB80A16AFA79B0379&index=3","Yu-Gi-Oh! Ultimate Masters Edition 2006 OST - Specials"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=VEUZGwwP0FY&list=PLB80A16AFA79B0379&index=9","Yu-Gi-Oh! Ultimate Masters Edition 2006 OST - Level 2 Monster"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=gpyuAT9q06c","rocket coaster"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=wHNSSbGDrfo","Owain Panchiko (DEATHMETAL Remix)"}}
            
            br {}         
                li{a {target:"_blank",href: "https://www.youtube.com/watch?v=DuWQk4eA3lU","'Linebreak' (Amiga .mod!)"}}
            

            } 
             
             
             
             
             
        }
        
    }
}
/*
#[component]
pub fn Seccion_futuro() -> Element {

    rsx! {
        br {id: "futuro"}         
        //hr { id: "lineas_separar"} 
        div { id: "Texto",  

            h1 {"Futuro"}
            a {href: "https://www.portalinmobiliario.com/venta/casa/maule-maule/10371-el-portal-de-talca-2-nva#polycard_client=recommendations_classi-portalinmobiliario-vip&reco_backend=realestate-odin&reco_model=organicos-vis&reco_client=classi-portalinmobiliario-vip&reco_item_pos=7&reco_backend_type=function&reco_id=efd7a643-2ab3-48be-9abc-d5d841e0baab", 
                h2 {  "Quiero vivir en algo asi, aunque quiero aislarme màs." }
            } 
        }
    }

    
}
*/
