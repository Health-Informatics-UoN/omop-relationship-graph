# OMOP relationship graphs

[OHDSI's](https://www.ohdsi.org/) [OMOP Common Data Model](https://www.ohdsi.org/data-standardization/) contains vocabularies of **concepts** which connect to one another by **relationships**.
A useful representation of data with relationships is a graph. This app provides a REST API that queries an existing Postgres instance containing the OMOP-CDM and returns a local graph of relationships.
The input is an OMOP concept ID and a maximum depth, and the output is a JSON object mapping the relationships the matching concept has, out to the depth of the maximum depth.

## Why?

## How?

## Usage
### Database

### 
