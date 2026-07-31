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
        project_idea::{
            aggregate::{ProjectIdea, ProjectIdeaAggregateServices},
            services::ProjectIdeaExistanceService,
        },
        report::aggregate::{Report, ReportAggregateServices},
        resource::{
            aggregate::{Resource, ResourceAggregateServices},
            services::ResourceServices,
        },
    },
    config::AppConfig,
    queries::{
        contribution::{ContributionListQuery, ContributionProcessManager, ContributionQuery},
        course::{CourseListQuery, CourseQuery},
        faq::CourseFaqQuery,
        link::CourseLinkQuery,
        project_idea::CourseProjectIdeaQuery,
        report::{ReportListQuery, ReportQuery},
        resource::CourseResourceQuery,
        test_logging_query,
    },
    storage::Storage,
    views::admin::report_detail::ReportDetailView,
    views::course::active_detailed::{ActiveCourseViewRepo, CourseDetailViewRepo},
};

use axum::extract::FromRef;
use cqrs_es::Query;
use jsonwebtoken::{DecodingKey, EncodingKey};
use openidconnect::{
    ClientId, ClientSecret, EndpointMaybeSet, EndpointNotSet, EndpointSet, IssuerUrl, RedirectUrl,
    core::{CoreClient, CoreProviderMetadata},
};
use postgres_es::{PostgresCqrs, PostgresViewRepository};
use reqwest::Client;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::sync::Arc;

pub type OidcClient = CoreClient<
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointMaybeSet,
    EndpointMaybeSet,
>;

type ReportDetailViewRepo = PostgresViewRepository<ReportDetailView, Report>;

#[derive(Clone)]
pub struct Cqrs {
    pub course: Arc<PostgresCqrs<Course>>,
    pub link: Arc<PostgresCqrs<Link>>,
    pub project_idea: Arc<PostgresCqrs<ProjectIdea>>,
    pub faq: Arc<PostgresCqrs<Faq>>,
    pub contribution: Arc<PostgresCqrs<Contribution>>,
    pub resource: Arc<PostgresCqrs<Resource>>,
    pub report: Arc<PostgresCqrs<Report>>,
}

#[derive(Clone)]
pub struct AppState {
    pub oidc_client: OidcClient,
    pub http_client: Client,
    pub karakterweb_api_key: String,
    pub cqrs: Arc<Cqrs>,
    pub course_view_repo: ActiveCourseViewRepo,
    pub pool: Pool<Postgres>,
    pub storage: Storage,
    pub jwt_encode: Arc<EncodingKey>,
    pub jwt_decode: Arc<DecodingKey>,
}

pub async fn get(config: &AppConfig) -> AppState {
    let db = PgPoolOptions::new()
        .connect(&config.database_url)
        .await
        .expect("Should be able to connect to database");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Migrations should succeed");

    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client_secret = std::env::var("GOOGLE_SECRET").expect("GOOGLE_SECRET must be set");
    let redirect_uri =
        std::env::var("GOOGLE_REDIRECT_URI").expect("GOOGLE_REDIRECT_URI must be set");
    let jsonwebtoken_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let storage = Storage::from_env().await;

    let http_client: Client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let provider_metadata = CoreProviderMetadata::discover_async(
        IssuerUrl::new("https://accounts.google.com".to_string()).unwrap(),
        &http_client,
    )
    .await
    .expect("Provider metadata should be discoverable");

    let oidc_client = CoreClient::from_provider_metadata(
        provider_metadata,
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_uri).unwrap());

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
            project_idea_cqrs.clone(),
        )),
    ];
    let contribution_aggregate_services = ContributionAggregateServices {
        link: LinkServices(
            LinkValidityService(reqwest::Client::new()),
            LinkExistanceService(db.clone()),
        ),
        course: CourseExistanceService(db.clone()),
        faq: FaqExistanceService(db.clone()),
        project_idea: ProjectIdeaExistanceService(db.clone()),
    };
    let contribution_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        contribution_queries,
        contribution_aggregate_services,
    ));
    let resource_queries: Vec<Box<dyn Query<Resource>>> = vec![
        Box::new(logging_query.clone()),
        Box::new(CourseResourceQuery::new(course_view_repo.clone())),
    ];
    let resource_aggregate_services = ResourceAggregateServices {
        course: CourseExistanceService(db.clone()),
        resource: ResourceServices(storage.clone()),
    };
    let resource_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        resource_queries,
        resource_aggregate_services,
    ));

    let report_detail_view_repo: Arc<ReportDetailViewRepo> = Arc::new(PostgresViewRepository::new(
        "report_detail_view",
        db.clone(),
    ));
    let report_query = ReportQuery::new(report_detail_view_repo.clone());
    let report_list_query = ReportListQuery::new(db.clone());
    let report_queries: Vec<Box<dyn Query<Report>>> = vec![
        Box::new(logging_query.clone()),
        Box::new(report_query),
        Box::new(report_list_query),
    ];
    let report_aggregate_services = ReportAggregateServices {
        course: CourseExistanceService(db.clone()),
    };
    let report_cqrs = Arc::new(postgres_es::postgres_cqrs(
        db.clone(),
        report_queries,
        report_aggregate_services,
    ));

    AppState {
        oidc_client,
        http_client,
        karakterweb_api_key: config.karakterweb_api_key.clone(),
        cqrs: Arc::new(Cqrs {
            course: course_cqrs,
            link: link_cqrs,
            project_idea: project_idea_cqrs,
            faq: faq_cqrs,
            contribution: contribution_cqrs,
            resource: resource_cqrs,
            report: report_cqrs,
        }),
        course_view_repo: ActiveCourseViewRepo(course_view_repo),
        pool: db,
        storage,
        jwt_encode: Arc::new(EncodingKey::from_secret(jsonwebtoken_secret.as_ref())),
        jwt_decode: Arc::new(DecodingKey::from_secret(jsonwebtoken_secret.as_ref())),
    }
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Storage {
    fn from_ref(state: &AppState) -> Storage {
        state.storage.clone()
    }
}
