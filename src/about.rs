use axum::{
    routing::{get, post},
    Form, Router,
};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use maud::{html, Markup};
use serde::Deserialize;

use crate::{
    links::{EMAIL, PHONE, WHATSAPP_LINK},
    under_construction, ClientState,
};

pub fn about_router() -> Router<ClientState> {
    return Router::new()
        .route("/about", get(about_page))
        .route("/bylaw", get(bylaw_page))
        .route("/team", get(team_page))
        .route("/contact", get(contact_page))
        .route("/contact_response", post(contact_response));
}
pub async fn about_page() -> Markup {
    html! {
        div class="bg-vertical-to-pink"{

        div class="bg-white bg-opacity-90 px-4 mx-auto max-w-screen-md relative z-5" {
            h2 class="mb-8 mt-4 text-center text-4xl tracking-tight font-extrabold text-gray-900" {
                "About Us"
            }

            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "Rooted in the rich heritage and traditions of the Tamil people, our nonprofit stands as a testament to the power of collective action and the transformative potential of community-driven initiatives. With a deep commitment to serving our members and uplifting the broader community, we strive to create a space where every individual can thrive, regardless of background or circumstance."
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "At the heart of our mission lies a dedication to preserving and promoting Tamil culture, language, and identity. Through a diverse array of programs, events, and initiatives, we seek to celebrate our heritage, instill pride in our traditions, and pass down our cultural legacy to future generations."
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "But our work extends far beyond cultural preservation – it's about empowerment, advocacy, and creating opportunities for growth and advancement. From educational initiatives and skill-building workshops to social welfare projects and community outreach efforts, we are committed to addressing the needs of our community and empowering individuals to realize their full potential."
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "Together, we are more than just a nonprofit – we are a family, united by a shared vision of a brighter future for all. Whether you're a newcomer seeking connection, a seasoned member eager to make a difference, or simply someone who believes in the power of community, there's a place for you here."
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "Join us as we embark on a journey of service, solidarity, and celebration – together, let's build a stronger, more vibrant Tamil community for generations to come."
            }

            h3 class="mb-4 text-center text-2xl tracking-tight font-extrabold text-gray-900" {
                "Mission"
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "To foster unity, cultural preservation, and empowerment within the Tamil community in the United States, enriching the lives of individuals and families through education, support, and celebration of our heritage."
            }

            h3 class="mb-2 text-center text-2xl tracking-tight font-extrabold text-gray-900" {
                "Vision"
            }
            p class="mb-4 lg:mb-8 font-light sm:text-xl" {
                "Our vision is to create a thriving and inclusive Tamil community in the US where every member feels a sense of belonging and pride in their cultural identity. We aspire to be a strong network of individuals and organizations dedicated to preserving Tamil language, traditions, and values while actively contributing to the broader American society. Through collaboration, education, and service, we seek to empower our community members to achieve their full potential and make meaningful contributions to the world around them."
            }
        }
    }
    }
}

pub async fn bylaw_page() -> Markup {
    html! {
        div class="container mx-auto px-4 py-8 text-center" {
                    h1 class="text-3xl font-bold mb-4" { "Bylaws" }

                    // Embedding PDF using iframe
                    div class="w-full h-screen overflow-y-auto border border-gray-300 shadow-lg rounded-lg" {
                        iframe
                            src="assets/img/NJTTS Bylaw.pdf"
                            class="w-full h-full"
                            title="Embedded PDF Viewer"
                            { "Your browser does not support PDF viewing. You can download the PDF file <a href=\"{pdf_url}\">here</a> instead." }
                    }
                }
    }
}

