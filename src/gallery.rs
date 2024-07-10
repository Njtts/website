use maud::{html, Markup, PreEscaped};

pub async fn gallery_page() -> Markup {
    html! {
        div class="flex justify-center items-center py-12 px-4 sm:px-6 lg:px-8" {
            // Gallery container
            div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8" {
                div class="grid grid-cols-1 gap-8 lg:grid-cols-3 lg:gap-10" {
                    // Year sidebar
                    div class="hidden lg:block lg:col-span-1" {
                        div class="sticky top-16 space-y-4"{
                            a href="#2024" class="text-gray-700 hover:text-gray-900 hover:underline block" {"2024"}
                            a href="#2023" class="text-gray-700 hover:text-gray-900 hover:underline block" {"2023"}
                            a href="#2022" class="text-gray-700 hover:text-gray-900 hover:underline block" {"2022"}
                            // Add more years as needed
                        }
                    }

                    // Photo grid
                    div class="lg:col-span-2 space-y-8 sm:space-y-0" {
                        // 2024 section
                        div id="2024" class="space-y-4" {
                            h2 class="text-2xl font-bold text-gray-900" {"2024"}
                            div class="grid grid-cols-2 sm:grid-cols-3 gap-4" {
                                // Image 1
                                div class="space-y-2" {
                                    img src="https://lh3.googleusercontent.com/drive-viewer/AKGpihalqTBP-9htcJ5jQUlw8VvzwNAlomVh6iAxx1pxnCVvi2XYiSLRhJvTPqsZqmEIwu3MdU3IyJbg-kQhn9Fb6u4sCCvsgpvxsVE=s1600-rw-v1" alt="Photo 1" class="rounded-lg object-cover w-full h-full";
                                    p class="text-center text-gray-700" {"Event 1 Title"}
                                }

                                // Image 2
                                div class="space-y-2" {
                                    img src="https://via.placeholder.com/600x400?text=Photo+2" alt="Photo 2" class="rounded-lg object-cover w-full h-full";
                                    p class="text-center text-gray-700" {"Event 2 Title"}
                                }

                                // Add more images and titles for 2024
                            }
                        }

                        // Add more year sections as needed
                    }
                }
            }
        }
    }
}

pub fn cc() -> Markup {
    let script = r#"
            <script>
                var carousel = document.getElementById('default-carousel');

                if (carousel) {
                    const items = carousel.querySelectorAll('[data-carousel-item]');
                    const indicators = carousel.querySelectorAll('[data-carousel-slide-to]');
                    const prevButton = carousel.querySelector('[data-carousel-prev]');
                    const nextButton = carousel.querySelector('[data-carousel-next]');
                    const duration = 5000; // Slide duration in milliseconds
                    let currentIndex = 0;
                    let intervalId = null;

                    // Function to show current slide
                    const showSlide = (index) => {
                        items.forEach(item => item.classList.add('hidden'));
                        indicators.forEach(indicator => indicator.setAttribute('aria-current', 'false'));

                        items[index].classList.remove('hidden');
                        indicators[index].setAttribute('aria-current', 'true');
                    };

                    // Function to go to next slide
                    const nextSlide = () => {
                        currentIndex = (currentIndex + 1) % items.length;
                        showSlide(currentIndex);
                    };

                    // Function to go to previous slide
                    const prevSlide = () => {
                        currentIndex = (currentIndex - 1 + items.length) % items.length;
                        showSlide(currentIndex);
                    };

                    // Function to start automatic sliding
                    const startCarousel = () => {
                        intervalId = setInterval(nextSlide, duration);
                    };

                    // Function to stop automatic sliding
                    const stopCarousel = () => {
                        clearInterval(intervalId);
                    };

                    // Event listeners for prev and next buttons
                    prevButton.addEventListener('click', () => {
                        prevSlide();
                        stopCarousel();
                        startCarousel();
                    });

                    nextButton.addEventListener('click', () => {
                        nextSlide();
                        stopCarousel();
                        startCarousel();
                    });

                    // Event listeners for indicators
                    indicators.forEach((indicator, index) => {
                        indicator.addEventListener('click', () => {
                            currentIndex = index;
                            showSlide(currentIndex);
                            stopCarousel();
                            startCarousel();
                        });
                    });

                    showSlide(0);
                    // Start automatic sliding
                    startCarousel();
                }
            </script>
        "#;
    html! {
        div id="default-carousel" class="relative w-full" data-carousel="slide" {
            // Carousel wrapper
            div class="relative min-h-screen overflow-hidden rounded-lg" {
                // Slide 1
                div class="hidden duration-700 ease-in-out" data-carousel-item {
                    img src="assets/img/home_bg.jpeg" class="absolute w-full h-full object-contain top-0 left-0" alt="Slide 1" {}
                }
                // Slide 2
                div class="hidden duration-700 ease-in-out" data-carousel-item {
                    img src="assets/img/hiking_code.jpeg" class="absolute w-full h-full object-contain top-0 left-0" alt="Slide 2" {}
                }
                // Slide 3
                div class="hidden duration-700 ease-in-out" data-carousel-item {
                    img src="assets/img/home_bg.jpeg" class="absolute w-full h-full object-contain top-0 left-0" alt="Slide 3" {}
                }
                // Slide 4
                div class="hidden duration-700 ease-in-out" data-carousel-item {
                    img src="assets/img/home_bg.jpeg" class="absolute w-full h-full object-contain top-0 left-0" alt="Slide 4" {}
                }
                // Slide 5
                div class="hidden duration-700 ease-in-out" data-carousel-item {
                    img src="assets/img/home_bg.jpeg" class="absolute w-full h-full object-contain top-0 left-0" alt="Slide 5" {}
                }
            }
            // Slider indicators
            div class="absolute bottom-5 left-1/2 transform -translate-x-1/2 flex space-x-3 rtl:space-x-reverse" {
                button type="button" class="w-3 h-3 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" aria-label="Slide 1" data-carousel-slide-to="0" {}
                button type="button" class="w-3 h-3 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" aria-label="Slide 2" data-carousel-slide-to="1" {}
                button type="button" class="w-3 h-3 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" aria-label="Slide 3" data-carousel-slide-to="2" {}
                button type="button" class="w-3 h-3 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" aria-label="Slide 4" data-carousel-slide-to="3" {}
                button type="button" class="w-3 h-3 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" aria-label="Slide 5" data-carousel-slide-to="4" {}
            }
            // Slider controls - Previous
            button type="button" class="absolute top-1/2 -translate-y-1/2 left-4 z-10 flex items-center justify-center w-10 h-10 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" data-carousel-prev {
                svg class="w-4 h-4 text-white rtl:rotate-180" viewBox="0 0 6 10" xmlns="http://www.w3.org/2000/svg" {
                    path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 1 1 5l4 4";
                }
            }
            // Slider controls - Next
            button type="button" class="absolute top-1/2 -translate-y-1/2 right-4 z-10 flex items-center justify-center w-10 h-10 rounded-full bg-gray-300 hover:bg-gray-400 focus:outline-none" data-carousel-next {
                svg class="w-4 h-4 text-white" viewBox="0 0 6 10" xmlns="http://www.w3.org/2000/svg" {
                    path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m1 9 4-4-4-4";
                }
            }
        }
       (PreEscaped(script.to_string()))
    }
}
