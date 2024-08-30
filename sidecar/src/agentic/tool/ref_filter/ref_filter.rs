use async_trait::async_trait;
use futures::{stream, StreamExt};
use llm_client::{
    broker::LLMBroker,
    clients::types::{LLMClientCompletionRequest, LLMClientMessage},
};
use std::sync::Arc;

use crate::{
    agentic::{
        symbol::identifier::LLMProperties,
        tool::{errors::ToolError, input::ToolInput, output::ToolOutput, r#type::Tool},
    },
    chunking::types::OutlineNode,
};

/// Represents a request for filtering references in the codebase.
#[derive(Debug, Clone)]
pub struct ReferenceFilterRequest {
    /// The instruction or query provided by the user.
    user_instruction: String,
    /// A collection of outline nodes representing the references to be filtered.
    reference_outlines: Vec<OutlineNode>,
    anchored_symbols: Vec<(String, String, String)>,
    llm_properties: LLMProperties,
    /// The unique identifier for the root request.
    root_id: String,
}

impl ReferenceFilterRequest {
    pub fn new(
        user_instruction: String,
        reference_outlines: Vec<OutlineNode>,
        anchored_symbols: Vec<(String, String, String)>, // consider naming these
        llm_properties: LLMProperties,
        root_id: String,
    ) -> Self {
        Self {
            user_instruction,
            reference_outlines,
            llm_properties,
            anchored_symbols,
            root_id,
        }
    }

    pub fn reference_outlines(&self) -> &[OutlineNode] {
        &self.reference_outlines
    }

    pub fn user_instruction(&self) -> &str {
        &self.user_instruction
    }

    pub fn llm_properties(&self) -> &LLMProperties {
        &self.llm_properties
    }

    pub fn anchored_symbols(&self) -> &[(String, String, String)] {
        &self.anchored_symbols
    }

    pub fn root_id(&self) -> &str {
        &self.root_id
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReferenceFilterResponse {
    answer: String,
}

impl ReferenceFilterResponse {
    pub fn new(answer: &str) -> Self {
        Self {
            answer: answer.to_string(),
        }
    }

    pub fn answer(&self) -> &str {
        &self.answer
    }
}

pub struct ReferenceFilterBroker {
    llm_client: Arc<LLMBroker>,
    _fail_over_llm: LLMProperties,
}

impl ReferenceFilterBroker {
    pub fn new(llm_client: Arc<LLMBroker>, fail_over_llm: LLMProperties) -> Self {
        Self {
            llm_client,
            _fail_over_llm: fail_over_llm,
        }
    }

    pub fn later_system_message(&self) -> String {
        format!(
            r#"You are an expert software engineer. 

You will be provided with:
1. a user query
2. a selection of code
3. the references of the symbols in the selection

The selection of code may change based on the user query.

Your job is to select the references that will also need to change based on the user query changes.

Omit those that do not need to change.

<reply>
<response>
<ref>
</ref>
<ref>
</ref>
<ref>
</ref>
</response>
</reply>"#
        )
    }

    // consider variants: tiny, regular, in-depth
    pub fn system_message(&self) -> String {
        format!(
            r#"You are an expert software engineer who is pair programming with another developer.
- The developer who you are helping with has selected some code which is present in <code_selected> and they intent to change it, the request for change will be provided to you in <user_query>.
- We found a reference for the code present in <code_selected> which is given to you in <reference> section. This means that any change made to <code_selected> might also require changes to the <reference> section.
- Given the changes which will be made to <code_selected> because of the <user_query> you need to decide if we need to change the code in <reference> section.
- Try to give back your reply in a single sentence if possible and keep it very high value.
- <user_query> which CAN lead to additional changes:
- - The user might be changing the function definition
- - The user might be adding a new parameter or removing a parameter for the class
- - Changing code from sync to async
- - and many more such cases which changes the structure and the meaning of the code, as these can be breaking changes.
- You have to decide and be certain if we are going to make a change as true or false, this should be put in a section called <change_required>
- Making a change requires a lot of effort, so be very certain if we should change the code in our selection in <code_selected> based on the <user_query>
- In your reply do not mention the <reference> as reference code, but instead talk about the code symbol.
- Your reason which you will put in the <reason> section of your reply, MUST contain the "WHY" for the change. We MUST explain to the user why the code in <reference> might require a change.

Your response must be in the following format:

<reply>
<reason>
your single sentence
</reason>
<change_required>
</change_required>
</reply>"#
        )
    }

    pub fn user_message(&self, request: &ReferenceFilterRequest) -> Vec<String> {
        let references = request.reference_outlines();
        let user_query = request.user_instruction();
        let anchored_symbols = request.anchored_symbols();

        let anchored_symbol_prompt = anchored_symbols
            .iter()
            .map(|(symbol_name, fs_file_path, contents)| {
                format!("{} in {}:\n{}", symbol_name, fs_file_path, contents)
            })
            .collect::<Vec<_>>()
            .join("\n");
        references
            .into_iter()
            .map(|reference| {
                format!(
                    r#"<user_query>
{}
</user_query>

<code_selected>
{}
</code_selected>

<reference>
{}
</reference>"#,
                    user_query,
                    anchored_symbol_prompt,
                    {
                        let name = reference.name();
                        let fs_file_path = reference.fs_file_path();
                        let content = reference.content().content();

                        format!("{} in {}\n{}", name, fs_file_path, content)
                    }
                )
            })
            .collect()
    }

    pub fn parse_response(response: &str) -> String {
        println!("parse_response::response: {}", response);
        let answer = response
            .lines()
            .skip_while(|l| !l.contains("<reply>"))
            .skip(1)
            .take_while(|l| !l.contains("</reply>"))
            .collect::<Vec<&str>>()
            .join("\n");

        answer
    }
}

#[async_trait]
impl Tool for ReferenceFilterBroker {
    async fn invoke(&self, input: ToolInput) -> Result<ToolOutput, ToolError> {
        let context = input.filter_references_request()?;
        let llm_properties = context.llm_properties.clone();
        let root_request_id = context.root_id.to_owned();

        let system_message = LLMClientMessage::system(self.system_message());
        let user_messages = self.user_message(&context);

        let _ = stream::iter(user_messages.into_iter().map(|user_message| {
            (
                LLMClientCompletionRequest::new(
                    llm_properties.llm().clone(),
                    vec![system_message.clone(), LLMClientMessage::user(user_message)],
                    0.2,
                    None,
                ),
                self.llm_client.clone(),
                llm_properties.clone(),
                root_request_id.to_owned(),
            )
        }))
        .map(
            |(request, llm_client, llm_properties, root_request_id)| async move {
                let (sender, _receiver) = tokio::sync::mpsc::unbounded_channel();
                let response = llm_client
                    .stream_completion(
                        llm_properties.api_key().clone(),
                        request,
                        llm_properties.provider().clone(),
                        vec![
                            ("event_type".to_owned(), "filter_references".to_owned()),
                            ("root_id".to_owned(), root_request_id.to_owned()),
                        ]
                        .into_iter()
                        .collect(),
                        sender,
                    )
                    .await;
                println!("reference_check::response::({:?})", response);
            },
        )
        .buffer_unordered(200)
        .collect::<Vec<_>>()
        .await;

        Err(ToolError::MissingTool)

        // // this may need to become more sophisticated later, but we roll for now
        // let answer = ReferenceFilterBroker::parse_response(&response);

        // println!("answer: {}", &answer);

        // Ok(ToolOutput::ReferencesFilter(ReferenceFilterResponse::new(
        //     &answer,
        // )))
    }
}