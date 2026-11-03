//! All site content as typed static data. Components only render what is here.

#[derive(Clone, Copy)]
pub struct SocialLink {
    pub label: &'static str,
    pub url: &'static str,
    pub icon: &'static str,
}

#[derive(Clone, Copy)]
pub struct NavLink {
    pub label: &'static str,
    /// Target element id, without the leading `#`.
    pub anchor: &'static str,
}

#[derive(Clone, Copy)]
pub struct PageIntro {
    pub intro: &'static str,
    pub highlight: &'static str,
    pub title: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct FloatingIcon {
    pub position: &'static str,
    pub glyph: Glyph,
}

/// A Font Awesome class, or a 24×24 SVG path for brands Font Awesome lacks.
#[derive(Clone, Copy)]
pub enum Glyph {
    FontAwesome(&'static str),
    Svg(&'static str),
}

// Brand paths from Simple Icons 13.21.0 (CC0), https://simpleicons.org
pub const CLAUDE_PATH: &str = "m4.7144 15.9555 4.7174-2.6471.079-.2307-.079-.1275h-.2307l-.7893-.0486-2.6956-.0729-2.3375-.0971-2.2646-.1214-.5707-.1215-.5343-.7042.0546-.3522.4797-.3218.686.0608 1.5179.1032 2.2767.1578 1.6514.0972 2.4468.255h.3886l.0546-.1579-.1336-.0971-.1032-.0972L6.973 9.8356l-2.55-1.6879-1.3356-.9714-.7225-.4918-.3643-.4614-.1578-1.0078.6557-.7225.8803.0607.2246.0607.8925.686 1.9064 1.4754 2.4893 1.8336.3643.3035.1457-.1032.0182-.0728-.164-.2733-1.3539-2.4467-1.445-2.4893-.6435-1.032-.17-.6194c-.0607-.255-.1032-.4674-.1032-.7285L6.287.1335 6.6997 0l.9957.1336.419.3642.6192 1.4147 1.0018 2.2282 1.5543 3.0296.4553.8985.2429.8318.091.255h.1579v-.1457l.1275-1.706.2368-2.0947.2307-2.6957.0789-.7589.3764-.9107.7468-.4918.5828.2793.4797.686-.0668.4433-.2853 1.8517-.5586 2.9021-.3643 1.9429h.2125l.2429-.2429.9835-1.3053 1.6514-2.0643.7286-.8196.85-.9046.5464-.4311h1.0321l.759 1.1293-.34 1.1657-1.0625 1.3478-.8804 1.1414-1.2628 1.7-.7893 1.36.0729.1093.1882-.0183 2.8535-.607 1.5421-.2794 1.8396-.3157.8318.3886.091.3946-.3278.8075-1.967.4857-2.3072.4614-3.4364.8136-.0425.0304.0486.0607 1.5482.1457.6618.0364h1.621l3.0175.2247.7892.522.4736.6376-.079.4857-1.2142.6193-1.6393-.3886-3.825-.9107-1.3113-.3279h-.1822v.1093l1.0929 1.0686 2.0035 1.8092 2.5075 2.3314.1275.5768-.3218.4554-.34-.0486-2.2039-1.6575-.85-.7468-1.9246-1.621h-.1275v.17l.4432.6496 2.3436 3.5214.1214 1.0807-.17.3521-.6071.2125-.6679-.1214-1.3721-1.9246L14.38 17.959l-1.1414-1.9428-.1397.079-.674 7.2552-.3156.3703-.7286.2793-.6071-.4614-.3218-.7468.3218-1.4753.3886-1.9246.3157-1.53.2853-1.9004.17-.6314-.0121-.0425-.1397.0182-1.4328 1.9672-2.1796 2.9446-1.7243 1.8456-.4128.164-.7164-.3704.0667-.6618.4008-.5889 2.386-3.0357 1.4389-1.882.929-1.0868-.0062-.1579h-.0546l-6.3385 4.1164-1.1293.1457-.4857-.4554.0608-.7467.2307-.2429 1.9064-1.3114Z";
pub const POSTGRESQL_PATH: &str = "M23.5594 14.7228a.5269.5269 0 0 0-.0563-.1191c-.139-.2632-.4768-.3418-1.0074-.2321-1.6533.3411-2.2935.1312-2.5256-.0191 1.342-2.0482 2.445-4.522 3.0411-6.8297.2714-1.0507.7982-3.5237.1222-4.7316a1.5641 1.5641 0 0 0-.1509-.235C21.6931.9086 19.8007.0248 17.5099.0005c-1.4947-.0158-2.7705.3461-3.1161.4794a9.449 9.449 0 0 0-.5159-.0816 8.044 8.044 0 0 0-1.3114-.1278c-1.1822-.0184-2.2038.2642-3.0498.8406-.8573-.3211-4.7888-1.645-7.2219.0788C.9359 2.1526.3086 3.8733.4302 6.3043c.0409.818.5069 3.334 1.2423 5.7436.4598 1.5065.9387 2.7019 1.4334 3.582.553.9942 1.1259 1.5933 1.7143 1.7895.4474.1491 1.1327.1441 1.8581-.7279.8012-.9635 1.5903-1.8258 1.9446-2.2069.4351.2355.9064.3625 1.39.3772a.0569.0569 0 0 0 .0004.0041 11.0312 11.0312 0 0 0-.2472.3054c-.3389.4302-.4094.5197-1.5002.7443-.3102.064-1.1344.2339-1.1464.8115-.0025.1224.0329.2309.0919.3268.2269.4231.9216.6097 1.015.6331 1.3345.3335 2.5044.092 3.3714-.6787-.017 2.231.0775 4.4174.3454 5.0874.2212.5529.7618 1.9045 2.4692 1.9043.2505 0 .5263-.0291.8296-.0941 1.7819-.3821 2.5557-1.1696 2.855-2.9059.1503-.8707.4016-2.8753.5388-4.1012.0169-.0703.0357-.1207.057-.1362.0007-.0005.0697-.0471.4272.0307a.3673.3673 0 0 0 .0443.0068l.2539.0223.0149.001c.8468.0384 1.9114-.1426 2.5312-.4308.6438-.2988 1.8057-1.0323 1.5951-1.6698zM2.371 11.8765c-.7435-2.4358-1.1779-4.8851-1.2123-5.5719-.1086-2.1714.4171-3.6829 1.5623-4.4927 1.8367-1.2986 4.8398-.5408 6.108-.13-.0032.0032-.0066.0061-.0098.0094-2.0238 2.044-1.9758 5.536-1.9708 5.7495-.0002.0823.0066.1989.0162.3593.0348.5873.0996 1.6804-.0735 2.9184-.1609 1.1504.1937 2.2764.9728 3.0892.0806.0841.1648.1631.2518.2374-.3468.3714-1.1004 1.1926-1.9025 2.1576-.5677.6825-.9597.5517-1.0886.5087-.3919-.1307-.813-.5871-1.2381-1.3223-.4796-.839-.9635-2.0317-1.4155-3.5126zm6.0072 5.0871c-.1711-.0428-.3271-.1132-.4322-.1772.0889-.0394.2374-.0902.4833-.1409 1.2833-.2641 1.4815-.4506 1.9143-1.0002.0992-.126.2116-.2687.3673-.4426a.3549.3549 0 0 0 .0737-.1298c.1708-.1513.2724-.1099.4369-.0417.156.0646.3078.26.3695.4752.0291.1016.0619.2945-.0452.4444-.9043 1.2658-2.2216 1.2494-3.1676 1.0128zm2.094-3.988-.0525.141c-.133.3566-.2567.6881-.3334 1.003-.6674-.0021-1.3168-.2872-1.8105-.8024-.6279-.6551-.9131-1.5664-.7825-2.5004.1828-1.3079.1153-2.4468.079-3.0586-.005-.0857-.0095-.1607-.0122-.2199.2957-.2621 1.6659-.9962 2.6429-.7724.4459.1022.7176.4057.8305.928.5846 2.7038.0774 3.8307-.3302 4.7363-.084.1866-.1633.3629-.2311.5454zm7.3637 4.5725c-.0169.1768-.0358.376-.0618.5959l-.146.4383a.3547.3547 0 0 0-.0182.1077c-.0059.4747-.054.6489-.115.8693-.0634.2292-.1353.4891-.1794 1.0575-.11 1.4143-.8782 2.2267-2.4172 2.5565-1.5155.3251-1.7843-.4968-2.0212-1.2217a6.5824 6.5824 0 0 0-.0769-.2266c-.2154-.5858-.1911-1.4119-.1574-2.5551.0165-.5612-.0249-1.9013-.3302-2.6462.0044-.2932.0106-.5909.019-.8918a.3529.3529 0 0 0-.0153-.1126 1.4927 1.4927 0 0 0-.0439-.208c-.1226-.4283-.4213-.7866-.7797-.9351-.1424-.059-.4038-.1672-.7178-.0869.067-.276.1831-.5875.309-.9249l.0529-.142c.0595-.16.134-.3257.213-.5012.4265-.9476 1.0106-2.2453.3766-5.1772-.2374-1.0981-1.0304-1.6343-2.2324-1.5098-.7207.0746-1.3799.3654-1.7088.5321a5.6716 5.6716 0 0 0-.1958.1041c.0918-1.1064.4386-3.1741 1.7357-4.4823a4.0306 4.0306 0 0 1 .3033-.276.3532.3532 0 0 0 .1447-.0644c.7524-.5706 1.6945-.8506 2.802-.8325.4091.0067.8017.0339 1.1742.081 1.939.3544 3.2439 1.4468 4.0359 2.3827.8143.9623 1.2552 1.9315 1.4312 2.4543-1.3232-.1346-2.2234.1268-2.6797.779-.9926 1.4189.543 4.1729 1.2811 5.4964.1353.2426.2522.4522.2889.5413.2403.5825.5515.9713.7787 1.2552.0696.087.1372.1714.1885.245-.4008.1155-1.1208.3825-1.0552 1.717-.0123.1563-.0423.4469-.0834.8148-.0461.2077-.0702.4603-.0994.7662zm.8905-1.6211c-.0405-.8316.2691-.9185.5967-1.0105a2.8566 2.8566 0 0 0 .135-.0406 1.202 1.202 0 0 0 .1342.103c.5703.3765 1.5823.4213 3.0068.1344-.2016.1769-.5189.3994-.9533.6011-.4098.1903-1.0957.333-1.7473.3636-.7197.0336-1.0859-.0807-1.1721-.151zm.5695-9.2712c-.0059.3508-.0542.6692-.1054 1.0017-.055.3576-.112.7274-.1264 1.1762-.0142.4368.0404.8909.0932 1.3301.1066.887.216 1.8003-.2075 2.7014a3.5272 3.5272 0 0 1-.1876-.3856c-.0527-.1276-.1669-.3326-.3251-.6162-.6156-1.1041-2.0574-3.6896-1.3193-4.7446.3795-.5427 1.3408-.5661 2.1781-.463zm.2284 7.0137a12.3762 12.3762 0 0 0-.0853-.1074l-.0355-.0444c.7262-1.1995.5842-2.3862.4578-3.4385-.0519-.4318-.1009-.8396-.0885-1.2226.0129-.4061.0666-.7543.1185-1.0911.0639-.415.1288-.8443.1109-1.3505.0134-.0531.0188-.1158.0118-.1902-.0457-.4855-.5999-1.938-1.7294-3.253-.6076-.7073-1.4896-1.4972-2.6889-2.0395.5251-.1066 1.2328-.2035 2.0244-.1859 2.0515.0456 3.6746.8135 4.8242 2.2824a.908.908 0 0 1 .0667.1002c.7231 1.3556-.2762 6.2751-2.9867 10.5405zm-8.8166-6.1162c-.025.1794-.3089.4225-.6211.4225a.5821.5821 0 0 1-.0809-.0056c-.1873-.026-.3765-.144-.5059-.3156-.0458-.0605-.1203-.178-.1055-.2844.0055-.0401.0261-.0985.0925-.1488.1182-.0894.3518-.1226.6096-.0867.3163.0441.6426.1938.6113.4186zm7.9305-.4114c.0111.0792-.049.201-.1531.3102-.0683.0717-.212.1961-.4079.2232a.5456.5456 0 0 1-.075.0052c-.2935 0-.5414-.2344-.5607-.3717-.024-.1765.2641-.3106.5611-.352.297-.0414.6111.0088.6356.1851z";

#[derive(Clone, Copy)]
pub struct Experience {
    /// Short name used by the progress tracker and "Next up" card.
    pub short: &'static str,
    pub icon: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub route: &'static str,
}

#[derive(Clone, Copy)]
pub struct Certificate {
    pub badge: &'static str,
    pub category: &'static str,
    pub kind_label: &'static str,
    pub date: &'static str,
    pub certificate_kind: &'static str,
    pub description: &'static str,
}

#[derive(Clone, Copy)]
pub struct WebProject {
    pub tag: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Copy)]
pub struct WorkSample {
    pub src: &'static str,
    pub alt: &'static str,
}

#[derive(Clone, Copy)]
pub struct PillLink {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Copy)]
pub struct Game {
    pub name: &'static str,
    pub description: &'static str,
    pub image: &'static str,
    pub title_url: &'static str,
    pub links: &'static [PillLink],
    /// Full path of the static legal page, rendered as the "Legal Info" pill.
    pub privacy: Option<&'static str>,
}

#[derive(Clone, Copy)]
pub struct ErpModule {
    pub icon: &'static str,
    pub label: &'static str,
}

#[derive(Clone, Copy)]
pub struct TechGroup {
    pub heading: &'static str,
    pub items: &'static [&'static str],
}

