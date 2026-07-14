use std::sync::Arc;

use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{
    aggregates::{
        contribution::aggregate::{Contribution, ContributionAggregateServices},
        course::{aggregate::Course, service::CourseExistanceService},
        faq::{
            aggregate::{Faq, FaqAggregateServices},
            services::FaqExistanceService,
        },
        link::{
            aggregate::{Link, LinkAggregateServices},
            services::{LinkExistanceService, LinkServices, LinkValidityService},
        },
        project_idea::aggregate::{ProjectIdea, ProjectIdeaAggregateServices},
    },
    config::AppConfig,
    queries::{
        contribution::{ContributionListQuery, ContributionProcessManager, ContributionQuery},
        course::{CourseListQuery, CourseQuery},
        faq::CourseFaqQuery,
        link::CourseLinkQuery,
        project_idea::CourseProjectIdeaQuery,
        test_logging_query,
    },
    views::course::active_detailed::{ActiveCourseViewRepo, CourseDetailViewRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub cqrs: Arc<Cqrs>,
    pub course_view_repo: ActiveCourseViewRepo,
    pub pool: Pool<Postgres>,
}

#[derive(Clone)]
pub struct Cqrs {
    pub course: Arc<PostgresCqrs<Course>>,
    pub link: Arc<PostgresCqrs<Link>>,
    pub project_idea: Arc<PostgresCqrs<ProjectIdea>>,
    pub faq: Arc<PostgresCqrs<Faq>>,
    pub contribution: Arc<PostgresCqrs<Contribution>>,
}

pub async fn get(config: &AppConfig) -> AppState {
    // Set up database connection
    let db = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Should be able to connect to database");

    // Migrate database
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    // Queries setup
    let logging_query = test_logging_query::SimpleLoggingQuery {};

    let course_view_repo: Arc<CourseDetailViewRepo> = Arc::new(PostgresViewRepository::new(
        "course_detail_view",
        db.clone(),
    ));
    let mut course_detail_query: CourseQuery = CourseQuery::new(course_view_repo.clone());
    course_detail_query.use_error_handler(Box::new(|e| println!("{e}")));
    let course_list_query = CourseListQuery::new(db.clone());

    let course_queries: Vec<Box<dyn Query<Course>>> = vec![
        Box::new(course_detail_query),
        Box::new(course_list_query),
        Box::new(logging_query.clone()),
    ];
    let course_cqrs = Arc::new(postgres_es::postgres_cqrs(db.clone(), course_queries, ()));

    let link_queries: Vec<Box<dyn Query<Link>>> = vec![
        Box::new(logging_query.clone()),
        Box::new(CourseLinkQuery::new(course_view_repo.clone())),
    ];
    let link_aggregate_services = LinkAggregateServices {
        course: CourseExistanceService(db.clone()),
        link: LinkValidityService(reqwest::Client::new()),
    };
    let link_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        link_queries,
        link_aggregate_services,
    ));
    let faq_queries: Vec<Box<dyn Query<Faq>>> = vec![
        Box::new(logging_query.clone()),
        Box::new(CourseFaqQuery::new(course_view_repo.clone())),
    ];
    let faq_aggregate_services = FaqAggregateServices {
        course: CourseExistanceService(db.clone()),
    };
    let faq_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        faq_queries,
        faq_aggregate_services,
    ));

    let project_idea_queries: Vec<Box<dyn Query<ProjectIdea>>> = vec![
        Box::new(logging_query.clone()),
        Box::new(CourseProjectIdeaQuery::new(course_view_repo.clone())),
    ];
    let project_idea_aggregate_services = ProjectIdeaAggregateServices {
        course: CourseExistanceService(db.clone()),
    };
    let project_idea_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        project_idea_queries,
        project_idea_aggregate_services,
    ));

    let contribution_queries: Vec<Box<dyn Query<Contribution>>> = vec![
        Box::new(ContributionListQuery::new(db.clone())),
        Box::new(logging_query.clone()),
        Box::new(ContributionQuery),
        Box::new(ContributionProcessManager::new(
            db.clone(),
            link_cqrs.clone(),
            faq_cqrs.clone(),
        )),
    ];
    let contribution_aggregate_services = ContributionAggregateServices {
        link: LinkServices(
            LinkValidityService(reqwest::Client::new()),
            LinkExistanceService(db.clone()),
        ),
        course: CourseExistanceService(db.clone()),
        faq: FaqExistanceService(db.clone()),
    };
    let contribution_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        contribution_queries,
        contribution_aggregate_services,
    ));

    AppState {
        cqrs: Arc::new(Cqrs {
            course: course_cqrs,
            link: link_cqrs,
            project_idea: project_idea_cqrs,
            faq: faq_cqrs,
            contribution: contribution_cqrs,
        }),
        course_view_repo: ActiveCourseViewRepo(course_view_repo),
        pool: db,
    }
}
