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
    pub icon: &'static str,
}

#[derive(Clone, Copy)]
pub struct Experience {
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
    pub new_tab: bool,
}

// ---------------------------------------------------------------- global

pub const SITE_TITLE: &str = "Dogu Bolut";
pub const OWNER: &str = "Dogu Bolut";

pub const BOOK_MEETING_LABEL: &str = "Book Meeting";
pub const BOOK_MEETING_URL: &str = "https://calendar.app.google/oDQZRGp8SBAZg6HM9";
pub const DOWNLOAD_CV_LABEL: &str = "Download CV";
pub const DOWNLOAD_CV_URL: &str = "/assets/pdf/Dogu%20Bolut%20CV%20-%20English.pdf";

pub const FORM_ACTION: &str = "https://formspree.io/f/xgvlodny";
pub const CONTACT_HEADING: &str = "Contacts";
pub const CONTACT_SUBHEADING: &str = "Let's Work Together!";
pub const FORM_NAME_PLACEHOLDER: &str = "Full Name";
pub const FORM_EMAIL_PLACEHOLDER: &str = "Corporate Email";
pub const FORM_MESSAGE_PLACEHOLDER: &str = "Your Message";
pub const FORM_SUBMIT: &str = "Submit";

pub const CONTACT_ITEMS: &[ContactItem] = &[
    ContactItem {
        label: "+90 536 700 09 31",
        url: "tel:+905367000931",
        icon: "fa-solid fa-phone",
        new_tab: false,
    },
    ContactItem {
        label: "dogubolut@gmail.com",
        url: "mailto:dogubolut@gmail.com",
        icon: "fa-solid fa-envelope",
        new_tab: false,
    },
    ContactItem {
        label: "Umraniye, Istanbul",
        url: "https://maps.app.goo.gl/HDoekYDTvmD95tMY9",
        icon: "fa-solid fa-house",
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
    description: "This website shows my portfolio. Feel free to explore the links below, where you can find details about almost every project I've worked on. If you have any questions, please don't hesitate to reach out to me via email.",
};

pub const PROFILE_IMAGE: &str = "/assets/img/logofalan.png";
pub const PROFILE_ALT: &str = "Dogu Bolut logo";

pub const FLOATING_ICONS: &[FloatingIcon] = &[
    FloatingIcon {
        position: "first",
        icon: "fab fa-css",
    },
    FloatingIcon {
        position: "second",
        icon: "fa-brands fa-square-js",
    },
    FloatingIcon {
        position: "third",
        icon: "fab fa-html5",
    },
    FloatingIcon {
        position: "fourth",
        icon: "fa-brands fa-node",
    },
    FloatingIcon {
        position: "fifth",
        icon: "fa-brands fa-angular",
    },
    FloatingIcon {
        position: "sixth",
        icon: "fa-brands fa-unity",
    },
    FloatingIcon {
        position: "seventh",
        icon: "fa-solid fa-database",
    },
    FloatingIcon {
        position: "eighth",
        icon: "fa-brands fa-linux",
    },
];

pub const EXPERIENCES_HEADING: &str = "Experiences";
pub const LEARN_MORE: &str = "Learn more";

pub const EXPERIENCES: &[Experience] = &[
    Experience {
        title: "Web Development",
        description: "Building responsive modern websites using HTML, CSS, and JavaScript.",
        image: "/assets/img/web-development.png",
        route: "/web",
    },
    Experience {
        title: "Game Development",
        description: "Creating games in Unity with smooth mechanics and engaging design.",
        image: "/assets/img/game_console.png",
        route: "/games",
    },
    Experience {
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