#[derive(Clone, Copy)]
pub struct ContactItem {
    pub label: &'static str,
    pub url: &'static str,
    pub icon: &'static str,
    /// Small caption above the value, saying what the link does.
    pub caption: &'static str,
    pub new_tab: bool,
}

// ---------------------------------------------------------------- global

pub const SITE_TITLE: &str = "Dogu Bolut";
pub const OWNER: &str = "Dogu Bolut";

pub const BOOK_MEETING_LABEL: &str = "Book Meeting";
pub const BOOK_MEETING_URL: &str = "https://calendar.app.google/oDQZRGp8SBAZg6HM9";
pub const DOWNLOAD_CV_LABEL: &str = "Download CV";
pub const DOWNLOAD_CV_URL: &str = "/assets/pdf/Dogu%20Bolut%20CV%20-%20English.pdf";

/// Netlify Forms: the form is detected from the static copy in `index.html`;
/// `FORM_ACTION` is the page shown after a successful submit.
pub const FORM_NAME: &str = "contact";
pub const FORM_ACTION: &str = "/thanks";
pub const FORM_HONEYPOT: &str = "bot-field";
pub const CONTACT_HEADING: &str = "Contacts";
pub const CONTACT_SUBHEADING: &str = "Let's Work Together!";
pub const FORM_NAME_PLACEHOLDER: &str = "Full Name";
pub const FORM_EMAIL_PLACEHOLDER: &str = "Corporate Email";
pub const FORM_MESSAGE_PLACEHOLDER: &str = "Your Message";
pub const FORM_SUBMIT: &str = "Submit";
pub const FORM_TITLE: &str = "Send a message";
pub const FORM_NOTE: &str = "I usually reply within 1–2 days";
pub const FORM_PRIVACY: &str = "Your details are only used to reply to you.";

