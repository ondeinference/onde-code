//! Onde Code's [`Profile`] for `ed-acp`: who the agent is, its prompt, the workspace tools, and
//! the `/models` and `/setup` commands it answers without calling the model.

use std::path::PathBuf;
use std::sync::Arc;

use agent_client_protocol::schema::v1::{AvailableCommand, ToolCallLocation, ToolKind};
use async_trait::async_trait;
use ed_acp::{
    AgentInfo, AuthProvider, CommandCtx, DescribeCtx, LlmEnv, Profile, PromptCtx, SessionCtx,
    ToolCtx, ToolOutcome, Toolset, WorkspaceTools, llm,
};
use serde_json::Value;

use crate::auth::OndeAuth;

pub const INFO: AgentInfo = AgentInfo {
    name: "onde-code",
    display_name: "Onde Code",
    version: env!("CARGO_PKG_VERSION"),
    env: LlmEnv {
        prefix: "ONDE_CODE",
        // Where `--setup` has always stored the key, so existing installs stay signed in.
        dir_name: "ondecode",
        default_model: llm::DEFAULT_MODEL,
    },
};

/// Onde Code as an `ed-acp` agent.
#[derive(Debug, Clone, Copy)]
pub struct OndeCode {
    shell: bool,
}

impl OndeCode {
    /// With the workspace tools, `run_command` included.
    pub fn new() -> Self {
        Self { shell: true }
    }

    /// For hosts that cannot start processes (an iOS app, a sandboxed Mac app, a snap): the
    /// workspace tools without `run_command`, and a prompt that says so.
    pub fn without_shell() -> Self {
        Self { shell: false }
    }
}

impl Default for OndeCode {
    fn default() -> Self {
        Self::new()
    }
}

/// `WorkspaceTools` minus `run_command`.
struct FileTools;

#[async_trait]
impl Toolset for FileTools {
    fn definitions(&self) -> Vec<Value> {
        WorkspaceTools
            .definitions()
            .into_iter()
            .filter(|d| d["function"]["name"] != "run_command")
            .collect()
    }

    fn handles(&self, name: &str) -> bool {
        name != "run_command" && WorkspaceTools.handles(name)
    }

    fn describe(
        &self,
        name: &str,
        args: &Value,
        ctx: &DescribeCtx<'_>,
    ) -> (String, ToolKind, Vec<ToolCallLocation>) {
        WorkspaceTools.describe(name, args, ctx)
    }

    async fn execute(
        &self,
        ctx: &ToolCtx,
        tool_call_id: &str,
        name: &str,
        args: Value,
    ) -> anyhow::Result<ToolOutcome> {
        if name == "run_command" {
            anyhow::bail!("This host cannot run commands.");
        }
        WorkspaceTools.execute(ctx, tool_call_id, name, args).await
    }
}

const NO_SHELL_NOTE: &str = "\nThis host cannot run commands: there is no run_command tool, \
    and no git or shell. When something needs a command, say which one and let the user run it.";

#[async_trait]
impl Profile for OndeCode {
    fn info(&self) -> AgentInfo {
        INFO
    }

    fn system_prompt(&self, ctx: &PromptCtx<'_>) -> String {
        let mut prompt = system_prompt(ctx.surface, ctx.cwd, ctx.roots);
        if !self.shell {
            prompt.push_str(NO_SHELL_NOTE);
        }
        prompt
    }

    fn auth(&self) -> Option<Arc<dyn AuthProvider>> {
        Some(Arc::new(OndeAuth))
    }

    fn toolsets(&self) -> Vec<Arc<dyn Toolset>> {
        if self.shell {
            vec![Arc::new(WorkspaceTools)]
        } else {
            vec![Arc::new(FileTools)]
        }
    }

    fn slash_commands(&self, _ctx: &SessionCtx<'_>) -> Vec<AvailableCommand> {
        vec![
            AvailableCommand::new("models", "List the models this agent can use"),
            AvailableCommand::new("setup", "How to configure the provider and API key"),
        ]
    }

