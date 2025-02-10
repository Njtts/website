use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Form, Router,
};
use dotenv::dotenv;
use links::{EMAIL, FACEBOOK_LINK, INSTAGRAM_LINK, PHONE, PHONE_LINK, WHATSAPP_LINK, YOUTUBE_LINK};
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
        .route("/byLaw", get(under_construction))
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
        .route("/sponsors", get(under_construction))
        .route("/tny25", get(newyear_redirect))
        .route("/library", get(under_construction))
        .route("/faq", get(under_construction))
        .with_state(client_state)
        .fallback(not_found);

    let listener = TcpListener::bind("0.0.0.0:3300").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
pub async fn newyear_redirect() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                //meta http-equiv="refresh" content="0; url=https://forms.gle/bkuLehsKS1Y5Gfrv5";
                meta http-equiv="refresh" content="0; url=https://forms.gle/BFTQZzQ8B19hvseJ8";
                title { "Redirecting..." }
            }
            body {
                p {
                    "If you are not redirected, "
                   // a href="https://forms.gle/bkuLehsKS1Y5Gfrv5" { "click here" }
                    a href="https://forms.gle/BFTQZzQ8B19hvseJ8" { "click here" }
                    "."
                }
            }
        }
    }
}

pub async fn vattam_page() -> Markup {
    html! {
        section class="bg-cover bg-center bg-no-repeat bg-fixed" style="background-image: url('assets/img/vasagar_vattam_bg.jpeg');" {
            div class="bg-white bg-opacity-90 py-8 lg:py-16 px-4 mx-auto max-w-screen-md relative z-5" {
                h2 class="mb-4 text-center text-4xl tracking-tight font-extrabold text-gray-900" { "Welcome to NJ Vasagar Vattam" }

                p class="mb-8 lg:mb-16 font-light text-center sm:text-xl" {
                    "Since January 21, 2024, our members have gathered every Wednesday to explore Tamil short stories through virtual discussions. But is reading storybooks necessary? As GK Chesterton aptly put it in 1901, \"Literature is a luxury; Fiction is a necessity.\""
                }

                div class="text-center mb-8" {
                    img src="assets/img/books.jpeg" alt="QR Code" class="mx-auto mb-4" {}
                }

                p class="mb-8 lg:mb-16 font-light text-center sm:text-xl" {
                    "Fiction is more than just entertainment; it's a gateway to new worlds, emotions, and experiences. Through stories, we can dream, play, and live vicariously, exploring human emotion in a unique and impactful way. From childhood tales to adult escapades, fiction has been an indispensable part of human societies, providing solace, inspiration, and a brief escape from reality."
                }

                p class="mb-8 lg:mb-16 font-light text-center sm:text-xl" {
                    "We've delved into over 30 captivating stories, each offering a fresh perspective and insight into the human experience."
                }

                div class="text-center mb-8" {
                    img src="assets/img/vattam_w.jpeg" alt="QR Code" class="mx-auto mb-4" {}
                    a href="https://chat.whatsapp.com/FjyUCpSVjIQDv04xSnBAZc" target="_blank" rel="noopener noreferrer" class="text-green-500 hover:underline" { "Join Our Whatsapp Group!" }
                }

                p class="mb-8 lg:mb-16 font-light text-center sm:text-xl" {
                    "If you wish to be part of our literary discussion, join us using the QR code above."
                }
            }
        }
    }
}