pub const THANKS_TITLE: &str = "Message sent. Thank you!";
pub const THANKS_TEXT: &str =
    "I read every message and usually reply within a day or two. If it's urgent, book a call and we can talk sooner.";
pub const THANKS_BACK: &str = "Back to home";

pub const CONTACT_ITEMS: &[ContactItem] = &[
    ContactItem {
        label: "+90 536 700 09 31",
        url: "tel:+905367000931",
        icon: "fa-solid fa-phone",
        caption: "Call me",
        new_tab: false,
    },
    ContactItem {
        label: "dogubolut@gmail.com",
        url: "mailto:dogubolut@gmail.com",
        icon: "fa-solid fa-envelope",
        caption: "Email me",
        new_tab: false,
    },
    ContactItem {
        label: "Umraniye, Istanbul",
        url: "https://maps.app.goo.gl/HDoekYDTvmD95tMY9",
        icon: "fa-solid fa-location-dot",
        caption: "Based in",
        new_tab: true,
    },
];

pub const SOCIALS: &[SocialLink] = &[
    SocialLink {
        label: "LinkedIn",
        url: "https://www.linkedin.com/in/dogu-bolut/",
        icon: "fab fa-linkedin",
    },
    SocialLink {
        label: "GitHub",
        url: "https://github.com/dogu-bolut",
        icon: "fab fa-github",
    },
    SocialLink {
        label: "Instagram",
        url: "https://instagram.com/dogubolut",
        icon: "fab fa-instagram",
    },
    SocialLink {
        label: "X",
        url: "https://x.com/dogubolut",
        icon: "fa-brands fa-x-twitter",
    },
];

