use crate::recursive_relationships::RelationshipDetails;
use std::collections::HashSet;

#[derive(Debug, serde::Serialize)]
struct OMOPNode {
    id: i32,
    name: String,
    standard_concept: Option<String>,
}

#[derive(Debug, serde::Serialize)]
struct OMOPEdge {
    source_id: i32,
    target_id: i32,
    relationship_id: String,
}

#[derive(Debug, serde::Serialize)]
pub struct OMOPGraph{
    nodes: Vec<OMOPNode>,
    edges: Vec<OMOPEdge>,
}

fn get_nodes(relationships: &Vec<RelationshipDetails>) -> Vec<OMOPNode> {
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for rel in relationships {
        // If I were smarter and less lazy, I would make it so only the related concepts are
        // checked because apart from the starting concept, all of the concept_id are going to be
        // in the related concepts anyway. This would make it ~2x faster TODO?
        if seen.insert(rel.concept_id) {
            result.push(OMOPNode {
                id: rel.concept_id.clone(),
                name: rel.concept_name.clone(),
                standard_concept: rel.standard_concept.clone(),
            })
        };
        if seen.insert(rel.related_concept_id) {
            result.push(
                OMOPNode {
                    id: rel.related_concept_id.clone(),
                    name: rel.related_concept_name.clone(),
                    standard_concept: rel.related_standard_concept.clone(),
                }
            )
        }
    }

    result
}

pub fn rows_to_graph(relationships: Vec<RelationshipDetails>) -> OMOPGraph {
    let nodes = get_nodes(&relationships);
    let edges: Vec<OMOPEdge> = relationships.iter().map(
        |e| OMOPEdge{
            source_id: e.concept_id.clone(),
            target_id: e.related_concept_id.clone(),
            relationship_id: e.relationship_id.clone(),
        }
    )
        .collect();
    
    OMOPGraph{
        nodes: nodes,
        edges: edges
    }
}
