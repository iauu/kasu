use slack_morphism::{SlackChannelId, SlackUserId};
use slack_morphism::blocks::{SlackBlock, SlackBlockText, SlackRichTextElement, SlackRichTextInlineElement, SlackRichTextQuote, SlackRichTextSection, SlackRichTextStyle, SlackRichTextText, SlackSectionBlock};
use slack_morphism::prelude::{SlackRichTextBlock, SlackRichTextUser};
use sqlx::Row;
use tracing::instrument;
use crate::lib::api::{ChannelRestriction, MessageData, SendRestriction};
use crate::lib::client::PartialClient;
use crate::lib::cmd::event::CmdParsedEvent;
use crate::lib::context::State;
use crate::lib::ctx_item::{Messageable, PartialChannel, PartialUser};
use crate::lib::ctx_trait::Sendable;
use crate::lib::ws::event::{WebsocketChannelMemberJoinEvent, WebsocketCommandsChangedEvent};
use crate::state::BotState;

#[instrument(level = "info", fields(module = module_path!()), target = "slash_cmd_change")]
pub(crate) async fn slash_cmd_change(
    event: WebsocketCommandsChangedEvent,
    State::State(state): State<BotState>,
    partial_client: PartialClient
) -> ()  {
    let Some(channel_id) = &state.0.read().await.new_slack_cmd else {
        return;
    };
    let changed_message = event.commands_updated.into_iter().map(
        |entry| {

            let name = entry.name
                .replace("`", "");
            let mut e = vec![
                SlackRichTextElement::Section(
                    SlackRichTextSection::new(vec![
                        SlackRichTextInlineElement::Text(SlackRichTextText::new("Updated command ".into())),
                        SlackRichTextInlineElement::Text(SlackRichTextText::new(entry.name).with_style(SlackRichTextStyle::new().opt_code(Some(true)))),
                        SlackRichTextInlineElement::Text(SlackRichTextText::new(" by ".into())),
                        SlackRichTextInlineElement::User(SlackRichTextUser::new(entry.app_id.0.into())) // App id attempt?
                    ]
                )
            )];
            if (entry.usage.len() > 0) {
                let usage_text = entry.usage
                    .replace("<!channel", "[blocked]")
                    .replace("<!here", "[blocked]")
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .join("\n> ");
                e.push(
                    SlackRichTextElement::Section(
                        SlackRichTextSection::new(vec![
                                SlackRichTextInlineElement::Text(SlackRichTextText::new("Usage:\n".into()))
                        ])
                    )
                );
                e.push(
                    SlackRichTextElement::Quote(
                        SlackRichTextQuote::new(
                            vec![
                                SlackRichTextInlineElement::Text(SlackRichTextText::new(usage_text).into())
                            ]
                        )
                    )
                );
            }
            if entry.description.len() > 0 {
                let description_text = entry.description
                    .replace("<!channel", "[blocked]")
                    .replace("<!here", "[blocked]")
                    .split("\n")
                    .collect::<Vec<&str>>()
                    .join("\n> ");
                e.push(
                    SlackRichTextElement::Section(
                        SlackRichTextSection::new(vec![
                            SlackRichTextInlineElement::Text(SlackRichTextText::new("Description:\n".into()))
                        ])
                    )
                );
                e.push(
                    SlackRichTextElement::Quote(
                        SlackRichTextQuote::new(
                            vec![
                                SlackRichTextInlineElement::Text(SlackRichTextText::new(description_text).into())
                            ]
                        )
                    )
                );
            }
            SlackBlock::RichText(SlackRichTextBlock::new(e))
        }
    ).collect::<Vec<SlackBlock>>();
    let deleted_message = event.commands_removed.into_iter().map(
        |entry| {
            let name = entry.name
                .replace("`", "");
            SlackBlock::RichText(SlackRichTextBlock::new(vec![
                SlackRichTextElement::Section(
                    SlackRichTextSection::new(vec![
                        SlackRichTextInlineElement::Text(SlackRichTextText::new("Deleted command ".into())),
                        SlackRichTextInlineElement::Text(SlackRichTextText::new(entry.name).with_style(SlackRichTextStyle::new().opt_code(Some(true)))),
                        SlackRichTextInlineElement::Text(SlackRichTextText::new(" by ".into())),
                        SlackRichTextInlineElement::User(SlackRichTextUser::new(entry.app_id.0.into())) // App id attempt?
                    ]
                ))
            ]))
        }
    ).collect::<Vec<SlackBlock>>();
    if changed_message.len() > 0 {
        let _ = partial_client.read().await.api_client.chat_post_message(channel_id.clone(), None, MessageData::Blockkit(changed_message)).await;
    }
    if deleted_message.len() > 0 {
        let _ = partial_client.read().await.api_client.chat_post_message(channel_id.clone(), None, MessageData::Blockkit(deleted_message)).await;
    }
}