// ---------------------------------------------------------------- navbar

const fn nav(label: &'static str, anchor: &'static str) -> NavLink {
    NavLink { label, anchor }
}

pub const NAV_HOME: &[NavLink] = &[
    nav("Home", "home"),
    nav("Experiences", "experiences"),
    nav("Certificates", "certificates"),
    nav("Contacts", "contact"),
];
pub const NAV_WEB: &[NavLink] = &[
    nav("Home", "home"),
    nav("Projects", "web-projects"),
    nav("Tech Stack", "tech-stack"),
    nav("Contact", "contact"),
];
pub const NAV_GAMES: &[NavLink] = &[
    nav("Home", "home"),
    nav("Games", "games"),
    nav("Tech Stack", "tech-stack"),
    nav("Contact", "contact"),
];
pub const NAV_ERP: &[NavLink] = &[
    nav("Home", "home"),
    nav("Journey", "journey"),
    nav("Tech Stack", "tech-stack"),
    nav("Contact", "contact"),
];

/// Link set for a pathname; unknown routes use the home set.
pub fn nav_links(path: &str) -> &'static [NavLink] {
    match path.trim_end_matches('/') {
        "/web" => NAV_WEB,
        "/games" => NAV_GAMES,
        "/erp" => NAV_ERP,
        _ => NAV_HOME,
    }
}

// ---------------------------------------------------------------- home

pub const HOME_INTRO: PageIntro = PageIntro {
    intro: "Hi, I'm ",
    highlight: "Dogu Bolut",
    title: "Software Developer",
    description: "I build websites people use, games people play, and the ERP systems businesses run on. Everything here is real and live, so see it for yourself.",
};

