use maud::{html, Markup};

pub async fn walking_page() -> Markup {
    html! {
        section class="bg-white py-8 lg:py-16 px-4 mx-auto max-w-screen-md" {
            div class="text-center mb-8" {
                img src="assets/img/hiking_code.jpeg" alt="QR Code" class="mx-auto mb-4" {}
                a href="your_whatsapp_link_here" target="_blank" rel="noopener noreferrer" class="text-green-500 hover:underline" { "Join Our Whatsapp Group!" }
            }
            h2 class="mb-4 text-center text-4xl tracking-tight font-extrabold text-gray-900" {"Step into Serenity: Introducing Our TTSWalking Club"}
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Nestled amidst the breathtaking landscapes of our community lies an invitation to embark on a journey unlike any other – welcome to our TTS Hiking Club! With a passion for adventure and a love for the great outdoors, our club is a vibrant community of individuals united by the desire to explore the wonders of nature, one trail at a time. Whether you're a seasoned hiker or just beginning to discover the joys of trekking, there's a place for you here."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Our club isn't just about reaching the summit; it's about forging connections, sharing stories, and experiencing the beauty of our surroundings in the company of like-minded enthusiasts. From serene woodland paths to rugged mountain trails, we offer a diverse range of excursions tailored to suit every skill level and interest."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Join us as we breathe in the crisp mountain air, soak in panoramic vistas, and immerse ourselves in the tranquility of nature's embrace. Together, let's create memories, conquer new heights, and cultivate a deep appreciation for the remarkable landscapes that surround us."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Whether you seek adventure, camaraderie, or simply a chance to unplug and recharge, our Hiking Club is here to guide you on an unforgettable journey of discovery. Lace up your boots, pack your sense of adventure, and let's hit the trails together!"
            }
        }
    }
}

pub async fn hiking_page() -> Markup {
    html! {
        section class="bg-white py-8 lg:py-16 px-4 mx-auto max-w-screen-md" {
            h2 class="mb-4 text-center text-4xl tracking-tight font-extrabold text-gray-900" { "Explore, Connect, and Conquer Nature: Welcome to Our TTS Hiking Club" }

            div class="text-center mb-8" {
                img src="assets/img/hiking_code.jpeg" alt="QR Code" class="mx-auto mb-4" {}
                a href="https://chat.whatsapp.com/FjyUCpSVjIQDv04xSnBAZc" target="_blank" rel="noopener noreferrer" class="text-green-500 hover:underline" { "Join Our Whatsapp Group!" }
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
               "In the gentle rhythm of each step lies a pathway to serenity, and our TTS Walking Club is your guide to this tranquil journey. Nestled within the heart of our vibrant community, our Walking Club invites you to embrace the simple joys of walking and the profound connections it fosters – with nature, with others, and with oneself. Whether you're seeking a leisurely stroll or a brisk stride, our club offers a sanctuary for walkers of all paces and preferences."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Here, amidst the beauty of our surroundings, we celebrate the art of slow living and the rejuvenating power of movement. From charming neighborhood streets to scenic trails winding through parks and green spaces, our expeditions are as diverse as they are delightful."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
               "Join us as we embark on a voyage of discovery, meandering through hidden pathways, uncovering hidden gems, and savoring the beauty of each moment along the way. Whether you're seeking solace in solitude or companionship amidst kindred spirits, our Walking Club offers a welcoming community where connections flourish and friendships bloom."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "So lace up your shoes, leave your worries behind, and let's wander together as we explore the beauty that surrounds us, one step at a time."
            }
        }
    }
}

pub async fn running_page() -> Markup {
    html! {
        section class="bg-white py-8 lg:py-16 px-4 mx-auto max-w-screen-md" {
            div class="text-center mb-8" {
                img src="assets/img/hiking_code.jpeg" alt="QR Code" class="mx-auto mb-4" {}
                a href="your_whatsapp_link_here" target="_blank" rel="noopener noreferrer" class="text-green-500 hover:underline" { "Join Our Whatsapp Group!" }
            }
            h2 class="mb-4 text-center text-4xl tracking-tight font-extrabold text-gray-900" { "Explore, Connect, and Conquer Nature: Welcome to Our TTS Hiking Club" }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
               "In the gentle rhythm of each step lies a pathway to serenity, and our TTS Walking Club is your guide to this tranquil journey. Nestled within the heart of our vibrant community, our Walking Club invites you to embrace the simple joys of walking and the profound connections it fosters – with nature, with others, and with oneself. Whether you're seeking a leisurely stroll or a brisk stride, our club offers a sanctuary for walkers of all paces and preferences."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "Here, amidst the beauty of our surroundings, we celebrate the art of slow living and the rejuvenating power of movement. From charming neighborhood streets to scenic trails winding through parks and green spaces, our expeditions are as diverse as they are delightful."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
               "Join us as we embark on a voyage of discovery, meandering through hidden pathways, uncovering hidden gems, and savoring the beauty of each moment along the way. Whether you're seeking solace in solitude or companionship amidst kindred spirits, our Walking Club offers a welcoming community where connections flourish and friendships bloom."
            }
            p class="mb-8 lg:mb-16 font-light text-center text-gray-500 sm:text-xl" {
                "So lace up your shoes, leave your worries behind, and let's wander together as we explore the beauty that surrounds us, one step at a time."
            }
        }
    }
}
