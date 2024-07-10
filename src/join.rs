use maud::{html, Markup};

use crate::links::{EMAIL, WHATSAPP_LINK};

pub async fn join_page() -> Markup {
    html! {
        div class="max-w-7xl mx-auto p-8" {
            h1 class="text-3xl font-bold mb-6 text-center" { "Join Our Community" }
            div class="md:flex md:justify-between md:items-start space-y-6 md:space-y-0" {

                // Contact Details Section
                div class="w-full md:w-1/3 bg-white p-8 rounded-lg shadow-lg space-y-8" {
                    div class="space-y-8" {
                        // Phone Number
                        div class="flex items-center space-x-4" {
                            i class="fas fa-phone-alt fa-2x text-gray-900" {}
                            span class="text-gray-900 text-lg" { "123-456-7890" }
                        }
                        // Email
                        div class="flex items-center space-x-4" {
                            i class="fas fa-envelope fa-2x text-gray-900" {}
                            span class="text-gray-900 text-lg" { (EMAIL) }
                        }
                        // WhatsApp Link with Icon
                        div class="" {
                            a href=(WHATSAPP_LINK) target="_blank" rel="noopener noreferrer" class="bg-green-500 text-white px-4 py-2 rounded-md hover:bg-green-600 inline-flex items-center space-x-2" {
                                i class="fab fa-whatsapp fa-lg" {}
                                span { "Join Our WhatsApp Group" }

                            }
                        }
                    }
                }

                // Join Form Section
                div class="w-full md:w-2/3 bg-white p-8 rounded-lg shadow-lg" {
                    form action="/submit_form" method="post" class="space-y-6" {
                        // First Name and Last Name
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

                        // Email
                        div {
                            label for="email" class="block text-sm font-medium text-gray-700" { "Email" }
                            input type="email" id="email" name="email" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                        }

                        // Phone Number
                        div {
                            label for="phone" class="block text-sm font-medium text-gray-700" { "Phone Number" }
                            input type="tel" id="phone" name="phone" class="mt-1 block w-full p-2 border border-gray-300 rounded-md" required {}
                        }

                        // Agreement Checkbox
                        div class="flex items-center" {
                            input type="checkbox" id="agree_emails" name="agree_emails" class="h-4 w-4 text-red-500 focus:ring-red-400 border-gray-300 rounded" {}
                            label for="agree_emails" class="ml-2 block text-sm text-gray-900" { "I agree to receive emails from the NJTTS team for promotions and upcoming event reminders" }
                        }
                        p class="text-center text-sm"{"NJTTS does not share personal details with any third party"}


                        // Submit Button
                        div class="text-center" {
                            button type="submit" class="bg-orange-500 text-white px-4 py-2 rounded-md hover:bg-orange-600" { "Submit" }
                        }
                    }
                }
            }
        }
    }
}