pub const PROFILE_IMAGE: &str = "/assets/img/logofalan.png";
pub const PROFILE_ALT: &str = "Dogu Bolut logo";

pub const FLOATING_ICONS: &[FloatingIcon] = &[
    FloatingIcon {
        position: "first",
        glyph: Glyph::FontAwesome("fab fa-css"),
    },
    FloatingIcon {
        position: "second",
        glyph: Glyph::FontAwesome("fa-brands fa-square-js"),
    },
    FloatingIcon {
        position: "third",
        glyph: Glyph::FontAwesome("fab fa-html5"),
    },
    FloatingIcon {
        position: "fourth",
        glyph: Glyph::FontAwesome("fa-brands fa-rust"),
    },
    FloatingIcon {
        position: "fifth",
        glyph: Glyph::Svg(CLAUDE_PATH),
    },
    FloatingIcon {
        position: "sixth",
        glyph: Glyph::FontAwesome("fa-brands fa-unity"),
    },
    FloatingIcon {
        position: "seventh",
        glyph: Glyph::Svg(POSTGRESQL_PATH),
    },
    FloatingIcon {
        position: "eighth",
        glyph: Glyph::FontAwesome("fa-brands fa-linux"),
    },
];

pub const AVAILABILITY: &str = "Available for full-time roles & collaborations";
pub const ERP_SINCE: &str = "2024";

pub const EXPERIENCES_HEADING: &str = "Experiences";
pub const EXPERIENCES_SUBHEADING: &str =
    "Three paths, one developer. See all three, then let's talk.";
pub const SEEN_LABEL: &str = "Seen";
pub const UP_NEXT_LABEL: &str = "Start here";

// home hero tour card
pub const TOUR_KICKER: &str = "The 2-minute tour";
pub const TOUR_TITLE: &str = "Three worlds. Pick one, see all three.";
pub const TOUR_START: &str = "Start the tour";
pub const TOUR_CONTINUE: &str = "Continue with";
pub const TOUR_DONE: &str = "You've seen it all. Let's talk";

// progress tracker and "Next up"
pub const TRACKER_LABEL: &str = "Explored";
pub const TRACKER_DONE: &str = "Seen it all? Let's talk";
pub const NEXT_UP_KICKER: &str = "Keep exploring";
pub const NEXT_UP_DONE_TITLE: &str = "You've seen all three. Let's work together.";
pub const NEXT_UP_DONE_TEXT: &str =
    "Web, games and ERP: if any of it fits what you're building, book a call or grab my CV.";
pub const LEARN_MORE: &str = "Learn more";

/// Headline number for an experience, from the data it summarises.
pub fn experience_stat(route: &str) -> String {
    match route {
        "/web" => format!("{} live websites", WEB_PROJECTS.len()),
        "/games" => format!("{} games", GAMES.len()),
        _ => format!("ERP since {ERP_SINCE}"),
    }
}

pub const EXPERIENCES: &[Experience] = &[
    Experience {
        short: "Web",
        icon: "fa-solid fa-code",
        title: "Web Development",
        description: "Building responsive modern websites using HTML, CSS, and JavaScript.",
        image: "/assets/img/web-development.png",
        route: "/web",
    },
    Experience {
        short: "Games",
        icon: "fa-solid fa-gamepad",
        title: "Game Development",
        description: "Creating games in Unity with smooth mechanics and engaging design.",
        image: "/assets/img/game_console.png",
        route: "/games",
    },
    Experience {
        short: "ERP",
        icon: "fa-solid fa-diagram-project",
        title: "ERP",
        description: "Implement ERP systems to streamline business processes and efficiency.",
        image: "/assets/img/graphics-design.png",
        route: "/erp",
    },
];

pub const CERTIFICATES_HEADING: &str = "Certificates";

pub const CERTIFICATES: &[Certificate] = &[
    Certificate {
        badge: "UNITY",
        category: "GAME DEVELOPMENT",
        kind_label: "COURSE",
        date: "20 January 2023",
        certificate_kind: "Completion Certificate",
        description: "This certificate verifies that Doğu Bolut successfully completed the course RPG Core Combat Creator: Learn Intermediate Unity C# Coding on 01/20/2023 as taught by Ben Tristem, Rick Davidson, GameDev.tv Team on Udemy.",
    },
    Certificate {
        badge: "DIGIAGE",
        category: "GAME DEVELOPMENT",
        kind_label: "SUMMER CAMP",
        date: "1-11 September 2022",
        certificate_kind: "Participation Certificate",
        description: "\"Thank you for participating in the 10-day Game Development Summer Camp, hosted by Bilişim Vadisi and organized by DIGIAGE and Oyunla Gelecek.\"",
    },
    Certificate {
        badge: "MAĞARA JAM",
        category: "GAME DEVELOPMENT",
        kind_label: "HACKATHON",
        date: "18-21 February 2022",
        certificate_kind: "Participation Certificate",
        description: "I joined the 4th Mağara Jam, creating a game within a short time, and earned a participation certificate.",
    },
    Certificate {
        badge: "UNITY",
        category: "GAME DEVELOPMENT",
        kind_label: "COURSE",
        date: "22 September 2020",
        certificate_kind: "Completion Certificate",
        description: "This certificate verifies that Doğu Bolut successfully completed the course Complete C# Unity Game Developer 2D on 10/22/2020 as taught by GameDev.tv Team, Rick Davidson, Gary Pettie on Udemy.",
    },
];

