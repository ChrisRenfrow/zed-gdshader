use zed_extension_api as zed;

struct GdshaderExtension;

impl zed::Extension for GdshaderExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed_extension_api::LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Err("Not implemented".into())
    }
}

zed::register_extension!(GdshaderExtension);
