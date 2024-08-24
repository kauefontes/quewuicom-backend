use axum::{response::Html, routing::get, Router};
use std::env;
use tower_http::cors::{Any, CorsLayer};

const MARKDOWN_CONTENT: &str = r#"
# Kaue Pereira

- **Github:** [github.com/kauefontes](https://github.com/kauefontes)
- **Phone:** +55 (92) 98138-6423
- **Email:** kauefontes@outlook.com
- **LinkedIn:** [linkedin.com/in/kauefontes](https://linkedin.com/in/kauefontes)

## Summary

Accomplished software engineer with over 9 years of experience specializing in frontend and backend development. Expert in React Native, Typescript, and Embedded Android, with a proven track record of leading cross-functional teams and driving agile practices. Demonstrated success in delivering high-quality software solutions and improving performance metrics. Ready to leverage my expertise and leadership skills to solve complex technical challenges.

## Technical Skills

- **Frontend:** React Native, React, Typescript, Javascript, HTML, CSS, Redux, Redux Toolkit, MobX, Hooks, Design Patterns.
- **Backend:** Rust, .NET, NodeJS, RESTful API, SOAP, Docker, AWS, Google Cloud, Azure, Azure for US Gov.
- **Tools:** Firebase, GitHub Actions, Jenkins, Storybook, Pipelines, Continuous Integration (CI), Continuous Deployment.
- **Mobile and IoT:** Android, iOS, Kotlin, Java, BLE, Push Notifications.
- **Agile:** Kanban, Scrum, Agile Coaching, Team Building, Risk Management, Capacity Management, Team Metrics.

## Professional Experience

### Senior Software Engineer | BairesDev | Dec 2023 - Present

- Led development efforts for the Lenslock client using React, Storybook, Typescript, .NET, Razor Pages, HTML, CSS, and Javascript.
- Delivered highly interactive and reusable frontend components, dramatically reducing the feature delivering time.
- Collaborated with cross-functional teams to ensure seamless frontend-backend integration, enhancing system performance.

### Coordinator / Developer | MTST Technology Center | Aug 2021 - Present (Volunteer)

- Coordinated hardware and software development for garden monitoring systems using soil and hydroponics techniques.
- Led a team in delivering 4 end-to-end IoT solutions, improving efficiency in urban gardening projects.
- Technologies used: React Native, Typescript, C++, ESP32, Redux, Styled Components, Github Pipelines.

### Agilist | AB InBev | Mar 2022 - Nov 2023

- Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements, increasing customer engagement by 35%.
- Implemented Kanban methodology, improving task visibility and team productivity by 150%.

### Mobile Developer | ParaChegar | Jan 2023 - Sep 2023

- Engineered React Native applications with intuitive UI/UX.
- Integrated native modules for push notifications and location-based services, enhancing app functionality.

### Developer / Agilist | Eldorado Research Institute | Apr 2018 - Feb 2022

- Promoted agile practices across 5 squads, leading to a 30% improvement in delivery times.
- Developed scalable B2B and B2C solutions for backend and frontend using .NET, Docker, and AWS.
- Designed and implemented continuous integration pipelines, reducing deployment time by 50%.

### QA Analyst | SIDIA - Samsung Institute | Nov 2015 - Mar 2018

- Conducted QA testing for Samsung products.

### Junior Developer | INDT (Nokia Institute) | May 2015 - Oct 2015

- Automated hardware and software testing processes.

## Education

- **Embedded Android Specialization** – UEA (State University of Amazonas) - 2019/2020
- **Bachelor's in Systems Analysis and Development** – Estácio de Sá University - 2017/2018
- **Systems Analysis and Development** – UEA (State University of Amazonas) - 2013/2017
"#;

#[tokio::main]
async fn main() {
    let host = env::var("SERVER_HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or("3001".to_string());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/cv", get(markdown_cv))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "I'm working!!"
}

async fn markdown_cv() -> Html<&'static str> {
    Html(MARKDOWN_CONTENT)
}