pub const HOME_BLURB: &str = "I am open to full-time positions and professional collaborations. Contact me to discuss potential work or opportunities.";

// ---------------------------------------------------------------- web

pub const WEB_INTRO: PageIntro = PageIntro {
    intro: "This section ",
    highlight: "shows",
    title: "My Websites",
    description: "Welcome to the website section of my portfolio. This space features my work in web development, where design meets functionality. I enjoy creating fast, accessible, and visually engaging websites that provide real value to users. Every project is an opportunity to turn a concept into a polished digital experience.",
};

pub const WEB_HERO_LINKS: &[SocialLink] = &[SocialLink {
    label: "Netlify projects",
    url: "https://app.netlify.com/teams/dogubolut/projects",
    icon: "fa-solid fa-diamond",
}];

pub const WORK_SAMPLES_HEADING: &str = "Work Samples";
pub const WORK_SAMPLES_URL: &str = "ersatr.com";

pub const WORK_SAMPLES: &[WorkSample] = &[
    WorkSample {
        src: "/assets/img/worksamples1.png",
        alt: "Ersatr",
    },
    WorkSample {
        src: "/assets/img/worksamples2.png",
        alt: "Ersatr",
    },
    WorkSample {
        src: "/assets/img/worksamples3.png",
        alt: "Ersatr",
    },
];

pub const PROJECTS_HEADING: &str = "Projects";
pub const GO_TO_WEBSITE: &str = "Go to Website";

pub const WEB_PROJECTS: &[WebProject] = &[
    WebProject {
        tag: "B2B",
        name: "Ersa Foreign Trade",
        description: "Designed primarily to act as a digital storefront.",
        url: "https://www.ersatr.com",
    },
    WebProject {
        tag: "E-Commerce",
        name: "Sağlık Paketi",
        description:
            "Created custom storefront with responsive layouts and intuitive shopping experience.",
        url: "https://www.saglikpaketi.com.tr",
    },
    WebProject {
        tag: "E-Commerce",
        name: "BAW Shop",
        description:
            "Modern e-commerce frontend with responsive layouts and interactive product display.",
        url: "https://dogubolut-baw.netlify.app/",
    },
    WebProject {
        tag: "Movie Review",
        name: "Angular Movie App",
        description: "Developed Angular app displaying movies with search, filters, and ratings.",
        url: "https://dogubolut-movieapp.netlify.app/",
    },
    WebProject {
        tag: "API",
        name: "Dog Breeds",
        description: "Search and explore dog breeds with real-time API integration functionality.",
        url: "https://dogubolut-dogbreedlist.netlify.app/",
    },
    WebProject {
        tag: "Clone",
        name: "Youtube",
        description: "Static homepage mockup of YouTube interface showcasing layout and design.",
        url: "https://dogubolut-youtube.netlify.app/",
    },
    WebProject {
        tag: "Entertainment",
        name: "Rock Paper Scissors",
        description: "Interactive Rock Paper Scissors game playable directly in the browser.",
        url: "https://dogubolut-rockpaperscissors.netlify.app/",
    },
    WebProject {
        tag: "Clone",
        name: "Amazon",
        description:
            "Amazon clone project demonstrating e-commerce layout, design, and functionality.",
        url: "https://dogubolut-amazon.netlify.app/",
    },
    WebProject {
        tag: "Radio",
        name: "Lounge FM",
        description: "A simple radio website where you can listen live anytime.",
        url: "https://dogubolut-radio.netlify.app/",
    },
];

pub const WEB_TECH: &[TechGroup] = &[
    TechGroup {
        heading: "Programming",
        items: &["HTML", "CSS", "JavaScript", "TypeScript"],
    },
    TechGroup {
        heading: "Framework & Library",
        items: &["Angular", "Apache Echarts", "Tabulator", "Leaflet"],
    },
    TechGroup {
        heading: "Tool & Deployment",
        items: &["Git", "Netlify"],
    },
];

pub const WEB_BLURB: &str = "I’m actively seeking full-time opportunities where I can contribute my skills in web development. If you’re a recruiter or company representative, feel free to reach out directly.";

// ---------------------------------------------------------------- games

