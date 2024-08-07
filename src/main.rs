use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use dotenv::dotenv;
use links::{EMAIL, FACEBOOK_LINK, INSTAGRAM_LINK, PHONE, PHONE_LINK, WHATSAPP_LINK};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use mongodb::{bson::Document, options::ClientOptions, Client};
use tokio::net::TcpListener;
use tower_http::{add_extension::AddExtensionLayer, services::ServeDir};

mod about;
mod club;
mod gallery;
mod join;
mod links;
mod page;
mod strings;
mod tamil_school;
use about::*;
use club::*;
use gallery::*;
use join::*;
use tamil_school::*;
#[derive(Clone)]
pub struct ClientState {
    client: Arc<Client>,
}
async fn connect_to_mongodb() -> mongodb::error::Result<Client> {
    let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI not set");
    let mut client_options = ClientOptions::parse(&uri).await?;
    client_options.app_name = Some("MyApp".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let client = connect_to_mongodb()
        .await
        .expect("Failed to connect to MongoDB");
    let client_state = ClientState {
        client: Arc::new(client),
    };

    let serve_dir = ServeDir::new("src/static");

    let app = Router::new()
        .nest_service("/assets", serve_dir)
        .nest("/about", about_router())
        .route("/", get(index))
        .route("/navbar", get(navbar))
        .route("/home", get(home))
        .route("/events", get(events_page))
        .route("/gallery", get(gallery_page))
        .route("/hiking_club", get(hiking_page))
        .route("/walking_club", get(walking_page))
        .route("/running_club", get(running_page))
        .route("/vattam", get(vattam_page))
        .route("/tamil_school", get(tamil_school_page))
        .route("/enrollment_guide", get(enrollment_guide))
        .route("/join", get(join_page))
        .route("/join_response", post(join_response))
        .route("/sponsors", get(sponsors_page))
        .with_state(client_state)
        .fallback(not_found);

    let listener = TcpListener::bind("0.0.0.0:3300").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn vattam_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10 bg-vertical-to-pink"{
            div class=""{
                h1 class="text-center text-red-800 text-5xl md:text-7xl lg:text-9xl font-bold "{"Vattam"}
            }
        }
    }
}

async fn events_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10 bg-vertical-to-pink"{
            div class=""{
                h1 class="text-center text-red-800 text-5xl md:text-7xl lg:text-9xl font-bold "{"Events"}
            }
        }

    }
}

