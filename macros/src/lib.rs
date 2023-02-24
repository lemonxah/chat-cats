use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse_macro_input, DeriveInput};

#[proc_macro_derive(ChatCommand)]
pub fn derive_chat_command(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let gen = quote! {
        #[async_trait]
        impl ChatCommand for #name {
            fn matches(&self, message: &str) -> bool {
                self.matches.iter().any(|m| message.starts_with(*m))
            }
            async fn handle(&self, message: &Message, discord: &Discord, config: &Config, db: Database) -> Result<Message, CommandError> {
                self.respond(message, discord, db).await
            }
        }
    };
    gen.into()
}