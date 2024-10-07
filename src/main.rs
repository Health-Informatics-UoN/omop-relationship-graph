use sqlx::postgres::PgPoolOptions;

#[derive(sqlx::FromRow, Debug)]
struct RelationshipDetails {
    concept_id: i32,
    concept_name: String,
    related_concept_id: i32,
    related_concept_name: String,
    standard_concept: Option<String>,
    relationship_id: String,
    level: i32,
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:5432/omop").await?;

    let relationships: Vec<RelationshipDetails> = sqlx::query_as::<_,RelationshipDetails>("
        WITH RECURSIVE concept_hierarchy AS (
            -- Base case
        SELECT 
            c.concept_id,
            c.concept_name,
            c.standard_concept,
            cr.relationship_id,
            cr.concept_id_2 AS related_concept_id,
            rc.concept_name AS related_concept_name,
            1 AS level,
            ARRAY[c.concept_id] AS visited_concepts
        FROM 
            cdm.concept c
        JOIN 
            cdm.concept_relationship cr ON c.concept_id = cr.concept_id_1
        JOIN 
            cdm.concept rc ON cr.concept_id_2 = rc.concept_id
        WHERE 
            c.concept_id = $1
    
        UNION ALL
    
        -- Recursive case
        SELECT 
            rc.concept_id,
            rc.concept_name,
            rc.standard_concept,
            cr.relationship_id,
            cr.concept_id_2 AS related_concept_id,
            next_c.concept_name AS related_concept_name,
            ch.level + 1 AS level,
            ch.visited_concepts || rc.concept_id
        FROM 
            concept_hierarchy ch
        JOIN 
            cdm.concept rc ON ch.related_concept_id = rc.concept_id
        JOIN 
            cdm.concept_relationship cr ON rc.concept_id = cr.concept_id_1
        JOIN 
            cdm.concept next_c ON cr.concept_id_2 = next_c.concept_id
        WHERE 
            ch.level < $2  -- Maximum depth
            AND rc.standard_concept IS NULL
            AND cr.relationship_id = ANY($3)
            AND NOT (cr.concept_id_2 = ANY(ch.visited_concepts))  -- Prevent cycles
        )
        SELECT 
            concept_id,
            concept_name,
            related_concept_id,
            related_concept_name,
            standard_concept,
            relationship_id,
            level
        FROM 
            concept_hierarchy
        ORDER BY 
            level, concept_id, related_concept_id;")
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
