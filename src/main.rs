use axum::{
    routing::{get, post},
    Form, Router,
};
use maud::{html, Markup, PreEscaped, DOCTYPE};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

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

#[tokio::main]
async fn main() {
    let serve_dir = ServeDir::new("src/static");

    let app = Router::new()
        .nest_service("/assets", serve_dir)
        .route("/", get(index))
        .route("/navbar", get(navbar))
        .route("/home", get(home))
        .route("/events", get(events_page))
        .route("/gallery", get(gallery_page))
        .route("/bylaw", get(bylaw_page))
        .route("/team", get(team_page))
        .route("/contact", get(contact_page))
        .route("/response", post(contact))
        .route("/hiking_club", get(hiking_page))
        .route("/walking_club", get(walking_page))
        .route("/running_club", get(running_page))
        .route("/vattam", get(vattam_page))
        .route("/tamil_school", get(tamil_school_page))
        .route("/join", get(join_page))
        .route("/sponsors", get(sponsors_page))
        .fallback(not_found);

    let listener = TcpListener::bind("0.0.0.0:3300").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn vattam_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10"{
            div class=""{
                h1 class="text-center text-red-800 text-5xl md:text-7xl lg:text-9xl font-bold "{"Vattam"}
            }
        }

    }
}

async fn events_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10"{
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
fn sponsors_markup() -> Markup {
    html! {
        div class="w-full flex flex-col items-center mt-8" {
            h2 class="text-3xl font-bold text-center text-gray-800 mb-8" { "2024 Annual Sponsors" }
            img src="assets/img/sponsor-collage.jpg" class="h-auto" alt="Sponsor Collage" {}
        }


    }
}
async fn sponsors_page() -> Markup {
    sponsors_markup()
}
async fn home() -> Markup {
    html! {
        div class=" z-0 relative space-y-8" {

            div class="w-full relative" {
                img src="assets/img/home_bg.jpeg" class="w-full h-auto opacity-75" alt="Background Image" {}

                div class="absolute top-1/3 left-1/2 transform -translate-x-1/2 -translate-y-1/3 p-4 text-center bg-blue-500 rounded-full" {
                    div id="notificationBanner" class="font-bold transition-opacity duration-500 opacity-100 text-3xl" {}
                }
            }
            (sponsors_markup())

        }
        (PreEscaped(r#"
        <script>
          const events = [
            '<div class="text-5xl">Tamil School Registration <br> for the upcoming 2024-2025 year <br> is now <a href="https://docs.google.com/document/d/1FAnGN5_vdso0mnCB9KZlDQJQlOnFeeHatiF2CViLgwQ/edit?usp=sharing" class="text-white underline"> open!</a></div>',
            '<div class="text-5xl"><a class="text-white underline" href="https://chat.whatsapp.com/FjyUCpSVjIQDv04xSnBAZc" >Hiking Club</a>: June 22, Saturday 6.15 am, <a class="text-white underline" href="https://www.alltrails.com/trail/us/new-jersey/normanook-tower-via-appalachian-trail-loop?sh=bcs169">Normanook Tower via Appalachian Trail Loop</a></div>',
            'Run on Wednesday'
          ];
          let eventIndex = 0;

          function updateBanner() {
            const banner = document.getElementById('notificationBanner');
            banner.style.opacity = 0; // Start fade out

            setTimeout(() => {
              banner.innerHTML = events[eventIndex];
              banner.style.opacity = 1; // Fade in
              eventIndex = (eventIndex + 1) % events.length;
            }, 500); // Wait for fade out to complete before changing text
          }

          setInterval(updateBanner, 5000); // Change event every 5 seconds
          updateBanner(); // Initial update
        </script>
        "#))
    }
}

async fn navbar() -> Markup {
    html! {
        nav id="navb" class=" p-4"{

            div class="container mx-auto flex items-center justify-between gap-6"{
                div class = "flex items-center gap-4"{

                    img src="assets/img/logo.jpeg" class="h-28 w-28 rounded-full object-cover" alt="Logo" {}

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
                              hx-get="/bylaw" hx-trigger="click" hx-target="#page" {
                                "Bylaw"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/team" hx-trigger="click" hx-target="#page" {
                                "Our Team"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/faq" hx-trigger="click" hx-target="#page" {
                                "FAQ's"
                            }
                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/sponsors" hx-trigger="click" hx-target="#page" {
                                "Our Sponsors"
                            }



                            a class="hover:text-blue-700 px-4 py-2"
                              hx-get="/contact" hx-trigger="click" hx-target="#page" {
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
                              hx-get="/tamil_school" hx-trigger="click" hx-target="#page" {
                                "NJ Tamil Schools"
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
