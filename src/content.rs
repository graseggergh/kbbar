pub mod content {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct DisplayContent {
        pub icon_top_left: Option<char>,
        pub text_top_left: Option<String>,
        pub icon_top_right: Option<char>,
        pub text_top_right: Option<String>,
        pub icon_bottom_left: Option<char>,
        pub text_bottom_left: Option<String>,
        pub icon_bottom_right: Option<char>,
        pub text_bottom_right: Option<String>,
    }

    impl Default for DisplayContent {
        fn default() -> Self {
            DisplayContent {
                icon_top_left: Some('i'),
                text_top_left: Some("text".to_string()),
                icon_top_right: Some('i'),
                text_top_right: Some("text".to_string()),
                icon_bottom_left: Some('i'),
                text_bottom_left: Some("text".to_string()),
                icon_bottom_right: Some('i'),
                text_bottom_right: Some("text".to_string()),
            }
        }
    }
}
