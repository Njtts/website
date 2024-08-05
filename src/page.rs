use crate::{
    links::{EMAIL, FACEBOOK_LINK, INSTAGRAM_LINK, PHONE, PHONE_LINK, WHATSAPP_LINK},
    navbar, strings,
};
use maud::{html, Markup, DOCTYPE};
fn body(content: Markup) -> Markup {
    html! {
        body {
            script src="assets/js/vendor/htmx.min.js" {}
            script src="assets/js/vendor/modernizr-3.11.2.min.js" {}
            script src="assets/js/plugins.js" {}
            script src="assets/js/main.js" {}
            link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;700&display=swap" rel="stylesheet";

            style {
                "
                .hamburger {
                    cursor: pointer;
                }
                #side-menu {
                    transform: translateX(-100%);
                }
                #side-menu.show {
                    transform: translateX(0);
                }"
            }

            script {
                "function toggleSideMenu() {
                    var sideMenu = document.getElementById('side-menu');
                    sideMenu.classList.toggle('show');
                }"
            }

            div class="min-h-screen flex flex-col" {
                // Hamburger Menu for small screens
                div class="md:hidden fixed top-4 right-4 flex items-center justify-between p-4 z-50" {
                            button onclick="toggleSideMenu()" class="hamburger" {
                                svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M4 6h16M4 12h16m-7 6h7" {}
                                }
                            }
                        }
                // Navbar for larger screens
                div class="hidden md:flex flex-col" {
                    (navbar_markup())
                }

                // Side Menu for small screens

                div id="side-menu" class="fixed top-0 left-0 h-full w-64 bg-vertical-to-pink py-2 px-4 flex flex-col space-y-4 transform -translate-x-full transition-transform duration-300 ease-in-out z-50"{
                    (mobile_side_menu_markup())
            }

                div class="flex-grow" {
                    (content)
                }

                (footer_markup())
            }
        }
    }
}
pub fn mobile_side_menu_markup() -> Markup {
    html! {
        div class="flex flex-col h-full p-4" {
            // Close button
            button onclick="toggleSideMenu()" class="mb-4" {
                svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" {
                    path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" {}
                }
            }
            div class="flex justify-center mb-4" {
                        img src="assets/img/logo.jpeg" class="h-28 w-28 rounded-full object-cover" alt="Logo" {}
                    }

            // Dropdowns
            div class="flex flex-col space-y-4 text-lg" {
                a class="rounded"
                  hx-get="/home" hx-trigger="click" hx-target="#page" {
                    "Home"
                }


                div class="relative" {
                    button type="button" class="dropdown-toggle text-lg flex items-center gap-2 rounded" {
                        span { "About" }
                        svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6" {
                            path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" {}
                        }
                    }
                    div class="hidden dropdown-menu absolute bg-white rounded-b-lg pb-2 w-full flex flex-col z-10" {
                        a hx-get="/about/bylaw" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Bylaw" }
                        a hx-get="/about/team" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Our Team" }
                        a hx-get="/about/faq" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "FAQ's" }
                        a hx-get="/sponsors" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Our Sponsors" }
                        a hx-get="/about/contact" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Contact Us" }
                    }
                }
                a class="text-lg rounded"
                                      hx-get="/events" hx-trigger="click" hx-target="#page" {
                                        "Events"
                                    }
                                    a class="text-lg rounded"
                                      hx-get="/gallery" hx-trigger="click" hx-target="#page" {
                                        "Gallery"
                                    }
                div class="relative" {
                    button type="button" class="dropdown-toggle text-lg flex items-center gap-2 rounded" {
                        span { "Community" }
                        svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="size-6" {
                            path stroke-linecap="round" stroke-linejoin="round" d="m19.5 8.25-7.5 7.5-7.5-7.5" {}
                        }
                    }
                    div class="hidden dropdown-menu absolute bg-white rounded-b-lg pb-2 w-full flex flex-col z-10" {
                        a hx-get="/library" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Tamil Library" }
                                                a hx-get="/vattam" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "NJ Vasagar Vattam" }
                                                a hx-get="/walking_club" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Walking Club" }
                                                a hx-get="/hiking_club" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Hiking Club" }
                                                a hx-get="/running_club" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "Running Club" }
                                                a hx-get="/tamil_school" hx-trigger="click" hx-target="#page" class=" px-4 py-2" { "NJ Tamil Schools" }
                    }
                }
            }


           div class="flex-grow" {}

            div class="flex space-x-4 mb-4" {
                a href=(FACEBOOK_LINK) class="text-lg" { i class="fab fa-facebook" {} }
                a href=(INSTAGRAM_LINK) class="text-lg" { i class="fab fa-instagram" {} }
                a href=(WHATSAPP_LINK) class="text-lg" { i class="fab fa-whatsapp" {} }
            }

            div class="flex flex-col" {
                a href=(format!("mailto:{EMAIL}")) class="flex items-center text-lg" {
                    i class="fas fa-envelope mr-2" {}
                    p { (EMAIL) }
                }
                a href=(PHONE_LINK) class="flex items-center text-lg" {
                    i class="fas fa-phone-alt mr-2" {}
                    p { (PHONE) }
                }
            }

        }
    }
}