pub const GAMES_INTRO: PageIntro = PageIntro {
    intro: "This section ",
    highlight: "shows",
    title: "My Games",
    description: "Welcome to the games section of my portfolio! Here are games I’ve worked on, from small prototypes to complete projects. Each project reflects my passion for game development, creative problem-solving, and technical design. Feel free to explore and play!",
};

pub const GAMES_HERO_LINKS: &[SocialLink] = &[
    SocialLink {
        label: "Google Play developer page",
        url: "https://play.google.com/store/apps/dev?id=7823482579787878315",
        icon: "fa-brands fa-google-play",
    },
    SocialLink {
        label: "itch.io profile",
        url: "https://itch.io/profile/dogubolut",
        icon: "fa-brands fa-itch-io",
    },
];

pub const GAMES_SHOWCASE_TITLE: &str = "game library";
pub const GAMES_SHOWCASE_CAPTION: &str = "PC & Mobile · Unity · Ren'Py";

pub const GAMES_HEADING: &str = "Games";
pub const LEGAL_INFO: &str = "Legal Info";

pub const GAMES: &[Game] = &[
    Game {
        name: "Brick Puzzle",
        description: "A relaxing 10×10 block puzzle with timeless, strategic fun for all.",
        image: "/assets/img/brickpuzzle.svg",
        title_url: "https://play.google.com/store/apps/details?id=com.mr.stijket.BrickPuzzle",
        links: &[PillLink {
            label: "Mobile",
            url: "https://play.google.com/store/apps/details?id=com.mr.stijket.BrickPuzzle",
        }],
        privacy: Some("/assets/pages/privacy/brickpuzzle"),
    },
    Game {
        name: "Çantamda Ne Var?",
        description: "What's in My Bag? was made in 3 days for Mağara Jam #4.",
        image: "/assets/img/whatsinmybag.png",
        title_url: "https://ozgurbyrm.itch.io/cantamda-ne-var",
        links: &[
            PillLink {
                label: "PC",
                url: "https://ozgurbyrm.itch.io/cantamda-ne-var",
            },
            PillLink {
                label: "Mobile",
                url:
                    "https://play.google.com/store/apps/details?id=com.WinterSkyStudio.antamdaNeVar",
            },
        ],
        privacy: Some("/assets/pages/privacy/whatsinmybag"),
    },
    Game {
        name: "Corridor",
        description: "Digiage game development summer camp, table no 18 : Corridor",
        image: "/assets/img/corridor.png",
        title_url: "https://dogubolut.itch.io/corridor",
        links: &[PillLink {
            label: "PC",
            url: "https://dogubolut.itch.io/corridor",
        }],
        privacy: None,
    },
    Game {
        name: "My New Life",
        description: "A heartfelt visual novel about love and tough choices in high school life.",
        image: "/assets/img/mynewlife.jpg",
        title_url: "https://dogubolut.itch.io/my-new-life",
        links: &[
            PillLink {
                label: "PC",
                url: "https://dogubolut.itch.io/my-new-life",
            },
            PillLink {
                label: "Mobile",
                url: "https://play.google.com/store/apps/details?id=com.mr.stijket.mynewlife",
            },
        ],
        privacy: Some("/assets/pages/privacy/mynewlife"),
    },
    Game {
        name: "3D Space Shooter",
        description:
            "Pilot your ship, dodge obstacles, and blast enemies in this fast arcade shooter.",
        image: "/assets/img/3dspaceshooter.png",
        title_url: "https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceShooter",
        links: &[PillLink {
            label: "Mobile",
            url: "https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceShooter",
        }],
        privacy: Some("/assets/pages/privacy/3dspaceshooter"),
    },
    Game {
        name: "Tile Vania",
        description:
            "Pixel-art platformer with traps and puzzles; dare your skills in an adventure.",
        image: "/assets/img/tilevania.jpg",
        title_url: "https://play.google.com/store/apps/details?id=com.mr.stijket.TileVania",
        links: &[PillLink {
            label: "Mobile",
            url: "https://play.google.com/store/apps/details?id=com.mr.stijket.TileVania",
        }],
        privacy: Some("/assets/pages/privacy/tilevania"),
    },
    Game {
        name: "Bounty Hunter",
        description: "Be a bounty hunter, track foes, and earn rewards in this action-packed game.",
        image: "/assets/img/bountyhunter.jpg",
        title_url: "https://play.google.com/store/apps/details?id=com.mr.stijket.BountyHunter",
        links: &[PillLink {
            label: "Mobile",
            url: "https://play.google.com/store/apps/details?id=com.mr.stijket.BountyHunter",
        }],
        privacy: Some("/assets/pages/privacy/bountyhunter"),
    },
    Game {
        name: "Space Drifter",
        description: "Embark on a cosmic adventure, dodge asteroids, and battle space pirates.",
        image: "/assets/img/spacedrifter.png",
        title_url: "https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceDrifter",
        links: &[PillLink {
            label: "Mobile",
            url: "https://play.google.com/store/apps/details?id=com.mr.stijket.SpaceDrifter",
        }],
        privacy: Some("/assets/pages/privacy/spacedrifter"),
    },
    Game {
        name: "Terminal Hacker",
        description: "Crack the code, solve the word puzzles, and find the hidden clues.",
        image: "/assets/img/terminalhacker.png",
        title_url: "https://dogubolut.itch.io/terminal-hacker",
        links: &[PillLink {
            label: "PC",
            url: "https://dogubolut.itch.io/terminal-hacker",
        }],
        privacy: None,
    },
];

