use cmark_writer::ast::{HeadingType, ListItem, Node};

// #[test]
// fn test_alignment() {
//     // Test alignment
//     let alignment = Alignment::Center;
//     assert_eq!(alignment, Alignment::Center);

//     // Test different alignments
//     let left_alignment = Alignment::Left;
//     let right_alignment = Alignment::Right;

//     assert_ne!(alignment, left_alignment);
//     assert_ne!(alignment, right_alignment);
// }

#[test]
fn test_node_equality() {
    // Test inline node equality
    let text1 = Node::Text("Hello".to_string());
    let text2 = Node::Text("Hello".to_string());
    let text3 = Node::Text("World".to_string());

    assert_eq!(text1, text2);
    assert_ne!(text1, text3);

    // Test complex node equality
    let heading1 = Node::Heading {
        level: 2,
        content: vec![Node::Text("Title".to_string())],
        heading_type: HeadingType::Atx,
    };

    let heading2 = Node::Heading {
        level: 2,
        content: vec![Node::Text("Title".to_string())],
        heading_type: HeadingType::Atx,
    };

    let heading3 = Node::Heading {
        level: 3, // Different level
        content: vec![Node::Text("Title".to_string())],
        heading_type: HeadingType::Atx,
    };

    assert_eq!(heading1, heading2);
    assert_ne!(heading1, heading3);
}

#[test]
fn test_document_creation() {
    // Create a test document and verify its structure
    let document = Node::Document(vec![
        Node::Heading {
            level: 1,
            content: vec![Node::Text("Document Title".to_string())],
            heading_type: HeadingType::Atx,
        },
        Node::Paragraph(vec![
            Node::Text("Paragraph with ".to_string()),
            Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
            Node::Text(" and ".to_string()),
            Node::Strong(vec![Node::Text("strong".to_string())]),
            Node::Text(" text.".to_string()),
        ]),
    ]);

    // Check document structure
    if let Node::Document(children) = &document {
        assert_eq!(children.len(), 2);

        // Verify heading
        if let Node::Heading {
            level,
            content,
            heading_type: _,
        } = &children[0]
        {
            assert_eq!(*level, 1);
            assert_eq!(content.len(), 1);
            assert_eq!(content[0], Node::Text("Document Title".to_string()));
        } else {
            panic!("First child should be a heading");
        }

        // Verify paragraph
        if let Node::Paragraph(content) = &children[1] {
            assert_eq!(content.len(), 5);
            assert_eq!(content[0], Node::Text("Paragraph with ".to_string()));

            if let Node::Emphasis(emph_content) = &content[1] {
                assert_eq!(emph_content[0], Node::Text("emphasis".to_string()));
            } else {
                panic!("Second element should be emphasis");
            }

            if let Node::Strong(strong_content) = &content[3] {
                assert_eq!(strong_content[0], Node::Text("strong".to_string()));
            } else {
                panic!("Fourth element should be strong");
            }
        } else {
            panic!("Second child should be a paragraph");
        }
    } else {
        panic!("Document should be of type Document");
    }
}

#[test]
fn test_list_item() {
    let unordered_item = ListItem::Unordered {
        content: vec![Node::Paragraph(vec![Node::Text(
            "Unordered item".to_string(),
        )])],
    };

    if let ListItem::Unordered { content } = &unordered_item {
        assert_eq!(content.len(), 1);
        if let Node::Paragraph(text) = &content[0] {
            assert_eq!(text[0], Node::Text("Unordered item".to_string()));
        } else {
            panic!("Content should be a paragraph");
        }
    } else {
        panic!("Should be an unordered list item");
    }

    let ordered_item = ListItem::Ordered {
        number: Some(3),
        content: vec![Node::Paragraph(vec![Node::Text(
            "Ordered item".to_string(),
        )])],
    };

    if let ListItem::Ordered { number, content } = &ordered_item {
        assert_eq!(*number, Some(3));
        assert_eq!(content.len(), 1);
        if let Node::Paragraph(text) = &content[0] {
            assert_eq!(text[0], Node::Text("Ordered item".to_string()));
        } else {
            panic!("Content should be a paragraph");
        }
    } else {
        panic!("Should be an ordered list item");
    }
}

#[test]
fn test_node_type_checking() {
    // Test block node type checking
    let heading = Node::Heading {
        level: 1,
        content: vec![Node::Text("Heading".to_string())],
        heading_type: HeadingType::Atx,
    };
    assert!(heading.is_block());
    assert!(!heading.is_inline());

    // Test inline node type checking
    let text = Node::Text("Hello".to_string());
    assert!(text.is_inline());
    assert!(!text.is_block());

    // Test nested nodes
    let paragraph = Node::Paragraph(vec![
        Node::Text("Text with ".to_string()),
        Node::Emphasis(vec![Node::Text("emphasis".to_string())]),
    ]);
    assert!(paragraph.is_block());
    assert!(!paragraph.is_inline());
}

#[test]
fn test_heading_constructor() {
    // 测试新添加的 heading 构造方法
    let heading = Node::heading(2, vec![Node::Text("标题".to_string())]);

    // 验证结构
    if let Node::Heading {
        level,
        content,
        heading_type,
    } = &heading
    {
        assert_eq!(*level, 2);
        assert_eq!(content.len(), 1);
        assert_eq!(content[0], Node::Text("标题".to_string()));
        assert_eq!(*heading_type, HeadingType::Atx); // 默认应该是 Atx 类型
    } else {
        panic!("应该是 Heading 节点");
    }

    // 验证与手动构造的等价性
    let manual_heading = Node::Heading {
        level: 2,
        content: vec![Node::Text("标题".to_string())],
        heading_type: HeadingType::default(),
    };

    assert_eq!(heading, manual_heading);
}

#[test]
fn test_code_block_constructor() {
    // 测试带语言标识的代码块
    let rust_code = Node::code_block(Some("rust".to_string()), "fn main() {}\n".to_string());

    if let Node::CodeBlock {
        language,
        content,
        block_type,
    } = &rust_code
    {
        assert_eq!(*language, Some("rust".to_string()));
        assert_eq!(*content, "fn main() {}\n".to_string());
        assert!(matches!(
            *block_type,
            cmark_writer::ast::CodeBlockType::Fenced
        ));
    } else {
        panic!("应该是 CodeBlock 节点");
    }

    // 测试不带语言标识的代码块
    let plain_code = Node::code_block(None, "plain text".to_string());

    if let Node::CodeBlock {
        language,
        content,
        block_type,
    } = &plain_code
    {
        assert_eq!(*language, None);
        assert_eq!(*content, "plain text".to_string());
        assert!(matches!(
            *block_type,
            cmark_writer::ast::CodeBlockType::Fenced
        ));
    } else {
        panic!("应该是 CodeBlock 节点");
    }
}
