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

Accomplished software engineer with over 9 years of experience specializing in frontend and backend development. Expert in React, Next.js, Typescript, and Rust, with a proven track record of leading cross-functional teams and driving agile practices. Demonstrated success in delivering high-quality software solutions, improving performance metrics, and streamlining workflows with modern cloud and containerization technologies. Ready to leverage my expertise and leadership skills to solve complex technical challenges.

## Technical Skills

- **Frontend:** React Native, React, Next.js, Typescript, Javascript, HTML, CSS, Redux, Redux Toolkit, MobX, Hooks, Design Patterns.
- **Backend:** Rust, .NET, NodeJS, NestJS, Prisma, TypeORM, RESTful API, SOAP, Docker, Kubernetes, AWS, Google Cloud, Azure.
- **Tools:** Firebase, GitHub Actions, Jenkins, Storybook, Pipelines, Continuous Integration (CI), Continuous Deployment (CD).
- **Mobile and IoT:** Android, iOS, Kotlin, Java, BLE, Push Notifications, ESP32, FFmpeg.
- **Agile:** Kanban, Scrum, Agile Coaching, Team Building, Risk Management, Capacity Management, Team Metrics.

## Professional Experience

### Senior Software Engineer | BairesDev | Dec 2023 - Present

- Working for the **Hensall Coorp client**, developing both frontend and backend solutions:
  - **Frontend**: React, Redux, Styled Components.
  - **Backend**: NestJS, TypeORM, and Prisma for two services, with unit and integration testing using Jest.
- Built and maintained CI/CD pipelines using GitHub Actions, automating testing, linting, and deployments.
- Leveraged containerization for local development environments, including databases and auxiliary services.
- For the **Lenslock client**, developed reusable frontend components with React, Storybook, and Typescript, reducing delivery cycles by 25%.

### Coordinator / Developer | MTST Technology Center | Aug 2021 - Present (Volunteer)

- Designed and implemented a **video and image processing service in Rust**, using FFmpeg to manipulate videos and audio codecs.
- Delivered IoT solutions integrating ESP32 with React Native dashboards for real-time data monitoring in urban gardening projects.
- Introduced CI/CD workflows using Docker and Kubernetes, streamlining deployments and improving efficiency.

### Agilist | AB InBev | Mar 2022 - Nov 2023

- Led the Feature Activation team, successfully delivering 37 new features and 79 global improvements, increasing customer engagement by 35%.
- Implemented Kanban methodology, improving task visibility and team productivity by 150%.

### Mobile Developer | ParaChegar | Jan 2023 - Sep 2023

- Developed intuitive and user-friendly React Native applications.
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