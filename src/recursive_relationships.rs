pub const QUERY: &str = "
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
            level, concept_id, related_concept_id;";

#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct RelationshipDetails {
    pub concept_id: i32,
    pub concept_name: String,
    pub related_concept_id: i32,
    pub related_concept_name: String,
    pub standard_concept: Option<String>,
    pub relationship_id: String,
    level: i32,
}
