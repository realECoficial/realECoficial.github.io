use dioxus::prelude::*;
use crate::Route;
#[component]
pub fn NavBar() -> Element {
    rsx! {
        
        div { id: "title",
           
            img {id:"logo", src: "https://avatars.githubusercontent.com/u/136939439?v=4"} 
            h2 {id:"logo-text", "Ec web!"}
            Link { to: Route::Presentacion {},
                h4 { "Principal" }
            }
            Link { to: Route::About_me {}, 
                h4 {"About Me"}
            } // <------- add this Link
            Link { to: Route::Seccion_estudio_rust {}, 
                h4 {"Estudios"}
            } // <------- add this Link
           /* 
            Link { to: Route::Codigo {}, 
                h3 {"Codigo :v"}
            } // <------- add this Link
           */ 
            Link { to: Route::Seccion_linux_general {}, 
                h4 {"Linux"}
            } // <------- add this Link
            Link { to: Route::Seccion_util {}, 
                h4 {"Util"}
            } // <------- add this Link
            Link { to: Route::Seccion_libros {}, 
                h4 {"Libros"}
            } // <------- add this Link
            Link { to: Route::Seccion_musica {}, 
                h4 {"Musica"}
            } // <------- add this Link
            Link { to: Route::Wallpapers {}, 
                h4 {"Wallpapers"}
            } // <------- add this Link
/*            Link { to: Route::Seccion_futuro  {}, 
                h3 {"Futuro"}
            } */ // <------- add this Link
        }
        Outlet::<Route> {}
    }
}
