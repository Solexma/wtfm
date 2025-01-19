pub struct Messages;

impl Messages {
    pub const PROJECT_TIPS: &'static str = "Tips for project information:
- Project name should be clear and descriptive
- Version should follow semantic versioning (X.Y.Z)
- Description should explain what your project does";

    pub const AUTHORS_TIPS: &'static str = "Tips for authors:
- Use full names when possible
- Email should be valid and accessible
- Multiple authors can be added";

    pub const LICENSE_TIPS: &'static str = "Tips for licensing:
- Choose a license that fits your project's needs
- Consider compatibility with dependencies
- Custom licenses should be well documented";

    pub const FEATURES_TIPS: &'static str = "Tips for features:
- List main functionalities
- Keep descriptions concise
- Highlight unique selling points";

    pub const INSTALL_TIPS: &'static str = "Tips for installation:
- List all prerequisites
- Include step-by-step instructions
- Mention any configuration needed";

    pub const CICD_TIPS: &'static str = "Tips for CI/CD:
- Choose appropriate platform
- Configure essential checks
- Define branch strategy";

    pub const SUCCESS_SECTION: &'static str = "Section configured successfully!";
    pub const CONFIRM_DELETE: &'static str = "Are you sure you want to delete this?";
    pub const ERR_INVALID_EMAIL: &'static str = "Invalid email format";
    pub const ERR_INVALID_PROJECT_NAME: &'static str = "Project name cannot be empty";
    pub const ERR_INVALID_VERSION: &'static str = "Invalid version format (use X.Y.Z)";
}
