use std::env;
use sqlx::postgres::PgPoolOptions;
mod recursive_relationships;
use recursive_relationships::RelationshipDetails;
mod graph_conversion;
use graph_conversion::{rows_to_graph, OMOPGraph};

use axum::{
    routing::get,
    Router,
    extract::{State, Path},
    Json,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    pool: sqlx::PgPool,
}

const GOOD_RELATIONSHIPS: [&'static str; 83] = [
    "Maps to",
    "Is a",
    "SPL - RxNorm",
    "Has Module",
    "Has status",
    "Concept replaces",
    "Has method",
    "ATC - RxNorm",
    "Has finding site",
    "Component of",
    "Has property",
    "Has asso morph",
    "Concept same_as from",
    "Drug class of drug",
    "Concept poss_eq from",
    "Has interprets",
    "Has access",
    "Concept was_a from",
    "ATC - RxNorm sec up",
    "Active ing of",
    "Has pathology",
    "Acc device used by",
    "Device used by",
    "Causative agent of",
    "Has Dose form",
    "Source - RxNorm eq",
    "Subst used by",
    "SNOMED - RxNorm eq",
    "Dir device of",
    "Intent of",
    "Due to of",
    "Maps to value",
    "Has interpretation",
    "Has occurrence",
    "Basis str subst of",
    "Prec ingredient of",
    "Plays role",
    "Dir subst of",
    "Has disposition",
    "Has relat context",
    "Has temporal context",
    "Focus of",
    "Has clinical course",
    "ATC - RxNorm pr lat",
    "Using finding method",
    "Has inherent",
    "Has finding context",
    "Asso finding of",
    "ATC - RxNorm pr up",
    "Followed by",
    "Occurs after",
    "Modification of",
    "Has route",
    "Using finding inform",
    "Spec active ing of",
    "Asso with finding",
    "Disp dose form of",
    "SNOMED - ATC eq",
    "ATC - RxNorm sec lat",
    "Proc device of",
    "Specimen subst of",
    "Concept alt_to from",
    "Has count of ing",
    "Has dev intend site",
    "During",
    "Affected by process",
    "Comp material of",
    "Has prod character",
    "Is sterile",
    "Manifestation of",
    "Has absorbability",
    "Temp related to",
    "Has state of matter",
    "Indir device of",
    "Specimen identity of",
    "Process output of",
    "Precondition of",
    "Coating material of",
    "Has severity",
    "Has variant",
    "Filling of",
    "Surf character of",
    "Before"
];

async fn query_limited_relationships(
    State(state): State<Arc<AppState>>,
    Path((starting_concept, max_depth)): Path<(i64, i64)>,
) -> Json<OMOPGraph> {
    println!("Querying database with starting concept {:?} to max depth of {:?}", &starting_concept, &max_depth);
    let relationships: Vec<RelationshipDetails> = sqlx::query_as::<_,RelationshipDetails>(
        recursive_relationships::QUERY_LIMITED_RELATIONSHIPS
    )
        .bind(starting_concept)
        .bind(max_depth)
        .bind(GOOD_RELATIONSHIPS)
        .fetch_all(&state.pool)
        .await
        .expect("Error in querying the database");
    println!("Database returned {:?} relationships", &relationships.len());
    
    let result = rows_to_graph(relationships);
    println!("Converted relationships to graph");

    Json(result)
}

// Yes, this should be one function with an argument as to whether to limit the relationships, I just can't be bothered to figure out routing nicely
async fn query_all_relationships(
    State(state): State<Arc<AppState>>,
    Path((starting_concept, max_depth)): Path<(i64, i64)>,
) -> Json<OMOPGraph> {
    println!("Querying database with starting concept {:?} to max depth of {:?}", &starting_concept, &max_depth);
    let relationships: Vec<RelationshipDetails> = sqlx::query_as::<_,RelationshipDetails>(
        recursive_relationships::QUERY_ALL_RELATIONSHIPS
    )
        .bind(starting_concept)
        .bind(max_depth)
        .fetch_all(&state.pool)
        .await
        .expect("Error in querying the database");
    println!("Database returned {:?} relationships", &relationships.len());

    let result = rows_to_graph(relationships);
    println!("Converted relationships to graph");

    Json(result)
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Couldn't read a .env file");
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        env::var("DB_USER").expect("Couldn't read DB_USER"),
        env::var("DB_PASSWORD").expect("Couldn't read DB_PASSWORD"),
        env::var("DB_HOST").expect("Couldn't read DB_HOST"),
        env::var("DB_PORT").expect("Couldn't read DB_PORT"),
        env::var("DB_NAME").expect("Couldn't read DB_NAME")
        );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Error connecting to database");

    let state = Arc::new(AppState {pool});

    let app = Router::new()
        .route("/recursive_relationships_limited/:starting_concept/:max_depth", get(query_limited_relationships))
        .route("/recursive_all_relationships/:starting_concept/:max_depth", get(query_all_relationships))
        .layer(CorsLayer::permissive())
        .with_state(state);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
