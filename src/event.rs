//! Semantic labels of things happening.

/// Semantic label of a span.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Name {
    /// Attention sequence.
    ///
    /// > 👉 **Note**: this is used while parsing but compiled away.
    AttentionSequence,
    /// Whole autolink.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`AutolinkEmail`][Name::AutolinkEmail],
    ///     [`AutolinkMarker`][Name::AutolinkMarker],
    ///     [`AutolinkProtocol`][Name::AutolinkProtocol]
    /// *   **Construct**:
    ///     [`autolink`][crate::construct::autolink]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <https://example.com> and <admin@example.com>
    ///     ^^^^^^^^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^
    /// ```
    Autolink,
    /// Email autolink w/o markers.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Autolink`][Name::Autolink]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`autolink`][crate::construct::autolink]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <admin@example.com>
    ///      ^^^^^^^^^^^^^^^^^
    /// ```
    AutolinkEmail,
    /// Marker of an autolink.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Autolink`][Name::Autolink]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`autolink`][crate::construct::autolink]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <https://example.com>
    ///     ^                   ^
    /// ```
    AutolinkMarker,
    /// Protocol autolink w/o markers.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Autolink`][Name::Autolink]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`autolink`][crate::construct::autolink]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <https://example.com>
    ///      ^^^^^^^^^^^^^^^^^^^
    /// ```
    AutolinkProtocol,
    /// Line ending preceded only by whitespace or nothing at all.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`blank_line`][crate::construct::blank_line]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ␠␠␊
    ///       ^
    /// ```
    BlankLineEnding,
    /// Whole block quote.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [document content][crate::construct::document]
    /// *   **Content model**:
    ///     [`BlockQuotePrefix`][Name::BlockQuotePrefix],
    ///     [flow content][crate::construct::flow]
    /// *   **Construct**:
    ///     [`block_quote`][crate::construct::block_quote]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | > a
    ///     ^^^
    /// > | b
    ///     ^
    /// ```
    BlockQuote,
    /// Block quote marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`BlockQuotePrefix`][Name::BlockQuotePrefix]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`block_quote`][crate::construct::block_quote]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | > a
    ///     ^
    ///   | b
    /// ```
    BlockQuoteMarker,
    /// Block quote prefix.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`BlockQuote`][Name::BlockQuote]
    /// *   **Content model**:
    ///     [`BlockQuoteMarker`][Name::BlockQuoteMarker],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`block_quote`][crate::construct::block_quote]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | > a
    ///     ^^
    ///   | b
    /// ```
    BlockQuotePrefix,
    /// Byte order mark.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     optional first event
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`document`][crate::construct::document]
    ByteOrderMark,
    /// Whole character escape.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [string content][crate::construct::string] or
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`CharacterEscapeMarker`][Name::CharacterEscapeMarker],
    ///     [`CharacterEscapeValue`][Name::CharacterEscapeValue]
    /// *   **Construct**:
    ///     [`character_escape`][crate::construct::character_escape]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a \- b
    ///       ^^
    /// ```
    CharacterEscape,
    /// Character escape marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterEscape`][Name::CharacterEscape]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_escape`][crate::construct::character_escape]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a \- b
    ///       ^
    /// ```
    CharacterEscapeMarker,
    /// Character escape value.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterEscape`][Name::CharacterEscape]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_escape`][crate::construct::character_escape]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a \- b
    ///        ^
    /// ```
    CharacterEscapeValue,
    /// Whole character reference.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [string content][crate::construct::string] or
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`CharacterReferenceMarker`][Name::CharacterReferenceMarker],
    ///     [`CharacterReferenceMarkerHexadecimal`][Name::CharacterReferenceMarkerHexadecimal],
    ///     [`CharacterReferenceMarkerNumeric`][Name::CharacterReferenceMarkerNumeric],
    ///     [`CharacterReferenceMarkerSemi`][Name::CharacterReferenceMarkerSemi],
    ///     [`CharacterReferenceValue`][Name::CharacterReferenceValue]
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///       ^^^^^   ^^^^^^^   ^^^^^^^^^
    /// ```
    CharacterReference,
    /// Character reference opening marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterReference`][Name::CharacterReference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///       ^       ^         ^
    /// ```
    CharacterReferenceMarker,
    /// Character reference hexadecimal numeric marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterReference`][Name::CharacterReference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///                           ^
    /// ```
    CharacterReferenceMarkerHexadecimal,
    /// Character reference numeric marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterReference`][Name::CharacterReference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///                ^         ^
    /// ```
    CharacterReferenceMarkerNumeric,
    /// Character reference closing marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterReference`][Name::CharacterReference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///           ^         ^           ^
    /// ```
    CharacterReferenceMarkerSemi,
    /// Character reference value.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CharacterReference`][Name::CharacterReference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`character_reference`][crate::construct::character_reference]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a &amp; b &#8800; c &#x1D306; d
    ///        ^^^      ^^^^       ^^^^^
    /// ```
    CharacterReferenceValue,
    /// Whole code (fenced).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`CodeFencedFence`][Name::CodeFencedFence],
    ///     [`CodeFlowChunk`][Name::CodeFlowChunk],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ````markdown
    /// > | ```js
    ///     ^^^^^
    /// > | console.log(1)
    ///     ^^^^^^^^^^^^^^
    /// > | ```
    ///     ^^^
    /// ````
    CodeFenced,
    /// A code (fenced) fence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeFenced`][Name::CodeFenced]
    /// *   **Content model**:
    ///     [`CodeFencedFenceInfo`][Name::CodeFencedFenceInfo],
    ///     [`CodeFencedFenceMeta`][Name::CodeFencedFenceMeta],
    ///     [`CodeFencedFenceSequence`][Name::CodeFencedFenceSequence],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ````markdown
    /// > | ```js
    ///     ^^^^^
    ///   | console.log(1)
    /// > | ```
    ///     ^^^
    /// ````
    CodeFencedFence,
    /// A code (fenced) fence info word.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeFencedFence`][Name::CodeFencedFence]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ````markdown
    /// > | ```js
    ///        ^^
    ///   | console.log(1)
    ///   | ```
    /// ````
    CodeFencedFenceInfo,
    /// A code (fenced) fence meta string.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeFencedFence`][Name::CodeFencedFence]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ````markdown
    /// > | ```js highlight="1"
    ///           ^^^^^^^^^^^^^
    ///   | console.log(1)
    ///   | ```
    /// ````
    CodeFencedFenceMeta,
    /// A code (fenced) fence sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeFencedFenceSequence`][Name::CodeFencedFenceSequence]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ````markdown
    /// > | ```js
    ///     ^^^
    ///   | console.log(1)
    /// > | ```
    ///     ^^^
    /// ````
    CodeFencedFenceSequence,
    /// A code (fenced, indented) chunk.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeFenced`][Name::CodeFenced],
    ///     [`CodeIndented`][Name::CodeIndented]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced],
    ///     [`code_indented`][crate::construct::code_indented]
    ///
    /// ## Example
    ///
    /// ````markdown
    ///   | ```js
    /// > | console.log(1)
    ///     ^^^^^^^^^^^^^^
    ///   | ```
    /// ````
    ///
    /// ```markdown
    /// > | ␠␠␠␠console.log(1)
    ///         ^^^^^^^^^^^^^^
    /// ```
    CodeFlowChunk,
    /// Whole code (indented).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`CodeFlowChunk`][Name::CodeFlowChunk],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`code_fenced`][crate::construct::code_fenced]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// ␠␠␠␠console.log(1)
    /// ^^^^^^^^^^^^^^^^^^
    /// ```
    CodeIndented,
    /// Whole code (text).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`CodeTextData`][Name::CodeTextData],
    ///     [`CodeTextSequence`][Name::CodeTextSequence],
    ///     [`LineEnding`][Name::LineEnding]
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a `b` c
    ///       ^^^
    /// ```
    CodeText,
    /// Code (text) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeText`][Name::CodeText],
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a `b` c
    ///        ^
    /// ```
    CodeTextData,
    /// Code (text) sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`CodeText`][Name::CodeText],
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a `b` c
    ///       ^ ^
    /// ```
    CodeTextSequence,
    /// Data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [string content][crate::construct::string],
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`data`][crate::construct::partial_data]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | aa *bb* cc
    ///     ^^^ ^^ ^^^
    /// ```
    Data,
    /// Whole definition.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`DefinitionMarker`][Name::DefinitionMarker],
    ///     [`DefinitionLabel`][Name::DefinitionLabel],
    ///     [`DefinitionDestination`][Name::DefinitionDestination],
    ///     [`DefinitionTitle`][Name::DefinitionTitle],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`definition`][crate::construct::definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///     ^^^^^^^^^^
    /// ```
    Definition,
    /// Whole definition destination.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Definition`][Name::Definition]
    /// *   **Content model**:
    ///     [`DefinitionDestinationLiteral`][Name::DefinitionDestinationLiteral],
    ///     [`DefinitionDestinationRaw`][Name::DefinitionDestinationRaw]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///          ^
    /// > | [a]: <b> "c"
    ///          ^^^
    /// ```
    DefinitionDestination,
    /// Definition destination literal.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionDestination`][Name::DefinitionDestination]
    /// *   **Content model**:
    ///     [`DefinitionDestinationLiteralMarker`][Name::DefinitionDestinationLiteralMarker],
    ///     [`DefinitionDestinationString`][Name::DefinitionDestinationString]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: <b> "c"
    ///          ^^^
    /// ```
    DefinitionDestinationLiteral,
    /// Definition destination literal marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionDestinationLiteral`][Name::DefinitionDestinationLiteral]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: <b> "c"
    ///          ^ ^
    /// ```
    DefinitionDestinationLiteralMarker,
    /// Definition destination raw.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionDestination`][Name::DefinitionDestination]
    /// *   **Content model**:
    ///     [`DefinitionDestinationString`][Name::DefinitionDestinationString]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///          ^
    /// ```
    DefinitionDestinationRaw,
    /// Definition destination data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionDestinationLiteral`][Name::DefinitionDestinationLiteral],
    ///     [`DefinitionDestinationRaw`][Name::DefinitionDestinationRaw]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///          ^
    /// > | [a]: <b> "c"
    ///           ^
    /// ```
    DefinitionDestinationString,
    /// Whole definition label.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Definition`][Name::Definition]
    /// *   **Content model**:
    ///     [`DefinitionLabelMarker`][Name::DefinitionLabelMarker],
    ///     [`DefinitionLabelString`][Name::DefinitionLabelString],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///     ^^^
    /// ```
    DefinitionLabel,
    /// Definition label marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionLabel`][Name::DefinitionLabel]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///     ^ ^
    /// ```
    DefinitionLabelMarker,
    /// Definition label data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionLabel`][Name::DefinitionLabel]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///      ^
    /// ```
    DefinitionLabelString,
    /// Definition marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Definition`][Name::Definition],
    ///     [`GfmFootnoteDefinition`][Name::GfmFootnoteDefinition]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`definition`][crate::construct::definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///        ^
    /// ```
    DefinitionMarker,
    /// Whole definition title.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Definition`][Name::Definition]
    /// *   **Content model**:
    ///     [`DefinitionTitleMarker`][Name::DefinitionTitleMarker],
    ///     [`DefinitionTitleString`][Name::DefinitionTitleString],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///            ^^^
    /// ```
    DefinitionTitle,
    /// Definition title marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionTitle`][Name::DefinitionTitle]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///            ^ ^
    /// ```
    DefinitionTitleMarker,
    /// Definition title data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`DefinitionTitle`][Name::DefinitionTitle]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [a]: b "c"
    ///             ^
    /// ```
    DefinitionTitleString,
    /// Emphasis.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`EmphasisSequence`][Name::EmphasisSequence],
    ///     [`EmphasisText`][Name::EmphasisText]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | *a*
    ///     ^^^
    /// ```
    Emphasis,
    /// Emphasis sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Emphasis`][Name::Emphasis]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | *a*
    ///     ^ ^
    /// ```
    EmphasisSequence,
    /// Emphasis text.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Emphasis`][Name::Emphasis]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | *a*
    ///      ^
    /// ```
    EmphasisText,
    /// Whole frontmatter.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [document content][crate::construct::document]
    /// *   **Content model**:
    ///     [`FrontmatterFence`][Name::FrontmatterFence],
    ///     [`FrontmatterChunk`][Name::FrontmatterChunk],
    ///     [`LineEnding`][Name::LineEnding]
    /// *   **Construct**:
    ///     [`frontmatter`][crate::construct::frontmatter]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ---
    ///     ^^^
    /// > | title: Neptune
    ///     ^^^^^^^^^^^^^^
    /// > | ---
    ///     ^^^
    /// ```
    Frontmatter,
    /// Frontmatter chunk.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Frontmatter`][Name::Frontmatter]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`frontmatter`][crate::construct::frontmatter]
    ///
    /// ## Example
    ///
    /// ```markdown
    ///   | ---
    /// > | title: Neptune
    ///     ^^^^^^^^^^^^^^
    ///   | ---
    /// ```
    FrontmatterChunk,
    /// Frontmatter fence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Frontmatter`][Name::Frontmatter]
    /// *   **Content model**:
    ///     [`FrontmatterSequence`][Name::FrontmatterSequence],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`frontmatter`][crate::construct::frontmatter]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ---
    ///     ^^^
    ///   | title: Neptune
    /// > | ---
    ///     ^^^
    /// ```
    FrontmatterFence,
    /// Frontmatter sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`FrontmatterFence`][Name::FrontmatterFence]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`frontmatter`][crate::construct::frontmatter]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ---
    ///     ^^^
    ///   | title: Neptune
    /// > | ---
    ///     ^^^
    /// ```
    FrontmatterSequence,
    /// GFM extension: email autolink.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void.
    /// *   **Construct**:
    ///     [`gfm_autolink_literal`][crate::construct::gfm_autolink_literal]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | context@example.com
    ///     ^^^^^^^^^^^^^^^^^^^
    /// ```
    GfmAutolinkLiteralEmail,
    /// GFM extension: autolink w/ protocol.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void.
    /// *   **Construct**:
    ///     [`gfm_autolink_literal`][crate::construct::gfm_autolink_literal]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | https://example.com
    ///     ^^^^^^^^^^^^^^^^^^^
    /// ```
    GfmAutolinkLiteralProtocol,
    /// GFM extension: autolink w/ www.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void.
    /// *   **Construct**:
    ///     [`gfm_autolink_literal`][crate::construct::gfm_autolink_literal]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | www.example.com
    ///     ^^^^^^^^^^^^^^^
    /// ```
    GfmAutolinkLiteralWww,
    /// GFM extension: whole footnote call.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`Label`][Name::Label]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [^b] c
    ///       ^^^^
    /// ```
    GfmFootnoteCall,
    /// GFM extension: label start (footnote).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Label`][Name::Label]
    /// *   **Content model**:
    ///     [`GfmFootnoteCallMarker`][Name::GfmFootnoteCallMarker],
    ///     [`LabelMarker`][Name::LabelMarker]
    /// *   **Construct**:
    ///     [`gfm_label_start_footnote`][crate::construct::gfm_label_start_footnote]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [^b] c
    ///       ^^
    /// ```
    GfmFootnoteCallLabel,
    /// GFM extension: label start (footnote) marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteCallLabel`][Name::GfmFootnoteCallLabel]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_label_start_footnote`][crate::construct::gfm_label_start_footnote]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [^b] c
    ///        ^
    /// ```
    GfmFootnoteCallMarker,
    /// GFM extension: whole footnote definition.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [document content][crate::construct::document]
    /// *   **Content model**:
    ///     [`GfmFootnoteDefinitionPrefix`][Name::GfmFootnoteDefinitionPrefix],
    ///     [document content][crate::construct::flow]
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///     ^^^^^^^
    /// ```
    GfmFootnoteDefinition,
    /// GFM extension: footnote definition prefix.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteDefinition`][Name::GfmFootnoteDefinition]
    /// *   **Content model**:
    ///     [`DefinitionMarker`][Name::DefinitionMarker],
    ///     [`GfmFootnoteDefinitionLabel`][Name::GfmFootnoteDefinitionLabel],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///     ^^^^^^
    /// ```
    GfmFootnoteDefinitionPrefix,
    /// GFM extension: footnote definition label.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteDefinitionPrefix`][Name::GfmFootnoteDefinitionPrefix]
    /// *   **Content model**:
    ///     [`GfmFootnoteDefinitionLabelMarker`][Name::GfmFootnoteDefinitionLabelMarker],
    ///     [`GfmFootnoteDefinitionLabelString`][Name::GfmFootnoteDefinitionLabelString],
    ///     [`GfmFootnoteDefinitionMarker`][Name::GfmFootnoteDefinitionMarker]
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///     ^^^^
    /// ```
    GfmFootnoteDefinitionLabel,
    /// GFM extension: footnote definition label marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteDefinitionLabel`][Name::GfmFootnoteDefinitionLabel]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///     ^  ^
    GfmFootnoteDefinitionLabelMarker,
    /// GFM extension: footnote definition label string.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteDefinitionLabel`][Name::GfmFootnoteDefinitionLabel]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///       ^
    GfmFootnoteDefinitionLabelString,
    /// GFM extension: footnote definition marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmFootnoteDefinitionLabel`][Name::GfmFootnoteDefinitionLabel]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_footnote_definition`][crate::construct::gfm_footnote_definition]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | [^a]: b
    ///      ^
    GfmFootnoteDefinitionMarker,
    /// GFM extension: Strikethrough.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`GfmStrikethroughSequence`][Name::GfmStrikethroughSequence],
    ///     [`GfmStrikethroughText`][Name::GfmStrikethroughText]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ~a~
    ///     ^^^
    /// ```
    GfmStrikethrough,
    /// GFM extension: Strikethrough sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmStrikethrough`][Name::GfmStrikethrough]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ~a~
    ///     ^ ^
    /// ```
    GfmStrikethroughSequence,
    /// GFM extension: Strikethrough text.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmStrikethrough`][Name::GfmStrikethrough]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ~a~
    ///      ^
    /// ```
    GfmStrikethroughText,
    /// GFM extension: task list item check.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`GfmTaskListItemMarker`][Name::GfmTaskListItemMarker],
    ///     [`GfmTaskListItemValueChecked`][Name::GfmTaskListItemValueChecked],
    ///     [`GfmTaskListItemValueUnchecked`][Name::GfmTaskListItemValueUnchecked]
    /// *   **Construct**:
    ///     [`gfm_task_list_item_check`][crate::construct::gfm_task_list_item_check]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * [x] y.
    ///       ^^^
    /// ```
    GfmTaskListItemCheck,
    /// GFM extension: task list item check marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmTaskListItemCheck`][Name::GfmTaskListItemCheck]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_task_list_item_check`][crate::construct::gfm_task_list_item_check]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * [x] y.
    ///       ^ ^
    /// ```
    GfmTaskListItemMarker,
    /// GFM extension: task list item value: checked.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmTaskListItemCheck`][Name::GfmTaskListItemCheck]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_task_list_item_check`][crate::construct::gfm_task_list_item_check]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * [x] y.
    ///        ^
    /// ```
    GfmTaskListItemValueChecked,
    /// GFM extension: task list item value: unchecked.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`GfmTaskListItemCheck`][Name::GfmTaskListItemCheck]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`gfm_task_list_item_check`][crate::construct::gfm_task_list_item_check]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * [ ] z.
    ///        ^
    /// ```
    GfmTaskListItemValueUnchecked,
    /// Whole hard break (escape).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`hard_break_escape`][crate::construct::hard_break_escape]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a\␊
    ///      ^
    /// > | b
    /// ```
    HardBreakEscape,
    /// Whole hard break (trailing).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`whitespace`][crate::construct::partial_whitespace]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a␠␠␊
    ///      ^^
    /// > | b
    /// ```
    HardBreakTrailing,
    /// Whole heading (atx).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`HeadingAtxSequence`][Name::HeadingAtxSequence],
    ///     [`HeadingAtxText`][Name::HeadingAtxText],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`heading_atx`][crate::construct::heading_atx]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | # alpha
    ///     ^^^^^^^
    /// ```
    HeadingAtx,
    /// Heading (atx) sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HeadingAtx`][Name::HeadingAtx]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`heading_atx`][crate::construct::heading_atx]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | # alpha
    ///     ^
    /// ```
    HeadingAtxSequence,
    /// Heading (atx) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HeadingAtx`][Name::HeadingAtx],
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`heading_atx`][crate::construct::heading_atx]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | # alpha
    ///       ^^^^^
    /// ```
    HeadingAtxText,
    /// Whole heading (setext).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`HeadingSetextText`][Name::HeadingSetextText],
    ///     [`HeadingSetextUnderline`][Name::HeadingSetextUnderline],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`heading_setext`][crate::construct::heading_setext]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | alpha
    ///     ^^^^^
    /// > | =====
    ///     ^^^^^
    /// ```
    HeadingSetext,
    /// Heading (setext) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HeadingSetext`][Name::HeadingSetext]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`heading_setext`][crate::construct::heading_setext]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | alpha
    ///     ^^^^^
    ///   | =====
    /// ```
    HeadingSetextText,
    /// Heading (setext) underline.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HeadingSetext`][Name::HeadingSetext]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`heading_setext`][crate::construct::heading_setext]
    ///
    /// ## Example
    ///
    /// ```markdown
    ///   | alpha
    /// > | =====
    ///     ^^^^^
    /// ```
    HeadingSetextUnderline,
    /// Whole html (flow).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`HtmlFlowData`][Name::HtmlFlowData],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`html_flow`][crate::construct::html_flow]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <div>
    ///     ^^^^^
    /// ```
    HtmlFlow,
    /// HTML (flow) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HtmlFlow`][Name::HtmlFlow],
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`html_flow`][crate::construct::html_flow]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | <div>
    ///     ^^^^^
    /// ```
    HtmlFlowData,
    /// Whole html (text).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`HtmlTextData`][Name::HtmlTextData],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`html_text`][crate::construct::html_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a <b> c
    ///       ^^^
    /// ```
    HtmlText,
    /// HTML (text) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`HtmlText`][Name::HtmlText]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`html_text`][crate::construct::html_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a <b> c
    ///       ^^^
    /// ```
    HtmlTextData,
    /// Image.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`Label`][Name::Label],
    ///     [`Resource`][Name::Resource],
    ///     [`Reference`][Name::Reference]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b] c
    ///       ^^^^
    /// > | a ![b][c] d
    ///       ^^^^^^^
    /// > | a ![b](c) d
    ///       ^^^^^^^
    /// ```
    Image,
    /// Label.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Image`][Name::Image],
    ///     [`Link`][Name::Link]
    /// *   **Content model**:
    ///     [`LabelImage`][Name::LabelImage],
    ///     [`LabelLink`][Name::LabelLink],
    ///     [`LabelEnd`][Name::LabelEnd],
    ///     [`LabelText`][Name::LabelText]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [b] c
    ///       ^^^
    /// > | a ![b][c] d
    ///       ^^^^
    /// > | a [b](c) d
    ///       ^^^
    /// ```
    Label,
    /// Label end.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Label`][Name::Label]
    /// *   **Content model**:
    ///     [`LabelMarker`][Name::LabelMarker]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c) d
    ///          ^
    /// > | a [b](c) d
    ///         ^
    /// ```
    LabelEnd,
    /// Label start (image).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Label`][Name::Label]
    /// *   **Content model**:
    ///     [`LabelImageMarker`][Name::LabelImageMarker],
    ///     [`LabelMarker`][Name::LabelMarker]
    /// *   **Construct**:
    ///     [`label_start_image`][crate::construct::label_start_image]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c) d
    ///       ^^
    /// ```
    LabelImage,
    /// Label start (image) marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`LabelImage`][Name::LabelImage]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`label_start_image`][crate::construct::label_start_image]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c) d
    ///       ^
    /// ```
    LabelImageMarker,
    /// Label start (link).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Label`][Name::Label]
    /// *   **Content model**:
    ///     [`LabelMarker`][Name::LabelMarker]
    /// *   **Construct**:
    ///     [`label_start_link`][crate::construct::label_start_link]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [b](c) d
    ///       ^
    /// ```
    LabelLink,
    /// Label marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`LabelImage`][Name::LabelImage],
    ///     [`LabelLink`][Name::LabelLink],
    ///     [`LabelEnd`][Name::LabelEnd]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`label_start_image`][crate::construct::label_start_image],
    ///     [`label_start_link`][crate::construct::label_start_link],
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c) d
    ///        ^ ^
    /// > | a [b](c) d
    ///       ^ ^
    /// ```
    LabelMarker,
    /// Label text.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Label`][Name::Label]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [b] c
    ///        ^
    /// > | a ![b][c] d
    ///         ^
    /// > | a [b](c) d
    ///        ^
    /// ```
    LabelText,
    /// Line ending.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     basically everywhere
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     n/a
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a␊
    ///      ^
    ///   | b
    /// ```
    LineEnding,
    /// Link.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`Label`][Name::Label],
    ///     [`Resource`][Name::Resource],
    ///     [`Reference`][Name::Reference]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a [b] c
    ///       ^^^
    /// > | a [b][c] d
    ///       ^^^^^^
    /// > | a [b](c) d
    ///       ^^^^^^
    /// ```
    Link,
    /// List item.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ListOrdered`][Name::ListOrdered],
    ///     [`ListUnordered`][Name::ListUnordered],
    /// *   **Content model**:
    ///     [`ListItemPrefix`][Name::ListItemPrefix],
    ///     [flow content][crate::construct::flow]
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * a
    ///     ^^^
    /// > | 1. b
    ///     ^^^^
    /// ```
    ListItem,
    /// List item (marker).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ListItemPrefix`][Name::ListItemPrefix]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * a
    ///     ^
    /// > | 1. b
    ///      ^
    /// ```
    ListItemMarker,
    /// List item (prefix).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ListItem`][Name::ListItem]
    /// *   **Content model**:
    ///     [`ListItemMarker`][Name::ListItemMarker],
    ///     [`ListItemValue`][Name::ListItemValue],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * a
    ///     ^^
    /// > |   b
    ///     ^^
    /// ```
    ListItemPrefix,
    /// List item (value).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ListItemPrefix`][Name::ListItemPrefix]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | 1. b
    ///     ^
    /// ```
    ListItemValue,
    /// List (ordered).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [document content][crate::construct::document]
    /// *   **Content model**:
    ///     [`BlankLineEnding`][Name::BlankLineEnding],
    ///     [`BlockQuotePrefix`][Name::BlockQuotePrefix],
    ///     [`ListItem`][Name::ListItem],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | 1. a
    ///     ^^^^
    /// > | 2. b
    ///     ^^^^
    /// ```
    ListOrdered,
    /// List (unordered).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [document content][crate::construct::document]
    /// *   **Content model**:
    ///     [`BlankLineEnding`][Name::BlankLineEnding],
    ///     [`BlockQuotePrefix`][Name::BlockQuotePrefix],
    ///     [`ListItem`][Name::ListItem],
    ///     [`LineEnding`][Name::LineEnding],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`list item`][crate::construct::list_item]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * a
    ///     ^^^
    /// > | * b
    ///     ^^^
    /// ```
    ListUnordered,
    /// Whole math (text).
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`MathTextData`][Name::MathTextData],
    ///     [`MathTextSequence`][Name::MathTextSequence],
    ///     [`LineEnding`][Name::LineEnding]
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a $b$ c
    ///       ^^^
    /// ```
    MathText,
    /// Math (text) data.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`MathText`][Name::MathText],
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a `b` c
    ///        ^
    /// ```
    MathTextData,
    /// Math (text) sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`MathText`][Name::MathText],
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`raw_text`][crate::construct::raw_text]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a $b$ c
    ///       ^ ^
    /// ```
    MathTextSequence,
    /// Whole paragraph.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`paragraph`][crate::construct::paragraph]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a b
    ///     ^^^
    /// > | c.
    ///     ^^
    /// ```
    Paragraph,
    /// Reference.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Image`][Name::Image],
    ///     [`Link`][Name::Link]
    /// *   **Content model**:
    ///     [`ReferenceMarker`][Name::ReferenceMarker],
    ///     [`ReferenceString`][Name::ReferenceString]
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b][c] d
    ///           ^^^
    /// ```
    Reference,
    /// Reference marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Reference`][Name::Reference]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b][c] d
    ///           ^ ^
    /// ```
    ReferenceMarker,
    /// Reference string.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Reference`][Name::Reference]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`label`][crate::construct::partial_label]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b][c] d
    ///            ^
    /// ```
    ReferenceString,
    /// Resource.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Image`][Name::Image],
    ///     [`Link`][Name::Link]
    /// *   **Content model**:
    ///     [`ResourceMarker`][Name::ResourceMarker],
    ///     [`ResourceDestination`][Name::ResourceDestination],
    ///     [`ResourceTitle`][Name::ResourceTitle],
    ///     [`SpaceOrTab`][Name::SpaceOrTab],
    ///     [`LineEnding`][Name::LineEnding]
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c "d") e
    ///           ^^^^^^^
    /// > | a [b](c) d
    ///          ^^^
    /// ```
    Resource,
    /// Resource destination.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Resource`][Name::Resource]
    /// *   **Content model**:
    ///     [`ResourceDestinationLiteral`][Name::ResourceDestinationLiteral],
    ///     [`ResourceDestinationRaw`][Name::ResourceDestinationRaw]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c "d") e
    ///            ^
    /// ```
    ResourceDestination,
    /// Resource destination literal.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceDestination`][Name::ResourceDestination]
    /// *   **Content model**:
    ///     [`ResourceDestinationLiteralMarker`][Name::ResourceDestinationLiteralMarker],
    ///     [`ResourceDestinationString`][Name::ResourceDestinationString]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///            ^^^
    /// ```
    ResourceDestinationLiteral,
    /// Resource destination literal marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceDestinationLiteral`][Name::ResourceDestinationLiteral]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///            ^ ^
    /// ```
    ResourceDestinationLiteralMarker,
    /// Resource destination raw.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceDestination`][Name::ResourceDestination]
    /// *   **Content model**:
    ///     [`ResourceDestinationString`][Name::ResourceDestinationString]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c "d") e
    ///            ^
    /// ```
    ResourceDestinationRaw,
    /// Resource destination raw.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceDestinationLiteral`][Name::ResourceDestinationLiteral],
    ///     [`ResourceDestinationRaw`][Name::ResourceDestinationRaw]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`destination`][crate::construct::partial_destination]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///             ^
    /// > | a ![b](c "d") e
    ///            ^
    /// ```
    ResourceDestinationString,
    /// Resource marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Resource`][Name::Resource]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`label_end`][crate::construct::label_end]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](c "d") e
    ///           ^     ^
    /// ```
    ResourceMarker,
    /// Resource title.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Resource`][Name::Resource]
    /// *   **Content model**:
    ///     [`ResourceTitleMarker`][Name::ResourceTitleMarker],
    ///     [`ResourceTitleString`][Name::ResourceTitleString]
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///                ^^^
    /// ```
    ResourceTitle,
    /// Resource title marker.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceTitle`][Name::ResourceTitle]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///                ^ ^
    /// ```
    ResourceTitleMarker,
    /// Resource title string.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ResourceTitle`][Name::ResourceTitle]
    /// *   **Content model**:
    ///     [string content][crate::construct::string]
    /// *   **Construct**:
    ///     [`title`][crate::construct::partial_title]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | a ![b](<c> "d") e
    ///                 ^
    /// ```
    ResourceTitleString,
    /// SpaceOrTab.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     basically everywhere
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     n/a
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | ␠* * *␠
    ///     ^ ^ ^ ^
    /// ```
    SpaceOrTab,
    /// Strong.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [text content][crate::construct::text]
    /// *   **Content model**:
    ///     [`StrongSequence`][Name::StrongSequence],
    ///     [`StrongText`][Name::StrongText]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a**
    ///     ^^^^^
    /// ```
    Strong,
    /// Strong sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Strong`][Name::Strong]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a**
    ///     ^^ ^^
    /// ```
    StrongSequence,
    /// Strong text.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`Strong`][Name::Strong]
    /// *   **Content model**:
    ///     [text content][crate::construct::text]
    /// *   **Construct**:
    ///     [`attention`][crate::construct::attention]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | **a**
    ///       ^
    /// ```
    StrongText,
    /// Whole thematic break.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [flow content][crate::construct::flow]
    /// *   **Content model**:
    ///     [`ThematicBreakSequence`][Name::ThematicBreakSequence],
    ///     [`SpaceOrTab`][Name::SpaceOrTab]
    /// *   **Construct**:
    ///     [`thematic_break`][crate::construct::thematic_break]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * * *
    ///     ^^^^^
    /// ```
    ThematicBreak,
    /// Thematic break sequence.
    ///
    /// ## Info
    ///
    /// *   **Context**:
    ///     [`ThematicBreak`][Name::ThematicBreak]
    /// *   **Content model**:
    ///     void
    /// *   **Construct**:
    ///     [`thematic_break`][crate::construct::thematic_break]
    ///
    /// ## Example
    ///
    /// ```markdown
    /// > | * * *
    ///     ^ ^ ^
    /// ```
    ThematicBreakSequence,
}