async fn index() -> Markup {
    let content = html! {

        div class="bg-gray-50" id="page" hx-trigger="load" hx-get="/home"{}
    };
    page::page(content)
}
pub fn mobile_navbar() -> Markup {
    html! {
        div class="flex bg-pink-to-white md:hidden" {
                    img src="assets/img/logo.jpg" class="h-28 w-28 rounded-full object-cover" alt="Logo" {}
                    div class="flex flex-col"{
                        div class="flex justify-center flex-grow"{


                             a class="hover:text-blue-700 hover:underline px-4 py-2 rounded"
                               hx-get="/home" hx-trigger="click" hx-target="#page" {
                                 "Home"
                             }
                             a class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                               hx-get="/events" hx-trigger="click" hx-target="#page" {
                                 "Events"
                             }

                              a class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                                hx-get="/gallery" hx-trigger="click" hx-target="#page" {
                                  "Gallery"
                              }



                        }
                        div class="flex justify-center flex-grow"{
                             div class="relative"{
                                 button type="button" class="dropdown-toggle py-2 px-3 hover:bg-gray-100 flex items-center gap-2 rounded"{
                                     span class="select-none"{
                                         "About"
                                     }
                                     svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"{
                                       path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"{}
                                     }
                                 }
                                 div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-10"{
                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/about/bylaw" hx-trigger="click" hx-target="#page" {
                                         "Bylaw"
                                     }
                                     a class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/about/team" hx-trigger="click" hx-target="#page" {
                                         "Our Team"
                                     }
                                     a class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/about/faq" hx-trigger="click" hx-target="#page" {
                                         "FAQ's"
                                     }
                                     a class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/sponsors" hx-trigger="click" hx-target="#page" {
                                         "Our Sponsors"
                                     }



                                     a class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/about/contact" hx-trigger="click" hx-target="#page" {
                                         "Contact Us"
                                     }

                                 }
                             }



                             div class="relative"{

                                 button type="button" class="dropdown-toggle py-2 px-3 hover:bg-gray-100 flex items-center gap-2 rounded"{
                                     span class="select-none"{
                                         "Community"
                                     }
                                     svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"{
                                       path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"{}
                                     }
                                 }
                                 div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-10"{
                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/library" hx-trigger="click" hx-target="#page" {
                                         "Tamil Library"
                                     }

                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/vattam" hx-trigger="click" hx-target="#page" {
                                         "NJ Vasagar Vattam"
                                     }


                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/walking_club" hx-trigger="click" hx-target="#page" {
                                         "Walking Club"
                                     }
                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/hiking_club" hx-trigger="click" hx-target="#page" {
                                         "Hiking Club"
                                     }
                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/running_club" hx-trigger="click" hx-target="#page" {
                                         "Running Club"
                                     }
                                     a class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/tamil_school" hx-trigger="click" hx-target="#page" {
                                         "NJ Tamil Schools"
                                     }

                                 }

                             }

                             }





                    }
                    div class="flex-grow flex items-center justify-center"{
                            a hx-get="/join" hx-trigger="click" hx-target="#page" class="text-white bg-orange-600 hover:bg-red-600 px-6 py-3 rounded-lg text-lg font-medium" {
                                "Join Us"
                            }
                        }



                }

        div class="flex w-full bg-orange-600 md:hidden justify-between"{
            div class="flex items-center space-x-4 justify-between bg-orange-600 text-white ml-2" {
                a href=(FACEBOOK_LINK) class="" { i class="fab fa-facebook" {} }
                a href=(INSTAGRAM_LINK) class="" { i class="fab fa-instagram" {} }
                a href=(WHATSAPP_LINK) class="" { i class="fab fa-whatsapp" {} }
            }
            div class="flex items-center space-x-4 mr-2"{
                // Email icon and text on the right side
                div class="flex items-center space-x-4"{
                                // Email icon and clickable email link
                                a href=(format!("mailto:{EMAIL}")) class="text-white hover:text-gray-200 flex items-center" {
                                    i class="fas fa-envelope mr-1" {}
                                    p class="text-sm" { (EMAIL) }
                                }
                                // Phone icon and clickable phone link
                                a href=(PHONE_LINK) class="text-white hover:text-gray-200 flex items-center" {
                                    i class="fas fa-phone-alt mr-1" {}
                                    p class="text-sm" { (PHONE)}
                                }
                            }
            }




        }


    }
}
pub fn sponsors_markup() -> Markup {
    html! {
        div class="w-full flex flex-col items-center mt-8 bg-vertical-to-pink relative" {

            div class="relative w-full max-w-3xl" {
                div class="carousel overflow-hidden relative" {
                    div class="carousel-items flex transition-transform duration-500 ease-in-out" id="carousel-items" {
                        // Repeat the following div for each sponsor image
                        div class="carousel-item min-w-full flex-shrink-0" {
                            img src="assets/img/sponsor.jpg" class="w-full h-auto" alt="Sponsor 1" {}
                        }
                        div class="carousel-item min-w-full flex-shrink-0" {
                            img src="assets/img/sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                        }
                        div class="carousel-item min-w-full flex-shrink-0" {
                            img src="assets/img/sponsor.jpg" class="w-full h-auto" alt="Sponsor 3" {}
                        }
                    }
                }
            }

            // Carousel Script
            script {
                (PreEscaped(r#"
                    let index = 0;
                    const interval = 3000; // 3 seconds

                    function showSlide(i) {
                        const slides = document.querySelectorAll('.carousel-item');
                        const totalSlides = slides.length;

                        // Wrap around if index is out of bounds
                        index = (i + totalSlides) % totalSlides;

                        // Move carousel items
                        const offset = -index * 100;
                        document.getElementById('carousel-items').style.transform = `translateX(${offset}%)`;
                    }

                    function nextSlide() {
                        showSlide(index + 1);
                    }

                    // Automatically advance slides every interval
                    setInterval(nextSlide, interval);

                    // Initial slide display
                    showSlide(index);
                "#))
            }
        }
    }
}

async fn sponsors_page() -> Markup {
    html! {
        div class="w-full flex flex-col items-center mt-8 bg-vertical-to-pink" {
            h2 class="text-3xl font-bold text-center text-gray-800 mb-8" { "2024 Annual Sponsors" }
            img src="assets/img/sponsor-collage.jpg" class="h-auto" alt="Sponsor Collage" {}
        }

    }
}
async fn home() -> Markup {
    html! {
        div class="z-0 relative" {
            div class="w-full relative" {

                img src="assets/img/home_bg.jpeg" class="w-full h-auto" alt="Background Image" {}
                div class="bg-slate-200 text-center h-20 flex items-center justify-center" id="notificationBanner" {
                            div class="font-bold transition-opacity duration-500 opacity-100 text-3xl" {
                            }
                        }


            }
            (sponsors_markup())
        }
        (PreEscaped(r##"
        <style>
          #notificationBanner {
            transition: opacity 0.5s ease-in-out; /* Smooth opacity transition */
          }
        </style>
        <script>
        const events = [
            '<div id="event" class="font-roboto sm:text-xl md:text-2xl lg:text-3xl">Tamil School Registration for the upcoming 2024-2025 year is now <a hx-get="/enrollment_guide" hx-trigger="click" hx-target="#page" class="text-blue-600 underline">open!</a></div>',

            '<div id="event" class="font-roboto sm:text-xl md:text-2xl lg:text-3xl"><a class="text-blue-600 underline" href="https://chat.whatsapp.com/FjyUCpSVjIQDv04xSnBAZc">Hiking Club</a>: June 22, Saturday 6.15 am, <a class="text-blue-600 underline" href="https://www.alltrails.com/trail/us/new-jersey/normanook-tower-via-appalachian-trail-loop?sh=bcs169">Normanook Tower via Appalachian Trail Loop</a></div>',

            '<div id="event" class="font-roboto sm:text-xl md:text-2xl lg:text-3xl">Run on Wednesday</div>'
        ];

          let eventIndex = 0;

          function updateBanner() {
            const banner = document.getElementById('notificationBanner');
            banner.style.opacity = 0; // Start fade out

            setTimeout(() => {
              banner.innerHTML = events[eventIndex];
              banner.style.opacity = 1; // Fade in
              eventIndex = (eventIndex + 1) % events.length;
             htmx.process(document.getElementById('event'));

            }, 500); // Wait for fade out to complete before changing text
          }

          setInterval(updateBanner, 5000); // Change event every 5 seconds
          updateBanner(); // Initial update
        </script>
        "##))
    }
}
async fn navbar() -> Markup {
    html! {
        nav id="navb" class=" p-4"{

            div class="container mx-auto flex items-center justify-between gap-6"{
                div class = "flex items-center gap-4"{

                    img src="assets/img/logo.jpg" class="h-28 w-28 rounded-full object-cover" alt="Logo" {}

                }
               div class="flex justify-center flex-grow"{


                    a class="hover:text-blue-700 hover:underline px-4 py-2 rounded"
                      hx-get="/home" hx-trigger="click" hx-target="#page" {
                        "Home"
                    }
                    div class="relative"{
                        button type="button" class="dropdown-toggle py-2 px-3 hover:bg-gray-100 flex items-center gap-2 rounded"{
                            span class="select-none"{
                                "About"
                            }
                            svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"{
                              path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"{}
                            }
                        }
                        div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-10"{
                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/about/bylaw" hx-trigger="click" hx-target="#page" {
                                "Bylaw"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/about/team" hx-trigger="click" hx-target="#page" {
                                "Our Team"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/about/faq" hx-trigger="click" hx-target="#page" {
                                "FAQ's"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/sponsors" hx-trigger="click" hx-target="#page" {
                                "Our Sponsors"
                            }



                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/about/contact" hx-trigger="click" hx-target="#page" {
                                "Contact Us"
                            }

                        }
                    }


                    a class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                      hx-get="/events" hx-trigger="click" hx-target="#page" {
                        "Events"
                    }
                    a class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                      hx-get="/gallery" hx-trigger="click" hx-target="#page" {
                        "Gallery"
                    }

                    div class="relative"{
                        button type="button" class="dropdown-toggle py-2 px-3 hover:bg-gray-100 flex items-center gap-2 rounded"{
                            span class="select-none"{
                                "Community"
                            }
                            svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6"{
                              path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5"{}
                            }
                        }
                        div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-10"{
                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/library" hx-trigger="click" hx-target="#page" {
                                "Tamil Library"
                            }

                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/vattam" hx-trigger="click" hx-target="#page" {
                                "NJ Vasagar Vattam"
                            }


                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/walking_club" hx-trigger="click" hx-target="#page" {
                                "Walking Club"
                            }
                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/hiking_club" hx-trigger="click" hx-target="#page" {
                                "Hiking Club"
                            }
                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/running_club" hx-trigger="click" hx-target="#page" {
                                "Running Club"
                            }
                            a class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/tamil_school" hx-trigger="click" hx-target="#page" {
                                "NJ Tamil Schools"
                            }
                        }
                    }
                    }
                    div {
                            a hx-get="/join" hx-trigger="click" hx-target="#page" class="text-white bg-orange-600 hover:bg-red-600 px-6 py-3 rounded-lg text-lg font-medium" {
                                "Join Us"
                            }
                        }





            }
        }
        script {
                (PreEscaped(r#"
                      // Select all dropdown toggle buttons
                      const dropdownToggles = document.querySelectorAll(".dropdown-toggle")

                      dropdownToggles.forEach((toggle) => {
                        toggle.addEventListener("click", () => {
                        console.log("clicked toggle");
                          // Find the next sibling element which is the dropdown menu
                          const dropdownMenu = toggle.nextElementSibling

                          // Toggle the 'hidden' class to show or hide the dropdown menu
                          if (dropdownMenu.classList.contains("hidden")) {
                            // Hide any open dropdown menus before showing the new one
                            document.querySelectorAll(".dropdown-menu").forEach((menu) => {
                              menu.classList.add("hidden")
                            })

                            dropdownMenu.classList.remove("hidden")
                          } else {
                            dropdownMenu.classList.add("hidden")
                          }
                        })
                      })
                      // Clicking outside of an open dropdown menu closes it

                      window.addEventListener("click", function (e) {
                        // Check if the clicked element or any of its ancestors contain the class 'dropdown-toggle'
                        if (!e.target.closest(".dropdown-toggle")) {
                          // Iterate through each dropdown menu
                          document.querySelectorAll(".dropdown-menu").forEach((menu) => {
                            // Check if the clicked element is not within the dropdown menu
                            if (!menu.contains(e.target)) {
                              // Hide the dropdown menu
                              menu.classList.add("hidden");
                            }
                          });
                        }
                      });


                "#))
            }
    }
}
async fn not_found() -> Markup {
    html! {
        html lang="en" {
            head {
                meta charset=(strings::UTF8);
                title { (strings::NOT_FOUND_TITLE) }
                meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
                style { (strings::NOT_FOUND_STYLE) }
            }
            body {
                h1 { (strings::NOT_FOUND_TITLE) }
                p { (strings::NOT_FOUND_MESSAGE) }
            }
            (PreEscaped(strings::NOT_FOUND_COMMENT))
        }
    }
}
