use std::sync::Arc;

use axum::extract::FromRef;
use cqrs_es::Query;
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::{
    aggregates::{
        course::{aggregate::Course, service::CourseServices},
        faq::{aggregate::Faq, service::FaqAggregateServices},
        link::{
            aggregate::{Link, LinkAggregateServices},
            services::LinkServices,
        },
        project_idea::aggregate::{ProjectIdea, ProjectIdeaAggregateServices},
        resource::{
            aggregate::{Resource, ResourceAggregateServices},
            services::ResourceServices,
        },
    },
    config::AppConfig,
    queries::{
        course::{CourseListQuery, CourseQuery},
        faq::CourseFaqQuery,
        link::CourseLinkQuery,
        project_idea::CourseProjectIdeaQuery,
        resource::CourseResourceQuery,
        test_logging_query,
    },
    storage::Storage,
    views::course::active_detailed::{ActiveCourseViewRepo, CourseDetailViewRepo},
};

#[derive(Clone)]
pub struct AppState {
    pub cqrs: Arc<Cqrs>,
    pub course_view_repo: ActiveCourseViewRepo,
    pub pool: Pool<Postgres>,
    pub storage: Storage,
}

// Let handlers extract just the piece of state they need: the file routes ask
// for `State<Storage>`, repository-based handlers can ask for the pool.
impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Storage {
        state.storage.clone()
    }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Pool<Postgres> {
        state.pool.clone()
    }
}

#[derive(Clone)]
pub struct Cqrs {
    pub course: Arc<PostgresCqrs<Course>>,
    pub link: Arc<PostgresCqrs<Link>>,
    pub project_idea: Arc<PostgresCqrs<ProjectIdea>>,
    pub faq: Arc<PostgresCqrs<Faq>>,
    pub resource: Arc<PostgresCqrs<Resource>>,
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

    // Object storage (Garage). Constructing the client makes no network
    // calls, so this is safe at startup even if Garage is not up yet.
    let storage = Storage::from_env().await;

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
        course: CourseServices(db.clone()),
        link: LinkServices(reqwest::Client::new()),
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
        course: CourseServices(db.clone()),
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
        course: CourseServices(db.clone()),
    };
    let project_idea_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        project_idea_queries,
        project_idea_aggregate_services,
    ));

    let resource_queries: Vec<Box<dyn Query<Resource>>> = vec![
        Box::new(logging_query),
        Box::new(CourseResourceQuery::new(course_view_repo.clone())),
    ];
    let resource_aggregate_services = ResourceAggregateServices {
        course: CourseServices(db.clone()),
        // The aggregate verifies upload keys against Garage before recording
        // events that reference them (Storage::exists via HeadObject).
        resource: ResourceServices(storage.clone()),
    };
    let resource_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        resource_queries,
        resource_aggregate_services,
    ));

    AppState {
        cqrs: Arc::new(Cqrs {
            course: course_cqrs,
            link: link_cqrs,
            project_idea: project_idea_cqrs,
            faq: faq_cqrs,
            resource: resource_cqrs,
        }),
        course_view_repo: ActiveCourseViewRepo(course_view_repo),
        pool: db,
        storage,
    }
}
