use std::collections::HashMap;

use indy_api_types::validation::Validatable;

pub use indy_vdr::ledger::requests::schema::{SchemaV1, MAX_ATTRIBUTES_COUNT, AttributeNames};
pub use indy_vdr::ledger::identifiers::schema::SchemaId;
use indy_vdr::utils::validation::Validatable as VdrValidatable;
use indy_vdr::config::VdrResultExt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "ver")]
pub enum Schema {
    #[serde(rename = "1.0")]
    SchemaV1(SchemaV1)
}

impl Schema {
    pub fn to_unqualified(self) -> Schema {
        match self {
            Schema::SchemaV1(schema) => {
                Schema::SchemaV1(SchemaV1 {
                    id: schema.id.to_unqualified(),
                    name: schema.name,
                    version: schema.version,
                    attr_names: schema.attr_names,
                    seq_no: schema.seq_no,
                })
            }
        }
    }
}

impl From<Schema> for SchemaV1 {
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::SchemaV1(schema) => schema
        }
    }
}

pub type Schemas = HashMap<SchemaId, Schema>;

pub fn schemas_map_to_schemas_v1_map(schemas: Schemas) -> HashMap<SchemaId, SchemaV1> {
    schemas.into_iter().map(|(schema_id, schema)| { (schema_id, SchemaV1::from(schema)) }).collect()
}

impl Validatable for Schema {
    fn validate(&self) -> Result<(), String> {
        match self {
            Schema::SchemaV1(schema) => {
                schema.validate().map_err_string()?;
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indy_vdr::common::did::DidValue;

    fn _did() -> DidValue {
        DidValue("NcYxiDXkpYi6ov5FcYDi1e".to_string())
    }

    fn _did_qualified() -> DidValue {
        DidValue("did:sov:NcYxiDXkpYi6ov5FcYDi1e".to_string())
    }

    fn _schema_id_seq_no() -> SchemaId {
        SchemaId("1".to_string())
    }

    fn _schema_id_unqualified() -> SchemaId {
        SchemaId("NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0".to_string())
    }

    fn _schema_id_qualified() -> SchemaId {
        SchemaId("schema:sov:did:sov:NcYxiDXkpYi6ov5FcYDi1e:2:gvt:1.0".to_string())
    }

    fn _schema_id_invalid() -> SchemaId {
        SchemaId("NcYxiDXkpYi6ov5FcYDi1e:2".to_string())
    }

    mod to_unqualified {
        use super::*;

        #[test]
        fn test_schema_id_unqualify_for_id_as_seq_no() {
            assert_eq!(_schema_id_seq_no(), _schema_id_seq_no().to_unqualified());
        }

        #[test]
        fn test_schema_id_parts_for_id_as_unqualified() {
            assert_eq!(_schema_id_unqualified(), _schema_id_unqualified().to_unqualified());
        }

        #[test]
        fn test_schema_id_parts_for_id_as_qualified() {
            assert_eq!(_schema_id_unqualified(), _schema_id_qualified().to_unqualified());
        }

        #[test]
        fn test_schema_id_parts_for_invalid_unqualified() {
            assert_eq!(_schema_id_invalid(), _schema_id_invalid().to_unqualified());
        }
    }

    mod parts {
        use super::*;

        #[test]
        fn test_schema_id_parts_for_id_as_seq_no() {
            assert!(_schema_id_seq_no().parts().is_none());
        }

        #[test]
        fn test_schema_id_parts_for_id_as_unqualified() {
            let (did, _, _) = _schema_id_unqualified().parts().unwrap();
            assert_eq!(_did(), did);
        }

        #[test]
        fn test_schema_id_parts_for_id_as_qualified() {
            let (did, _, _) = _schema_id_qualified().parts().unwrap();
            assert_eq!(_did_qualified(), did);
        }

        #[test]
        fn test_schema_id_parts_for_invalid_unqualified() {
            assert!(_schema_id_invalid().parts().is_none());
        }
    }

    mod validate {
        use super::*;

        #[test]
        fn test_validate_schema_id_as_seq_no() {
            _schema_id_seq_no().validate().unwrap();
        }

        #[test]
        fn test_validate_schema_id_as_unqualified() {
            _schema_id_unqualified().validate().unwrap();
        }

        #[test]
        fn test_validate_schema_id_as_fully_qualified() {
            _schema_id_qualified().validate().unwrap();
        }

        #[test]
        fn test_validate_schema_id_for_invalid_unqualified() {
            _schema_id_invalid().validate().unwrap_err();
        }

        #[test]
        fn test_validate_schema_id_for_invalid_fully_qualified() {
            let id = SchemaId("schema:sov:NcYxiDXkpYi6ov5FcYDi1e:2:1.0".to_string());
            id.validate().unwrap_err();
        }
    }

    mod test_schema_validation {
        use super::*;

        #[test]
        fn test_valid_schema() {
            let schema_json = json!({
                "id": _schema_id_qualified(),
                "name": "gvt",
                "ver": "1.0",
                "version": "1.0",
                "attrNames": ["aaa", "bbb", "ccc"],
            }).to_string();

            let schema: Schema = serde_json::from_str(&schema_json).unwrap();
            schema.validate().unwrap();
            match schema {
                Schema::SchemaV1(schema) => {
                    assert_eq!(schema.name, "gvt");
                    assert_eq!(schema.version, "1.0");
                }
            }
        }

        #[test]
        fn test_invalid_name_schema() {
            let schema_json = json!({
                "id": _schema_id_qualified(),
                "name": "gvt1",
                "ver": "1.0",
                "version": "1.0",
                "attrNames": ["aaa", "bbb", "ccc"],
            }).to_string();

            let schema: Schema = serde_json::from_str(&schema_json).unwrap();
            schema.validate().unwrap_err();
        }

        #[test]
        fn test_invalid_version_schema() {
            let schema_json = json!({
                "id": _schema_id_qualified(),
                "name": "gvt",
                "ver": "1.0",
                "version": "1.1",
                "attrNames": ["aaa", "bbb", "ccc"],
            }).to_string();

            let schema: Schema = serde_json::from_str(&schema_json).unwrap();
            schema.validate().unwrap_err();
        }
    }
}