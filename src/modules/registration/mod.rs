pub mod controllers {
    mod registration;
    pub use registration::init;
}

pub mod dto {
    mod email_registration;
    pub use email_registration::EmailRegistrationDTO;
}

pub mod errors {
    mod registration;
    pub use registration::RegistrationError;
}

pub mod strategies {
    mod registration;
    pub(crate) use registration::RegistrationStrategy;

    mod email_registration;
    pub(crate) use email_registration::EmailRegistrationStrategy;
}

pub mod services {
    mod registration;
    pub(crate) use registration::RegistrationServiceFactory;
}