/// List of void events, used to make sure everything is working well.
pub const VOID_EVENTS: [Name; 55] = [
    Name::AttentionSequence,
    Name::AutolinkEmail,
    Name::AutolinkMarker,
    Name::AutolinkProtocol,
    Name::BlankLineEnding,
    Name::BlockQuoteMarker,
    Name::ByteOrderMark,
    Name::CharacterEscapeMarker,
    Name::CharacterEscapeValue,
    Name::CharacterReferenceMarker,
    Name::CharacterReferenceMarkerHexadecimal,
    Name::CharacterReferenceMarkerNumeric,
    Name::CharacterReferenceMarkerSemi,
    Name::CharacterReferenceValue,
    Name::CodeFencedFenceSequence,
    Name::CodeFlowChunk,
    Name::CodeTextData,
    Name::CodeTextSequence,
    Name::Data,
    Name::DefinitionDestinationLiteralMarker,
    Name::DefinitionLabelMarker,
    Name::DefinitionMarker,
    Name::DefinitionTitleMarker,
    Name::EmphasisSequence,
    Name::FrontmatterChunk,
    Name::GfmAutolinkLiteralEmail,
    Name::GfmAutolinkLiteralProtocol,
    Name::GfmAutolinkLiteralWww,
    Name::GfmFootnoteCallMarker,
    Name::GfmFootnoteDefinitionLabelMarker,
    Name::GfmFootnoteDefinitionMarker,
    Name::GfmStrikethroughSequence,
    Name::GfmTaskListItemMarker,
    Name::GfmTaskListItemValueChecked,
    Name::GfmTaskListItemValueUnchecked,
    Name::FrontmatterSequence,
    Name::HardBreakEscape,
    Name::HardBreakTrailing,
    Name::HeadingAtxSequence,
    Name::HeadingSetextUnderline,
    Name::HtmlFlowData,
    Name::HtmlTextData,
    Name::LabelImageMarker,
    Name::LabelMarker,
    Name::LineEnding,
    Name::ListItemMarker,
    Name::ListItemValue,
    Name::MathTextData,
    Name::MathTextSequence,
    Name::ReferenceMarker,
    Name::ResourceMarker,
    Name::ResourceTitleMarker,
    Name::SpaceOrTab,
    Name::StrongSequence,
    Name::ThematicBreakSequence,
];