pub fn navbar_markup() -> Markup {
    html! {
        div class="bg-orange-600 text-white py-2 px-4 flex justify-between items-center" {
            div class="flex items-center space-x-4" {
                a href=(FACEBOOK_LINK) class="text-white hover:text-gray-200" { i class="fab fa-facebook" {} }
                a href=(INSTAGRAM_LINK) class="text-white hover:text-gray-200" { i class="fab fa-instagram" {} }
                a href=(WHATSAPP_LINK) class="text-white hover:text-gray-200" { i class="fab fa-whatsapp" {} }
            }
            div class="flex items-center space-x-4"{
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
        div hx-trigger="load" hx-get="/navbar"{}

    }
}

pub fn footer_markup() -> Markup {
    html! {
        footer class="bg-gray-800 text-white py-8" {
            div class="container mx-auto px-4" {
                div class="flex flex-col md:flex-row md:justify-between md:items-start" {

                    // First Column: Contact Information
                    div class="mb-6 md:mb-0 md:w-1/3 text-center md:text-left" {
                        h2 class="text-2xl font-bold mb-2" { "Contact Information" }
                        div class="text-gray-400 flex flex-col space-y-2 mt-4" {
                            a href=(PHONE_LINK) class="hover:text-white flex items-center justify-center space-x-2 md:justify-start" {
                                i class="fas fa-phone text-lg" {}
                                span { (PHONE) }
                            }
                            a href=(format!("mailto:{EMAIL}")) class="hover:text-white flex items-center justify-center space-x-2 md:justify-start" {
                                i class="fas fa-envelope text-lg" {}
                                span { (EMAIL) }
                            }
                            div class="flex space-x-4 justify-center md:justify-start mt-4" {
                                a href=(FACEBOOK_LINK) class="hover:text-white flex items-center justify-center w-12 h-12 bg-gray-700 rounded-full" {
                                    i class="fab fa-facebook text-2xl" {}
                                }
                                a href=(INSTAGRAM_LINK) class="hover:text-white flex items-center justify-center w-12 h-12 bg-gray-700 rounded-full" {
                                    i class="fab fa-instagram text-2xl" {}
                                }
                                a href=(WHATSAPP_LINK) class="hover:text-white flex items-center justify-center w-12 h-12 bg-gray-700 rounded-full" {
                                    i class="fab fa-whatsapp text-2xl" {}
                                }
                            }
                        }
                    }

                    // Second Column: Quick Links
                    div class="mb-6 md:mb-0 md:w-1/3 text-center md:text-left" {
                        h2 class="text-2xl font-bold mb-2" { "Quick Links" }
                        div class="text-gray-400 flex flex-col space-y-2 mt-4" {
                            a hx-get="/about" hx-trigger="click" hx-target="#page" class="hover:text-white" { "About Us" }
                            a hx-get="/contact" hx-trigger="click" hx-target="#page" class="hover:text-white" { "Contact Us" }
                            a hx-get="/faq" hx-trigger="click" hx-target="#page" class="hover:text-white" { "FAQ's" }
                        }
                    }

                    // Third Column: Get Involved
                    div class="text-center md:text-left md:w-1/3" {
                        h2 class="text-2xl font-bold mb-2" { "Get Involved" }
                        div class="text-gray-400 flex flex-col space-y-2 mt-4" {
                            a hx-get="/sponsors" hx-trigger="click" hx-target="#page" class="hover:text-white" { "Sponsors" }
                            a hx-get="/membership" hx-trigger="click" hx-target="#page" class="hover:text-white" { "Membership" }
                            a hx-get="/donate" hx-trigger="click" hx-target="#page" class="hover:text-white" { "Donate" }
                        }
                    }
                }

                hr class="border-gray-700 my-6" {}

                div class="flex flex-col items-center md:flex-row md:justify-between"{
                    p class="text-gray-400 text-sm md:text-base mb-4 md:mb-0" { "© 2024 NJ Thiruvalluvar Tamil Sangam. All rights reserved." }
                }
            }
        }
    }
}

fn head(title: &str, desc: &str, url: &str) -> Markup {
    html! {
        head {
            meta charset=(strings::UTF8);
            title { (title) }
            meta name=(strings::DESCRIPTION) content=(desc);
            meta name=(strings::VIEWPORT) content=(strings::VIEWPORT_CONTENT);
            meta property="og:title" content=(title);
            meta property="og:type" content=(strings::WEBSITE);
            meta property="og:url" content=(url);
            meta property="og:image" content="";
            link rel="manifest" href="assets/site.webmanifest";
            link rel="apple-touch-icon" href="assets/icon.png";
            link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0-beta3/css/all.min.css";

            link rel = "stylesheet" href="assets/css/main.css";

            meta name="theme-color" content="#fafafa";
        }
    }
}

pub(crate) fn page(content: Markup) -> Markup {
    let host = "njtts.org";
    let title = "njtts";
    let desc = "Tamil sangam website";
    let lang = "en";

    html! {
        (DOCTYPE)
        html class="no-js" lang=(lang) {
            (head(title, desc, host))
            (body(content))
        }
    }
}
