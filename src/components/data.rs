pub struct ProfileInfo<'a> {
    pub name: &'a str,
    pub title: &'a str,
}

pub const PROFILE: ProfileInfo = ProfileInfo {
    name: "Alexander Alexandrov",
    title: "Full-Stack Developer",
};

pub struct SkillCategory<'a> {
    pub name: &'a str,
    pub skills: &'a [&'a str],
}

pub const SKILL_CATEGORIES: &[SkillCategory] = &[
    SkillCategory {
        name: "Languages",
        skills: &["Rust", "TypeScript", "JavaScript", "Java", "ExtendScript"],
    },
    SkillCategory {
        name: "Frontend",
        skills: &[
            "React",
            "Dioxus",
            "Redux",
            "Material UI",
            "HTML",
            "CSS",
            "WebAssembly",
        ],
    },
    SkillCategory {
        name: "Backend",
        skills: &["Spring MVC", "PostgreSQL", "iBATIS"],
    },
    SkillCategory {
        name: "Fullstack",
        skills: &["Next.js", "Node.js"],
    },
    SkillCategory {
        name: "AI / ML",
        skills: &["LLM Integration (GPT, Claude)", "Vercel AI SDK"],
    },
    SkillCategory {
        name: "DevOps",
        skills: &["AWS", "Terraform", "Docker", "Vercel", "Git"],
    },
];

pub struct ExperienceEntry<'a> {
    pub title: &'a str,
    pub company_and_period: &'a str,
    pub responsibilities: &'a [&'a str],
}

pub const EXPERIENCE_ENTRIES: &[ExperienceEntry] = &[
    ExperienceEntry {
        title: "Full-Stack Developer",
        company_and_period: "Proxiad | Aug 2023 – Present",
        responsibilities: &[
            "Develop front-end applications for a digital publishing suite — Publications Editor, InDesign Plugin, Reader, Configurator, Assets Drive, and a cross-platform mobile app — using React, TypeScript, JavaScript, ExtendScript, Redux, and SCSS.",
            "Contribute to Java back-end APIs, maintaining coding standards and test coverage.",
            "Collaborate with UI/UX teams to streamline releases and improve feature turnaround time.",
        ],
    },
    ExperienceEntry {
        title: "Full-Stack Developer",
        company_and_period: "DXC Technology | Apr 2020 - Aug 2023",
        responsibilities: &[
            "Modernized an enterprise solution by migrating critical components from Java to Rust.",
            "Developed a Visual Product Modeling System using React, TypeScript, Material UI, and Java, improving product visualization and user workflows.",
            "Built a full-stack service using AWS SDK, React, TypeScript, and Terraform to integrate cloud infrastructure into customer-facing solutions.",
            "Developed features for the BMW Group Vulnerability Tool; optimized PostgreSQL schemas and iBATIS data layers, reducing query time by 90% (from 30s to 3s).",
        ],
    },
    ExperienceEntry {
        title: "Web Developer Intern",
        company_and_period: "Camplight | Sep 2019 - Apr 2020",
        responsibilities: &[
            "Developed a property management web application using React, Next.js, and GraphQL.",
            "Wrote reusable UI components in TypeScript and Material UI.",
            "Collaborated with mentors on architecture reviews and code quality, delivering production-ready components.",
        ],
    },
];

pub struct ProjectEntry<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub url: &'a str,
}

pub const PROJECTS: &[ProjectEntry] = &[
    ProjectEntry {
        name: "Ferrocrypt",
        description: "A highly secure multi-platform GUI/CLI encryption/decryption tool, implemented in Rust",
        url: "https://github.com/alexylon/ferrocrypt",
    },
    ProjectEntry {
        name: "Sofos Code",
        description: "A blazing-fast, interactive AI coding agent for your terminal, implemented in pure Rust",
        url: "https://github.com/alexylon/sofos-code",
    },
    ProjectEntry {
        name: "Sofos Web",
        description: "A multi-modal, multi-model AI chatbot, implemented in React/Next.js",
        url: "https://github.com/alexylon/sofos-web",
    },
    ProjectEntry {
        name: "Wordle",
        description: "The popular Wordle game in Rust",
        url: "https://github.com/alexylon/wordle",
    },
];

pub struct EducationEntry<'a> {
    pub title: &'a str,
    pub institution: &'a str,
}

pub const EDUCATION: &[EducationEntry] = &[
    EducationEntry {
        title: "React & JavaScript",
        institution: "Camplight Academy",
    },
    EducationEntry {
        title: "Java Fundamentals",
        institution: "MaxPlus",
    },
    EducationEntry {
        title: "M.Eng., Engineering",
        institution: "University of Forestry",
    },
    EducationEntry {
        title: "PhD, Theology",
        institution: "Sofia University “St. Kliment Ohridski”",
    },
    EducationEntry {
        title: "Mathematics",
        institution: "High School of Mathematics",
    },
];

pub struct Language<'a> {
    pub name: &'a str,
    pub level: &'a str,
}

pub const LANGUAGES: &[Language] = &[
    Language {
        name: "Bulgarian",
        level: "(Native)",
    },
    Language {
        name: "English",
        level: "(C1)",
    },
    Language {
        name: "Italian",
        level: "(B2)",
    },
    Language {
        name: "Russian",
        level: "(B1)",
    },
    Language {
        name: "Greek",
        level: "(A2)",
    },
];

pub enum ContactLinkHref<'a> {
    Plain(&'a str),
    // ResumeAsset,
}

pub struct ContactLink<'a> {
    pub label: &'a str,
    pub href: ContactLinkHref<'a>,
    pub target: Option<&'a str>,
    pub rel: Option<&'a str>,
    pub _download: Option<&'a str>,
}

pub const CONTACT_LINKS: &[ContactLink] = &[
    ContactLink {
        label: "Email",
        href: ContactLinkHref::Plain("mailto:hi@alexo.io"),
        target: None,
        rel: None,
        _download: None,
    },
    ContactLink {
        label: "LinkedIn",
        href: ContactLinkHref::Plain("https://www.linkedin.com/in/alexandrovalexander/"),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        _download: None,
    },
    ContactLink {
        label: "GitHub",
        href: ContactLinkHref::Plain("https://github.com/alexylon"),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        _download: None,
    },
    ContactLink {
        label: "Mastodon",
        href: ContactLinkHref::Plain("https://fosstodon.org/@lexer"),
        target: Some("_blank"),
        rel: Some("noopener noreferrer"),
        _download: None,
    },
    // ContactLink {
    //     label: "Resume ⬇",
    //     href: ContactLinkHref::ResumeAsset,
    //     target: None,
    //     rel: None,
    //     download: Some("resume_alexander_alexandrov.pdf"),
    // },
];
