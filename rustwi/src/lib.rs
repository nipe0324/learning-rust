mod controllers {
    mod root;

    pub use root::app;
}

mod response;

mod views {
    mod home;
    mod partial {
        mod tweet;

        pub use tweet::Tweet;
    }

    pub use home::Home;
    pub use partial::Tweet;
}

pub use controllers::app;
