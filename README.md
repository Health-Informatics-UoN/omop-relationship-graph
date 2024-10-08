# OMOP relationship graphs

[OHDSI's](https://www.ohdsi.org/) [OMOP Common Data Model](https://www.ohdsi.org/data-standardization/) contains vocabularies of **concepts** which connect to one another by **relationships**.
A useful representation of data with relationships is a graph. This app provides a REST API that queries an existing Postgres instance containing the OMOP-CDM and returns a local graph of relationships.
The input is an OMOP concept ID and a maximum depth, and the output is a JSON object mapping the relationships the matching concept has, out to the depth of the maximum depth.

## Why?

I wanted to be able to examine the relationships an OMOP concept has. You can look at this at one level fairly simply. This might not be all you need though.
At the core of the OMOP-CDM are "standard concepts". It's important to know the standard concepts that a concept is related to.
This relationship might be at more than one remove. If you want to examine the neighbourhood of a concept, privileging standard concepts, something more complex is required.

## How?
## Database query
The core of this app is a query to the OMOP-CDM.

```sql
WITH RECURSIVE concept_hierarchy AS (
    -- Base  
    SELECT 
        c.concept_id,
        c.concept_name,
        c.standard_concept,
        cr.relationship_id,
        cr.concept_id_2 AS related_concept_id,
        rc.concept_name AS related_concept_name,
        rc.standard_concept AS related_standard_concept,
        1 AS level,
        ARRAY[c.concept_id] AS visited_concepts
    FROM 
        cdm.concept c
    JOIN 
        cdm.concept_relationship cr ON c.concept_id = cr.concept_id_1
    JOIN 
        cdm.concept rc ON cr.concept_id_2 = rc.concept_id
    WHERE 
        c.concept_id = {starting_concept}
    
    UNION ALL
    
    -- Recursive case
    SELECT 
        rc.concept_id,
        rc.concept_name,
        rc.standard_concept,
        cr.relationship_id,
        cr.concept_id_2 AS related_concept_id,
        next_c.concept_name AS related_concept_name,
        next_c.standard_concept AS related_standard_concept,
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
        ch.level < {Maximum depth}
        AND rc.standard_concept IS NULL
        AND NOT (cr.concept_id_2 = ANY(ch.visited_concepts))  -- Prevent cycles
    )
    SELECT 
        concept_id,
        concept_name,
        related_concept_id,
        related_concept_name,
        standard_concept,
        related_standard_concept,
        relationship_id,
        level
    FROM 
        concept_hierarchy
    ORDER BY 
        level, concept_id, related_concept_id;
```

Definitely the longest query I've written.

It's a recursive query.
The base case is stated first. This finds the starting concept in the concept table, and finds its relationships in the concept_relationship table. It sets `level` to 1, and adds the concepts it has found to a `visited_concepts` array. If you set the maximum depth to 1, this is all it does. However, if you don't, the recursive case makes the same query for each of the related concepts. There are three restrictions on when it does this:

1. The `level` is increased by 1 each round, and the query will stop once it hits a defined number. This means that at max depth 1, you just get your starting concept's related concepts, at max depth 2, you get the relationships of related concepts, at max depth 3 you get the relationships of the concepts related to your concept, etc. This can balloon. Theoretically, the third restriction would stop this from being infinitely recursive, but the query would take ages. I wouldn't recommend going above level 5
2. If the concept under examination that round is a standard or classification concept, its relationships are not found. This is because these are the important concepts that everything else is ultimately defined by.
3. The concept hasn't already been explored. This is to stop the query needlessly going round in a cycle.

## Graph conversion
The query returns rows from the database with fields describing the concepts either side of a relationship, and the nature of the relationship. This contains a lot of redundancy.

A graph is a set of vertices, or nodes, and a set of edges. As both the nodes and edges hold information about themselves other than what they are connected to, a reasonably efficient way to store the info is to define both of these sets.

The program reads through all of the concept_id and related_concept_id fields returned from the query. If the id is not already found in the set of nodes, its details are added to the set. It then runs through the query result and takes just the ids of the source and target, and the relationship_id and adds those to the set of edges.

## Usage
### Database

You need to have access to an OMOP-CDM database. This needs to be running on Postgres because [`sqlx`](https://github.com/launchbadge/sqlx), which runs the query against the database, needs to use a feature of PostgreSQL syntax. If you don't have access, [omop-lite](https://github.com/AndyRae/omop-lite) can get a local instance running.

Once you have access to an OMOP-CDM database, you need to put your database credentials into a `.env` file in the home directory with the following key/value pairs.

```
DB_HOST="Database host"
DB_USER="Database user"
DB_PASSWORD="Database password"
DB_NAME="Database name"
DB_PORT="Database port"
```

Then just `cargo run`

### API

HTTP GET requests can be made to `http://localhost:3000/`

There are two endpoints:

- `http://localhost:3000/recursive_relationships_limited/:starting_concept/:max_depth`
- `http://localhost:3000/recursive_all_relationships/:starting_concept/:max_depth`

The endpoints do basically the same thing. The difference is that the limited version filters by relationship. There are a lot of different relationship types in OMOP-CDM. If you retrieve all these relationships, you end up with a lot of output. I've selected a very biased list of ones I think are important. If you don't want to selectively return relationships based on my judgement, just use the `recursive_all_relationships` endpoint.

### Output format

Requests to the API return the same JSON format. For example, the local graph for concept 3955088 is

```json
{
    "nodes":
        [
            {"id":3955088,"name":"Stomach awareness nausea vomiting when reading while travelling","standard_concept":null},
            {"id":30284,"name":"Motion sickness","standard_concept":"S"},
            {"id":1340204,"name":"History of event","standard_concept":"S"}
        ],
    "edges":
        [
            {"source_id":3955088,"target_id":30284,"relationship_id":"Maps to value"},
            {"source_id":3955088,"target_id":1340204,"relationship_id":"Maps to"}
        ]
}
```

There are two attributes

1. `nodes`: an array of objects representing all the concepts involved in the graph, each with three attributes
    - `id`: the concept_id for that concept
    - `name`: the concept_name for that concept
    - `standard_concept`: either `'S'`, `'C'`, or `null`
2. `edges`: an array of objects representing the edges of the relationship graph, each with three attributes
    - `source_id`: the concept_id of the source concept for that edge
    - `target_id`: the concept_id of the target concept for that edge
    - `relationship_id`: the relationship_id of the relationship represented by that edge

## Feedback

If you have any feedback, [email me](mailto:james.mitchell-white1@nottingham.ac.uk)
