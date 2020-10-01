#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use lombok_rs::{
        AllArgsConstructor,
        Builder,
        Getter,
        GetterMut,
        Setter,
    };

    #[derive(Getter, GetterMut, Setter, AllArgsConstructor, Builder)]
    pub struct TestStructure {
        age: u8,
        nick: &'static str,
        position: String,
        languages: Vec<String>,
        hobby: Box<String>,
    }

    #[test]
    fn test_getters() {
        let data = TestStructure {
            age: 25,
            nick: "sokomishalov",
            position: "developer".to_string(),
            languages: vec!["rust".to_string(), "kotlin".to_string()],
            hobby: Box::new("soccer".to_string()),
        };

        assert_eq!(&data.age, data.get_age());
        assert_eq!(&data.nick, data.get_nick());
        assert_eq!(&data.position, data.get_position());
        assert_eq!(&data.languages, data.get_languages());
        assert_eq!(&data.hobby, data.get_hobby());
    }
}