    fn answer_slash(&self, text: &str, ctx: &CommandCtx<'_>) -> Option<String> {
        match text.split_whitespace().next()? {
            "/models" => Some(models_reply(ctx.models, ctx.model)),
            "/setup" => Some(SETUP_REPLY.to_string()),
            _ => None,
        }
    }
}

const SETUP_REPLY: &str = "Choose \"Sign in with Onde\" in your editor, or run `onde-code \
    --login` in a terminal, to connect an Onde Inference app in your browser. To paste a key or \
    use an OpenAI API compatible endpoint instead, run `onde-code --setup`. You can also set \
    `ONDE_API_KEY`, or `OPENAI_BASE_URL`, `OPENAI_API_KEY` and `OPENAI_MODEL`, in the agent's \
    environment.";

fn models_reply(models: &[String], current: &str) -> String {
    let list = models
        .iter()
        .map(|m| {
            if m == current {
                format!("- `{m}` (current)")
            } else {
                format!("- `{m}`")
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    format!("Available models:\n\n{list}\n\nSwitch with the model picker.")
}

fn system_prompt(surface: &str, cwd: &std::path::Path, roots: &[PathBuf]) -> String {
    let mut extra: Vec<&PathBuf> = roots.iter().filter(|r| **r != cwd).collect();
    extra.sort();
    extra.dedup();
    let roots_section = if extra.is_empty() {
        String::new()
    } else {
        let list = extra
            .iter()
            .map(|r| format!("- {}", r.display()))
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            "\nAdditional workspace roots the user has opened alongside the working directory:\n\
             {list}\n\
             You may read, edit and run commands in them too: pass a root's absolute path as \
             the `root` parameter of a tool (relative `path` arguments then resolve against \
             that root instead of the working directory)."
        )
    };
    format!(
        "You are onde-code, an autonomous coding agent working inside the user's editor.\n\
         Working directory: {}{roots_section}\n\
         Use the tools to inspect and modify the project: read files before editing, prefer \
         edit_file for small changes, and run commands to build or test your work. Keep \
         answers concise and use Markdown.\n\
         When creating git commits, include the trailer:\n\
         Co-Authored-By: OndeCode v{version}-{surface} <noreply@ondeinference.com>",
        cwd.display(),
        version = env!("CARGO_PKG_VERSION"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prompt_lists_extra_roots_once_and_names_the_surface() {
        let cwd = PathBuf::from("/work/app");
        let roots = vec![
            PathBuf::from("/work/lib"),
            cwd.clone(),
            PathBuf::from("/work/lib"),
        ];
        let p = system_prompt("tui", &cwd, &roots);
        assert!(p.contains("Working directory: /work/app\n"));
        assert_eq!(p.matches("- /work/lib").count(), 1);
        assert!(!p.contains("- /work/app"));
        assert!(p.ends_with(&format!(
            "OndeCode v{}-tui <noreply@ondeinference.com>",
            env!("CARGO_PKG_VERSION")
        )));
        assert!(!system_prompt("acp", &cwd, &[]).contains("Additional workspace roots"));
    }

    #[test]
    fn without_a_shell_there_is_no_run_command() {
        let names = |p: &OndeCode| -> Vec<String> {
            p.toolsets()
                .iter()
                .flat_map(|t| t.definitions())
                .map(|d| d["function"]["name"].as_str().unwrap().to_string())
                .collect()
        };
        assert!(names(&OndeCode::new()).contains(&"run_command".to_string()));
        let bare = names(&OndeCode::without_shell());
        assert!(!bare.contains(&"run_command".to_string()));
        assert!(bare.contains(&"edit_file".to_string()));
    }

    #[test]
    fn models_reply_marks_the_current_model() {
        let models = ["a".to_string(), "b".to_string()];
        assert_eq!(
            models_reply(&models, "b"),
            "Available models:\n\n- `a`\n- `b` (current)\n\nSwitch with the model picker."
        );
    }
}
