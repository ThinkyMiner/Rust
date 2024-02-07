// main.rs
use askama::Template;
use std::fs::write;

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct CvTemplate<'a> {
    name: &'a str,
    email: &'a str,
    phone: &'a str,
    education: Vec<EducationEntry<'a>>,
    experience: Vec<ExperienceEntry<'a>>,
}

#[derive(Template)]
struct EducationEntry<'a> {
    degree: &'a str,
    field: &'a str,
    year: &'a str,
}

#[derive(Template)]
struct ExperienceEntry<'a> {
    position: &'a str,
    company: &'a str,
    year: &'a str,
}

fn main() {
    // Replace this with your actual data
    let cv_data = CvTemplate {
        name: "John Doe",
        email: "john.doe@example.com",
        phone: "+1 555-1234",
        education: vec![
            EducationEntry {
                degree: "Bachelor",
                field: "Computer Science",
                year: "2018",
            },
        ],
        experience: vec![
            ExperienceEntry {
                position: "Software Engineer",
                company: "ABC Inc.",
                year: "2018-2022",
            },
        ],
    };

    let rendered_cv = cv_data.render().unwrap();

    // Write the generated HTML to a file
    write("output/cv_output.html", rendered_cv).unwrap();
}
