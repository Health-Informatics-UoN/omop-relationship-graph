use sqlx::postgres::PgPoolOptions;
mod recursive_relationships;
use recursive_relationships::RelationshipDetails;


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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/omop").await?;

    let relationships: Vec<RelationshipDetails> = sqlx::query_as::<_,RelationshipDetails>(
        recursive_relationships::QUERY
    )
        .bind(618919_i64)
        .bind(2_i64)
        .bind(GOOD_RELATIONSHIPS)
        .fetch_all(&pool).await?;

    println!("Fetched {:?} entries", &relationships.len());

    for relationship in relationships {
        println!("{:?}", &relationship)
    };
    Ok(())
}
