mod test {
    use solalumin_attribute_event::schema_generator;
    use bryte_descriptor_state::states::{Discriminator, SchemaEvent};
    use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};

    #[schema_generator]
    #[derive( Debug, Default, PartialEq, Eq)]
    pub struct NonAnchorExample {
        pub name: String,
        pub tags: Vec<String>
    }

    #[test]
    fn test_data() {
        let example = NonAnchorExample {
            name: "".to_string(),
            tags: vec![],
        };
        let data = example.data();
        println!("{:?}", data);
    }

    #[test]
    fn test_generate_schema() {
        let example = NonAnchorExample {
            name: "".to_string(),
            tags: vec![],
        };
        let schema_vec = example.generate_schema();
        println!("{:?}", schema_vec);
    }

    #[test]
    fn test_discriminator() {
        let discriminator = NonAnchorExample::discriminator();
        println!("{:?}", discriminator);
    }
}