/// Embedded content type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Content {
    /// Represents [flow content][crate::construct::flow].
    Flow,
    /// Represents [string content][crate::construct::string].
    String,
    /// Represents [text content][crate::construct::text].
    Text,
}

/// Link to another event.
#[derive(Clone, Debug)]
pub struct Link {
    /// Previous event.
    pub previous: Option<usize>,
    /// Next event.
    pub next: Option<usize>,
    /// Content type.
    pub content: Content,
}

/// Place in the document.
///
/// The interface for the location in the document comes from unist
/// [`Point`](https://github.com/syntax-tree/unist#point).
#[derive(Clone, Debug)]
pub struct Point {
    /// 1-indexed line number.
    pub line: usize,
    /// 1-indexed column number.
    ///
    /// This is increased up to a tab stop for tabs.
    /// Some editors count tabs as 1 character, so this position is not the
    /// same as editors.
    pub column: usize,
    /// 0-indexed position in the document.
    ///
    /// Also an `index` into `bytes`.
    pub index: usize,
    /// Virtual step on the same `index`.
    pub vs: usize,
}

impl Point {
    /// Create a new point, that is shifted from the close earlier current
    /// point, to `index.`
    // To do: tabs.
    pub fn shift_to(&self, bytes: &[u8], index: usize) -> Point {
        let mut next = self.clone();
        debug_assert!(index > next.index, "expect");

        while next.index < index {
            match bytes[next.index] {
                b'\n' | b'\r' => unreachable!("cannot move past line endings"),
                b'\t' => {
                    unreachable!("to do: tab")
                    // let remainder = next.column % TAB_SIZE;
                    // let vs = if remainder == 0 {
                    //     0
                    // } else {
                    //     TAB_SIZE - remainder
                    // };

                    // next.index += 1;
                    // next.column += 1 + vs;
                }
                _ => {
                    next.index += 1;
                    next.column += 1;
                }
            }
        }

        next
    }
}

/// Event kinds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Kind {
    /// The start of something.
    Enter,
    /// The end of something.
    Exit,
}

/// Something semantic happening somewhere.
#[derive(Clone, Debug)]
pub struct Event {
    /// Kind of event.
    pub kind: Kind,
    /// Name of event.
    pub name: Name,
    /// Place where this happens.
    pub point: Point,
    /// Link to another event.
    pub link: Option<Link>,
}