pub const GAMES_TECH: &[TechGroup] = &[
    TechGroup {
        heading: "Game Engine",
        items: &["Unity", "Unreal Engine", "Ren'Py"],
    },
    TechGroup {
        heading: "Programming",
        items: &["C#", "Git"],
    },
    TechGroup {
        heading: "Design & Art",
        items: &["Photoshop", "Gimp", "Blender"],
    },
];

pub const GAMES_BLURB: &str = "I create polished, engaging game experiences and thrive in collaborative environments. I’m ready to contribute my skills to projects that value innovation and quality.";

// ---------------------------------------------------------------- erp

pub const ERP_INTRO: PageIntro = PageIntro {
    intro: "This section ",
    highlight: "shows",
    title: "My ERP Journey",
    description: "Welcome to the ERP section of my portfolio. This space highlights my journey in enterprise resource planning, where technology connects and streamlines business processes. I enjoy learning how ERP brings structure and efficiency, and I’m focused on growing my skills to contribute to real-world solutions.",
};

pub const ERP_SHOWCASE_TITLE: &str = "erp · modules";
pub const ERP_CORE: &str = "ERP";
pub const ERP_SHOWCASE_CAPTION: &str = "one shared database";

/// Modules shown around the core, left column first.
pub const ERP_MODULES: &[ErpModule] = &[
    ErpModule {
        icon: "fa-solid fa-industry",
        label: "Production",
    },
    ErpModule {
        icon: "fa-solid fa-user-group",
        label: "Human Resources",
    },
    ErpModule {
        icon: "fa-solid fa-boxes-stacked",
        label: "Inventory",
    },
    ErpModule {
        icon: "fa-solid fa-handshake",
        label: "CRM",
    },
    ErpModule {
        icon: "fa-solid fa-chart-line",
        label: "Sales",
    },
    ErpModule {
        icon: "fa-solid fa-screwdriver-wrench",
        label: "Service",
    },
    ErpModule {
        icon: "fa-solid fa-truck-fast",
        label: "Distribution",
    },
    ErpModule {
        icon: "fa-solid fa-diagram-project",
        label: "Business Process",
    },
];

pub const JOURNEY_HEADING: &str = "Journey";

pub const JOURNEY_STEPS: &[&str] = &[
    "🚀 Started as an intern at WorkRunner in March 2024",
    "💡 After 2 months, I stepped into the role of Junior Developer, expanding my skills",
    "⚡ Gained expertise in ERP solutions, leading small projects end-to-end",
    "🌟 Ready to take on new adventures and contribute to innovative teams",
];

pub const ERP_TECH: &[TechGroup] = &[
    TechGroup {
        heading: "Core Development",
        items: &["Javascript", "SQL"],
    },
    TechGroup {
        heading: "Libraries & Tools",
        items: &[
            "Apache ECharts",
            "Tabulator",
            "Leaflet",
            "Other Javascript Libraries",
        ],
    },
    TechGroup {
        heading: "Infrastructure",
        items: &["Redis", "MS SQL", "REST API"],
    },
];

pub const ERP_BLURB: &str = "I’m seeking opportunities to contribute my experience in ERP development, database management, and process optimization within a collaborative team.";

// ---------------------------------------------------------------- shared labels

pub const TECH_STACK_HEADING: &str = "Tech Stack";

// ---------------------------------------------------------------- 404

pub const NOT_FOUND_HEADING: &str = "404 - Page Not Found";
pub const NOT_FOUND_TEXT: &str = "The page you are looking for does not exist.";
pub const NOT_FOUND_LINK: &str = "Back to Home";

// ---------------------------------------------------------------- page meta

pub const META_HOME: &str =
    "Dogu Bolut — Software Developer. Web development, game development and ERP portfolio.";
pub const META_WEB: &str = "Web development portfolio of Dogu Bolut, Software Developer.";
pub const META_GAMES: &str = "Game development portfolio of Dogu Bolut, Software Developer.";
pub const META_ERP: &str = "ERP journey of Dogu Bolut, Software Developer.";
