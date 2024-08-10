use maud::{html, Markup, PreEscaped};

pub async fn gallery_page() -> Markup {
    html! {
        div class="flex justify-center items-center py-12 px-4 sm:px-6 lg:px-8 bg-vertical-to-pink" {
            // Gallery container
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                div class="grid grid-cols-1 gap-8 lg:grid-cols-3 lg:gap-10" {
                    // Year sidebar
                    div class="hidden lg:block lg:col-span-1" {
                        div class="sticky top-16 space-y-4"{
                            a href="#2024" class="text-gray-700 hover:text-gray-900 hover:underline block" {"2024"}
                        }
                    }

                    // Photo grid
                    div class="lg:col-span-2 space-y-8 sm:space-y-0" {
                        // 2024 section
                        div id="2024" class="space-y-4" {
                            h2 class="text-2xl font-bold text-gray-900" {"2024"}

                            // Event 1
                            div class="space-y-4" {
                                h3 class="text-xl font-semibold text-gray-800" {"Father's Day Celebration"}

                                div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 border rounded-lg shadow-lg bg-white" {
                                    // Image 1
                                    a href="assets/img/image1.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image1.jpg" alt="Photo 1" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }

                                    // Image 2
                                    a href="assets/img/image2.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image2.jpg" alt="Photo 2" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }

                                    // Image 3
                                    a href="assets/img/image3.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image3.jpg" alt="Photo 3" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }

                                    // Image 4 (optional)
                                    a href="assets/img/image4.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image4.jpg" alt="Photo 4" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }
                                    a href="assets/img/image5.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image5.jpg" alt="Photo 5" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }
                                    a href="assets/img/image6.jpg" target="_blank" rel="noopener noreferrer" class="group relative" {
                                        img src="assets/img/image6.jpg" alt="Photo 6" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                                    }


                                }
                            }


                            // // Event 2
                            // div class="space-y-4" {
                            //     h3 class="text-xl font-semibold text-gray-800" {"Event 2 Title"}
                            //     div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 border rounded-lg shadow-lg bg-white" {
                            //         // Image 1
                            //         div class="group relative" {
                            //             img src="https://via.placeholder.com/600x400?text=Photo+1" alt="Photo 1" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                            //         }

                            //         // Image 2
                            //         div class="group relative" {
                            //             img src="https://via.placeholder.com/600x400?text=Photo+2" alt="Photo 2" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                            //         }

                            //         // Image 3
                            //         div class="group relative" {
                            //             img src="https://via.placeholder.com/600x400?text=Photo+3" alt="Photo 3" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                            //         }

                            //         // Image 4 (optional)
                            //         div class="group relative" {
                            //             img src="https://via.placeholder.com/600x400?text=Photo+4" alt="Photo 4" class="rounded-lg object-cover w-full h-full transform transition-transform duration-300 group-hover:scale-105";
                            //         }
                            //     }
                            // }

                            // Add more events for 2024
                        }

                        // Add more year sections as needed
                    }
                }
            }
        }
    }
}