pub async fn team_page() -> Markup {
    html! {
        div class="min-h-screen flex items-center justify-center flex-col space-y-10 bg-vertical-to-pink"{
            div class=""{
                h1 class="text-center text-red-800 text-5xl md:text-7xl lg:text-9xl font-bold "{"Team"}
                a href="asset/img/our_team.jpg";
            }
        }

    }
}
pub async fn contact_page() -> Markup {
    html! {
        div class="bg-vertical-to-pink"{
        div class="max-w-7xl mx-auto p-8" {
            h1 class="text-3xl font-bold mb-6 text-center" { "Contact Us" }
            div class="md:flex md:justify-between md:items-start space-y-6 md:space-y-0" {
                div class="w-full md:w-1/3 bg-white p-8 rounded-lg shadow-lg space-y-8" {
                    div class="space-y-8" {
                        div class="flex items-center space-x-4" {
                            i class="fas fa-phone-alt fa-2x text-gray-900" {}
                            span class="text-gray-900 text-lg" { (PHONE) }
                        }
                        div class="flex items-center space-x-4" {
                            i class="fas fa-envelope fa-2x text-gray-900" {}
                            span class="text-gray-900 text-lg" { (EMAIL) }
                        }
                        div {
                            a href=(WHATSAPP_LINK) target="_blank" rel="noopener noreferrer" class="bg-green-500 text-white px-4 py-2 rounded-md hover:bg-green-600 inline-flex items-center space-x-2" {
                                i class="fab fa-whatsapp fa-lg" {}
                                span { "Join Our WhatsApp Group" }

                            }
                        }

                    }
                }
                div class="w-full md:w-2/3 bg-white p-8 rounded-lg shadow-lg" {
                    form id="contact_form" hx-post="/about/contact_response" hx-swap="innerHTML" hx-target="#response" class="space-y-6" {
                        div class="flex space-x-4" {
                            div class="w-1/2" {
                                label for="first_name" class="block text-sm font-medium text-gray-700" { "First Name" }
                                input type="text" id="first_name" name="first_name" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                            }
                            div class="w-1/2" {
                                label for="last_name" class="block text-sm font-medium text-gray-700" { "Last Name" }
                                input type="text" id="last_name" name="last_name" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                            }
                        }
                        div {
                            label for="email" class="block text-sm font-medium text-gray-700" { "Email" }
                            input type="email" id="email" name="email" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                        }
                        div {
                            label for="subject" class="block text-sm font-medium text-gray-700" { "Subject" }
                            input type="text" id="subject" name="subject" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                        }
                        div {
                            label for="message" class="block text-sm font-medium text-gray-700" { "Message" }
                            textarea id="message" name="message" rows="6" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                        }
                        div class="text-center" {
                            button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md hover:bg-orange-600" { "Submit" }
                        }
                    }
                    div id="response"{}

                }
            }
        }
        }
    }
}
#[derive(Deserialize)]
pub struct ContactFormData {
    first_name: String,
    last_name: String,
    email: String,
    subject: String,
    message: String,
}
impl std::fmt::Display for ContactFormData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "First Name: {}\nLast Name: {}\nEmail: {}\nSubject: {}\nMessage: {}",
            self.first_name, self.last_name, self.email, self.subject, self.message
        )
    }
}

pub async fn contact_response(Form(data): Form<ContactFormData>) -> Markup {
    let smtp_username = std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let smtp_server = std::env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let self_email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(smtp_username.parse().unwrap())
        .subject(format!("Contact from {}", data.email))
        .header(ContentType::TEXT_PLAIN)
        .body(data.to_string())
        .unwrap();
    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(data.email.parse().unwrap())
        .subject(format!("Contact from {}", data.email))
        .header(ContentType::TEXT_PLAIN)
        .body(data.to_string())
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay(smtp_server.as_str())
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&self_email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
    html! {
            div {
                h2 { "Thank you for contacting us, " (data.first_name) "!" }
                br;
                p { "We have received your message and will get back to you at " strong { (data.email) } " as soon as possible." }
                br;
                p { "Our team will review your message and respond accordingly. If you have any urgent queries, feel free to call us at " (PHONE)"." }
            }
            script {
                "document.getElementById('contact_form').style.display = 'none';"
            }
    }
}