async fn events_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10 bg-vertical-to-pink text-center text-xl md:text-2xl lg:text-3xl" {
            // Upcoming Events Section
                h2 class="text-red-700 font-semibold" { "Upcoming Events" }
             p class="text-sm md:text-base lg:text-lg xl:text-xl 2xl:text-2xl font-semibold cursor-pointer"{
                                   a href="https://njtts.org/tny25" class="text-blue-500 underline"{"Click here to register for Tamil New Year 2025 Event"}
            }
             p class="text-sm md:text-base lg:text-lg xl:text-xl 2xl:text-2xl font-semibold cursor-pointer"{
                                   a href="" class="text-blue-500 underline"{"TTS 2025 Event"
                    br;
                                   "Parambhariyavillaiyattu"
                    br;

                                   "Tamil New year"
                    br;
                                   "Science Fair"
                    br;
                                   "Father's Day"
                    br;
                                   "Camping"
                    br;
                                   "Diwali"
                    br;
                                   "PechuPotti"
            }}

              div  class="w-full max-w-xs md:max-w-sm lg:max-w-md object-cover" {
                        img src="assets/img/posters/TTS-Parambhariyavillaiyattu_2025.jpg" class="w-full h-full object-cover" alt="TTS-Parambhariyavillaiyattu_2025" {}}

             a href="https://njtts.org/tny25"  class="transition-transform transform hover:scale-105 relative" {
                        img src="assets/img/posters/TTS-Tamil_New_Year_2025.jpg" class="w-full max-w-xs md:max-w-sm lg:max-w-md object-cover" alt="Tamil New Year 2025 poster" {}
                    }

                }

                // Add more events here as needed

            // Past Events Section
            div class="space-y-4 px-4" {
                h2 class="text-red-700 font-semibold" { "Past Events - 2024" }
                div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 justify-center items-center"{
                div class="flex flex-row space-x-4"{
                            img src="assets/img/posters/fall_festival.jpg" class="w-full max-w-xs md:max-w-sm lg:max-w-md object-cover" alt="fall festival Poster" {}
                    }
             div class="flex flex-row space-x-4"{
                            img src="assets/img/posters/Diwali-2024.jpg" class="w-full max-w-xs md:max-w-sm lg:max-w-md object-cover" alt="Diwali-2024" {}
                    }
                    div class="transition-transform transform hover:scale-105 relative aspect-w-3 aspect-h-4"{
                        img src="assets/img/posters/camping.jpeg" class="w-full h-full object-cover" alt="Camping poster" {}
                    }
                    div class="transition-transform transform hover:scale-105 relative aspect-w-3 aspect-h-4"{
                        img src="assets/img/posters/sciencefair.jpeg" class="w-full h-full object-cover" alt="Science Fair poster" {}
                }

                    div class="transition-transform transform hover:scale-105 relative aspect-w-3 aspect-h-4"{
                        img src="assets/img/posters/fathersday.jpeg" class="w-full h-full object-cover" alt="Father's Day poster" {}
            }


                    a href="https://drive.google.com/file/d/1tbAjLiUVyjootpo2b6hAeE4RH9vDQWy4/view?ts=66bbd394" class="transition-transform transform hover:scale-105 relative aspect-w-3 aspect-h-4"{
                        img src="assets/img/posters/villaiyattu.jpeg" class="w-full h-full object-cover" alt="Villaiyattu poster" {}
                    }

                    // Add more past events here as needed
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
                    div class="flex flex-col py-2 text-sm"{
                        div class="flex justify-center flex-grow"{


                             div class="hover:text-blue-700 hover:underline px-4 py-2 rounded"
                               hx-get="/home" hx-trigger="click" hx-target="#page" {
                                 "Home"
                             }
                             div class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                               hx-get="/events" hx-trigger="click" hx-target="#page" {
                                 "Events"
                             }

                              div class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
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
                                 div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-30"{
                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/about/about" hx-trigger="click" hx-target="#page" {
                                         "About Us"
                                     }

                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/bylaw" hx-trigger="click" hx-target="#page" {
                                         "Bylaw"
                                     }
                                     div class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/about/team" hx-trigger="click" hx-target="#page" {
                                         "Our Team"
                                     }
                                     div class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/faq" hx-trigger="click" hx-target="#page" {
                                         "FAQ's"
                                     }
                                     div class="hover:text-blue-700 px-4 py-2"
                                       hx-get="/sponsors" hx-trigger="click" hx-target="#page" {
                                         "Our Sponsors"
                                     }



                                     div class="hover:text-blue-700 px-4 py-2"
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
                                 div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-30"{

                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/vattam" hx-trigger="click" hx-target="#page" {
                                         "NJ Vasagar Vattam"
                                     }


                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/walking_club" hx-trigger="click" hx-target="#page" {
                                         "Walking Club"
                                     }
                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/hiking_club" hx-trigger="click" hx-target="#page" {
                                         "Hiking Club"
                                     }
                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/running_club" hx-trigger="click" hx-target="#page" {
                                         "Running Club"
                                     }
                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/tamil_school" hx-trigger="click" hx-target="#page" {
                                         "NJ Tamil Schools"
                                     }
                                     div class="hover:text-blue-700 hover:underline px-4 py-2"
                                       hx-get="/library" hx-trigger="click" hx-target="#page" {
                                         "Tamil Library"
                                     }


                                 }

                             }

                             }





                    }
                    div class="flex-grow flex items-center justify-center"{
                            a hx-get="/join" hx-trigger="click" hx-target="#page" class="text-white bg-orange-600 hover:bg-red-600 px-3 py-1 rounded-md text-sm font-small" {
                                "Join Us"
                            }
                        }
                }

            div class="flex w-full items-center justify-between bg-orange-600 md:hidden text-white" {
                div class="ml-3 flex space-x-4"{
                    a href=(FACEBOOK_LINK) class="" { i class="fab fa-facebook" {} }
                    a href=(INSTAGRAM_LINK) class="" { i class="fab fa-instagram" {} }
                    a href=(WHATSAPP_LINK) class="" { i class="fab fa-whatsapp" {} }
                        a href=(YOUTUBE_LINK) class="" { i class="fab fa-youtube" {} }



                }
                    // Email icon and clickable email link
                    a href=(format!("mailto:{EMAIL}")) class="text-white hover:text-gray-200 flex items-center" {
                        i class="fas fa-envelope mr-1" {}
                        p class="text-sm" { (EMAIL) }
                    }
                    // Phone icon and clickable phone link
                    a href=(PHONE_LINK) class="text-white hover:text-gray-200 flex items-center mr-2" {
                        i class="fas fa-phone-alt mr-1" {}
                        p class="text-sm" { (PHONE)}
                    }

            }
    }
}
pub fn sponsors_markup() -> Markup {
    html! {
       // div class="flex flex-col items-center bg-vertical-to-white relative mx-[10%] my-[5%]" {
            div class="flex flex-col items-center relative mx-[10%] my-[5%]" {

            div class="relative w-full" {
                div class="carousel overflow-hidden relative" {
                    div class="carousel-items flex transition-transform duration-500 ease-in-out" id="carousel-items" {
                        // Repeat the following div for each sponsor image
                        div class="carousel-item min-w-full flex-shrink items-center" {
                           // img src="assets/img/sponsor.jpg" class="w-full h-auto" alt="Sponsor 1" {}
                         /*p class="mb-2  text-xl tracking-tight font-extrabold animate-blink-color text-center" {
                                                     "1000$ - Gold Sponsors"
                           }
                              p class="mb-2  text-xl tracking-tight font-extrabold animate-blink-color text-center" {
                                                          "300$ per Annum - Annual Sponsors"
                                }
                                       p class="mb-2  text-xl tracking-tight font-extrabold animate-blink-color text-center" {
                                                                   "500$(discounted price) - 2years"
                                         }*/
                            img src="assets/img/Sponsors-Package.jpg" class="w-full h-auto" alt="Sponsor 2" {}

                        }
                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Trinity-sponsor1.jpg"   class="w-full h-auto" alt="Sponsor 2" {}
                            //p class="text-sm md:text-base lg:text-lg xl:text-xl 50xl:text-50xl animate-blink-color text-center" { "Gold Sponsor" }
                         p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"{ " Gold Sponsor " }
                       //p class="mb-5 text-5l tracking-tight font-extrabold size=100 animate-blink-color text-center" { "Gold Sponsor" }

                            // h4 class="mb-5  text-xxxl tracking-tight font-extrabold animate-blink-color text-center"{
                                                    // "Gold Sponsor"}
                        }
                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Finminds-sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {" Gold Sponsor "}
                        }

                         div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Financial-advisor-sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {" Silver Sponsor "}
                        }
                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Veda-dentist.png" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {" Silver Sponsor "}
                        }

                           div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/VELCAB-Broonze-sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {"Bronze Sponsor"}
                        }

                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/NMLS-sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {"Bronze Sponsor"}
                        }
                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Pen-Drapes-sponsor.jpg" class="w-full h-auto" alt="Sponsor 2" {}
                             img src="assets/img/saree-drape-logo.png" class="w-full h-auto" alt="Sponsor 2" {}
                             // p class="mb-2  text-xxxl tracking-tight font-extrabold animate-blink-color text-center size=100"
                             p class="text-lg md:text-2xl lg:text-5xl xl:text-5xl 100xl:text-100xl font-bold animate-blink-color text-center"
                            {"Bronze Sponsor"}
                        }
                        div class="carousel-item min-w-full flex-shrink items-center" {
                            img src="assets/img/Sponsors-Package.jpg" class="w-full h-auto" alt="Sponsor 2" {}
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

async fn under_construction() -> Markup {
    html! {
        div class="w-full flex flex-col items-center mt-8 bg-vertical-to-pink" {
            // h2 class="text-3xl font-bold text-center text-gray-800 mb-8" { "Annual Sponsors" }
            // img src="assets/img/sponsor-collage.jpg" class="h-auto" alt="Sponsor Collage" {}
             // Image 1
            a href="assets/img/under construction.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
            img src="assets/img/under construction.jpg" alt="Photo 1" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }
        }

    }
}
async fn home() -> Markup {
    html! {
        div class="z-0 relative" {
            div class="w-full relative" {

                img src="assets/img/home_bg.jpeg" class="w-full h-auto" alt="Background Image" {}
                        div class=
                            "absolute bottom-5 left-1/2
                                transform
                                -translate-x-1/2
                                -translate-y-1/4
                                sm:-translate-y-1/3
                                md:-translate-y-2/3
                                flex flex-col items-center justify-center text-center font-taviraj "

                            {

                                p class="text-lg md:text-2xl lg:text-3xl xl:text-4xl 2xl:text-5xl font-bold animate-blink-color" {
                                    "🎉  Tamil New year 🎉"
                                }
                                p class="text-lg md:text-2xl lg:text-3xl xl:text-4xl 2xl:text-5xl font-bold animate-blink-color" {
                                    "29th Mar 2025"
                                }
                                p class="text-sm md:text-base lg:text-lg xl:text-xl 2xl:text-2xl font-semibold cursor-pointer"{
                                    a href="https://njtts.org/tny25" class="text-blue-500 underline"{"Click here"} " to register"

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

            '<div id="event" class="font-roboto sm:text-xl md:text-2xl lg:text-3xl"><a class="text-blue-600 underline" hx-get="/events" hx-trigger="click" hx-target="#page" >TTS-Fall Festival</a>: Sep 15th at Smith Field Park, Parsippany</div>',

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
        nav id="navb" class=" bg-pink-to-white p-4"{

            div class="container mx-auto flex items-center justify-between gap-6"{
                div class = "flex items-center gap-4"
                    hx-get="/home" hx-trigger="click" hx-target="#page" {
                        img src="assets/img/logo.jpg" class="h-28 w-28 rounded-full object-cover" alt="Logo" {}




                }
               div class="flex justify-center flex-grow"{


                    div class="hover:text-blue-700 hover:underline px-4 py-2 rounded"
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
                        div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-30"{
                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/about/about" hx-trigger="click" hx-target="#page" {
                                "About Us"
                            }

                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/byLaw" hx-trigger="click" hx-target="#page" {
                                "Bylaw"
                            }
                            div class="hover:text-blue-700 px-4 py-2"
                              hx-get="/about/team" hx-trigger="click" hx-target="#page" {
                                "Our Team"
                            }
                            div class="hover:text-blue-700 px-4 py-2"
                              hx-get="/faq" hx-trigger="click" hx-target="#page" {
                                "FAQ's"
                            }
                            div class="hover:text-blue-700 px-4 py-2"
                              hx-get="/sponsors" hx-trigger="click" hx-target="#page" {
                                "Our Sponsors"
                            }
                            div class="hover:text-blue-700 px-4 py-2"
                              hx-get="/about/contact" hx-trigger="click" hx-target="#page" {
                                "Contact Us"
                            }
                        }
                    }


                    div class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
                      hx-get="/events" hx-trigger="click" hx-target="#page" {
                        "Events"
                    }
                    div class=" hover:text-blue-700 hover:underline px-4 py-2 rounded"
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
                        div class="hidden dropdown-menu absolute bg-gray-100 rounded-b-lg pb-2 w-48 flex flex-col z-30"{

                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/vattam" hx-trigger="click" hx-target="#page" {
                                "NJ Vasagar Vattam"
                            }


                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/walking_club" hx-trigger="click" hx-target="#page" {
                                "Walking Club"
                            }
                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/hiking_club" hx-trigger="click" hx-target="#page" {
                                "Hiking Club"
                            }
                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/running_club" hx-trigger="click" hx-target="#page" {
                                "Running Club"
                            }
                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/tamil_school" hx-trigger="click" hx-target="#page" {
                                "NJ Tamil Schools"
                            }
                            div class="hover:text-blue-700 hover:underline px-4 py-2"
                              hx-get="/library" hx-trigger="click" hx-target="#page" {
                                "Tamil Library"
                            }

                        }

                    }
                    }
                    div {
                            div hx-get="/join" hx-trigger="click" hx-target="#page" class="text-white bg-orange-600 hover:bg-red-600 px-6 py-3 rounded-lg text-lg font-medium" {
                                "Join Us"
                            }
                        }





            }
        }
        script {
            (PreEscaped(r#"
                // Select all dropdown toggle buttons
                const dropdownToggles = document.querySelectorAll(".dropdown-toggle");

                dropdownToggles.forEach((toggle) => {
                    toggle.addEventListener("click", () => {
                        // Find the next sibling element which is the dropdown menu
                        const dropdownMenu = toggle.nextElementSibling;

                        // Toggle the 'hidden' class to show or hide the dropdown menu
                        if (dropdownMenu.classList.contains("hidden")) {
                            // Hide any open dropdown menus before showing the new one
                            document.querySelectorAll(".dropdown-menu").forEach((menu) => {
                                menu.classList.add("hidden");
                            });

                            dropdownMenu.classList.remove("hidden");
                        } else {
                            dropdownMenu.classList.add("hidden");
                        }
                    });

                    // Close the dropdown when a link inside it is clicked
                    const links = toggle.nextElementSibling.querySelectorAll("div");
                    links.forEach(link => {
                        link.addEventListener("click", () => {
                            toggle.nextElementSibling.classList.add("hidden");
                        });
                    });
                });

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
