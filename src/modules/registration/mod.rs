pub mod dto {
    mod email_registration;
    pub use email_registration::EmailRegistrationDTO;
}

pub mod strategies {
    mod registration;
    pub use registration::RegistrationStrategy;

    mod email_registration;
    pub use email_registration::EmailRegistrationStrategy;
}

pub mod services {
    mod registration;
    pub use registration::RegistrationServiceFactory;
}