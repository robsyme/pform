pub mod client;
pub mod models;
pub mod errors;
pub mod utils;
pub mod tui;

pub use client::SeqeraClient;
pub use models::*;
pub use errors::SeqeraError;

#[cfg(test)]
mod tests {
    use super::workspace::CreateWorkspaceRequestBuilder;
    use super::workspace::WorkspaceVisibility;
    use super::team::CreateTeamRequestBuilder;
    use super::organization::CreateOrganizationRequestBuilder;
    
    #[test]
    fn test_workspace_builder_public_api() {
        let request = CreateWorkspaceRequestBuilder::new()
            .name("test-workspace")
            .full_name("Test Workspace")
            .description("A test workspace")
            .visibility(WorkspaceVisibility::Private)
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_team_builder_public_api() {
        let request = CreateTeamRequestBuilder::new()
            .name("test-team")
            .description("A test team")
            .avatar_id("avatar123")
            .build();
        assert!(request.is_ok());
    }

    #[test]
    fn test_organization_builder_public_api() {
        let request = CreateOrganizationRequestBuilder::new()
            .name("test-org")
            .full_name("Test Organization")
            .description("A test organization")
            .location("Test Location")
            .website("https://test.org")
            .logo_id("logo123")
            .build();
        assert!(request.is_ok());
    }
}
