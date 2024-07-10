use maud::{html, Markup};

pub struct School {
    name: String,
    venue: String,
    timing: String,
    email: String,
    phone: String,
    website: String,
}
impl School {
    pub fn new(
        name: &str,
        venue: &str,
        timing: &str,
        email: &str,
        phone: &str,
        website: &str,
    ) -> Self {
        School {
            name: name.to_string(),
            venue: venue.to_string(),
            timing: timing.to_string(),
            email: email.to_string(),
            phone: phone.to_string(),
            website: website.to_string(),
        }
    }
    pub fn as_markup(&self) -> Markup {
        html! {
            div class="school bg-white p-6 rounded-lg shadow-lg mb-6" {
                h2 class="text-2xl font-bold mb-2" { (self.name) }
                p class="mb-2" { strong { "Venue: " } (self.venue) }
                p class="mb-2" { strong { "Timing: " } (self.timing) }
                p class="mb-2" { strong { "Email Address: " } a href={"mailto:" (self.email)} { (self.email) } }
                p class="mb-2" { strong { "Phone Number: " } a href={"tel:" (self.phone)} { (self.phone) } }
                p class="mb-2" { strong { "Website: " } a href={(self.website)} { (self.website) } }
            }
        }
    }
}
pub async fn tamil_school_page() -> Markup {
    let schools = vec![
            School::new(
                "Vallalar Tamil School",
                "Community Middle School, Plainsboro, 95 Grovers Mill Rd, Plainsboro Township, NJ 08536",
                "Fridays 6:30 PM – 8:00 PM",
                "mail@njvallalarpalli.org",
                "1 (609) 904-3155",
                "http://njvallalarpalli.org"
            ),
            School::new(
                "Kumarasamy Tamil School",
                "Crossroads North Middle School, 635 Georges Rd, Monmouth Junction, NJ 08852",
                "Saturday 3:00 – 5:00 PM",
                "contact@sbtamilschool.org",
                "1 (732) 392-6224",
                "http://sbtamilschool.org"
            ),
            School::new(
                "Thiruvalluvar Tamil School",
                "J.P. Stevens High School, 855 Grove Ave, Edison, NJ 08820",
                "Sunday 3:00 – 4:30 PM",
                "tamilschool.edison@gmail.com",
                "1 (732) 659-4125",
                "http://www.jerseytamilacademy.org"
            ),
            School::new(
                "Kurinchi Thamizh School",
                "Public Library",
                "Saturday 9.30 AM – 11:30 AM",
                "admin@kurinchithamizhschool.org",
                "1 732-201-8882",
                "https://www.kurinchithamizhschool.org/"
            ),
            School::new(
                "Poongaa Virtual Tamil School",
                "Online Tamil Classes",
                "Sunday 10:30 AM – 12:00 PM",
                "info@poongaa.org",
                "1-609-414-7878",
                "https://poongaa.org/"
            ),
            School::new(
                "Marlboro Tamil School",
                "Sri Guruvaayoorappan Temple, Morganville, NJ 07751",
                "Friday 7:00 – 8:30 PM",
                "smohandoss@hotmail.com",
                "1 (732) 861-5242",
                "http://www.krishnatemple.org/saraswati-vidyalaya.html"
            ),
            School::new(
                "NJ Thiruvalluvar Tamil School",
                "309 Baldwin Rd, Parsippany, NJ 07054",
                "Saturday 9.45 AM – 12PM",
                "tamilschool.parsippany@gmail.com",
                "1 973-797-9826",
                "https://www.facebook.com/Parsippany-Thiruvalluvar-Tamil-School-New-Jersey-1586393398323361/"
            ),
            School::new(
                "Ilankai Tamil School (Newark)",
                "Branch Brook Library, 235 Clifton Ave, Newark, NJ 07104",
                "",
                "secretary@njtacsusa.org",
                "Kala: (973) 220-5081, Siri: (682) 235-9563, Gnana: (973) 666-2105",
                ""
            ),
            School::new(
                "Ilankai Tamil School (New Brunswick)",
                "Ruth Adams Building, New Brunswick, NJ 08901",
                "",
                "secretary@njtacsusa.org",
                "",
                ""
            ),
            School::new(
                "Tamil Jersey School",
                "Jotham W. Wakeman No. 6 Elementary School, 100 St Pauls Ave, Jersey City, NJ 07306",
                "Saturday 10:00 AM – 1:00 PM",
                "tamiljerseyschool@gmail.com",
                "1 (201) 238-7548",
                "https://www.facebook.com/TJTamilSchool"
            ),
            School::new(
                "Kamban Thamizh Palli",
                "Bridgewater-Raritan Middle School, 128 Merriwood Rd, Bridgewater Township, NJ 08807",
                "Sundays 3:30 to 5 PM",
                "support@kambantamilschool.org",
                "1 (908) 547-0759",
                "https://www.kambantamilschool.org/"
            ),
            School::new(
                "HTCS Tamil School",
                "Bridgewater Temple, 1 Balaji Temple Dr, Bridgewater, NJ 08807",
                "Sunday 3:30 – 5:00 PM",
                "tamilclass@venkateswaratemple.org",
                "1 (908) 725-4477",
                "https://www.venkateswaratemple.org/CLASSES/temp.tamil.htm"
            )
        ];

    html! {
        section class="bg-white py-8 lg:py-16 px-4 mx-auto max-w-screen-md" {
            h2 class="mb-4 text-4xl tracking-tight font-extrabold text-gray-900 text-center" {"NJ Tamil Schools"}
            @for school in schools {
                div class="h-1 bg-blue-500 my-4" {}
                (school.as_markup())
            }
        }
    }
}
