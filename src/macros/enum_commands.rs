#[macro_export]
macro_rules! enum_commands {
    ($module_name:ident, ($($module:ident),*)) => (
        paste::paste! {
            // use crate::{CommandContext as CC};

            #[derive(Debug, Serialize, Subcommand)]
            enum Commands {
                $(
                    [<$module:camel>]($module::[<$module:camel>]),
                )*
            }

            impl Executor for $module_name {
                async fn execute(&self, /* context: CC */) -> SgResult<()> {
                    match &self.command {
                        $(
                          Commands::[<$module:camel>](command) => command.execute().await?,
                        )*
                    }
                    Ok(())
                }
            }
        }
    );
}
