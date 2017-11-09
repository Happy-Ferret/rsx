/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

#![cfg(any(feature = "rsx-macro", feature = "css-macro"))]
#![feature(proc_macro)]

extern crate rsx;
extern crate rsx_primitives;

#[cfg(feature = "css-macro")]
use rsx::css;
#[cfg(feature = "rsx-macro")]
use rsx::rsx;
#[cfg(feature = "rsx-macro")]
use rsx_primitives::rsx_dom::types::*;
#[cfg(feature = "css-macro")]
use rsx_primitives::rsx_stylesheet::types::*;
use std::borrow::Cow;

#[cfg(feature = "rsx-macro")]
#[test]
fn test_rsx_to_ast() {
    struct Props {
        visible: bool,
        menu: MenuProps
    }

    struct MenuProps {
        icon: String
    }

    fn should_do_something_fun() -> bool {
        true
    }

    fn what_fun() -> String {
        "Something Fun!".to_string()
    }

    fn what_else() -> String {
        "Something Else".to_string()
    }

    let props = Props {
        visible: true,
        menu: MenuProps {
            icon: "icon.png".to_string()
        }
    };

    let ast: RSXElement = rsx! {
        <Dropdown show={props.visible}>
            A dropdown list
            <Menu icon={props.menu.icon}>
                <MenuItem>Do Something</MenuItem>
                {
                    if should_do_something_fun() {
                        <MenuItem>Do{ what_fun() }</MenuItem>
                    } else {
                        <MenuItem>Do{ what_else() }</MenuItem>
                    }
                }
            </Menu>
        </Dropdown>
    };

    let expected = RSXElement::Normal(RSXNormalElement(
        RSXElementName::Name(RSXIdentifier("Dropdown".into())),
        RSXAttributes(vec![
            RSXAttribute::Named(
                RSXAttributeName::Name(RSXIdentifier("show".into())),
                RSXAttributeValue::CodeBlock(ArbitraryCodeRegion::RuntimeValue(
                    ComputedValue(Box::new(true))
                ))
            ),
        ]),
        RSXChildren(vec![
            RSXChild::Text(RSXText("A dropdown list".into())),
            RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                RSXElementName::Name(RSXIdentifier("Menu".into())),
                RSXAttributes(vec![
                    RSXAttribute::Named(
                        RSXAttributeName::Name(RSXIdentifier("icon".into())),
                        RSXAttributeValue::CodeBlock(ArbitraryCodeRegion::RuntimeValue(
                            ComputedValue(Box::new("icon.png"))
                        ))
                    ),
                ]),
                RSXChildren(vec![
                    RSXChild::Element(RSXElement::Normal(RSXNormalElement(
                        RSXElementName::Name(RSXIdentifier("MenuItem".into())),
                        RSXAttributes(vec![]),
                        RSXChildren(vec![RSXChild::Text(RSXText("Do Something".into()))])
                    ))),
                    RSXChild::CodeBlock(ArbitraryCodeRegion::RuntimeValue(ComputedValue(Box::new({
                        RSXElement::Normal(RSXNormalElement(
                            RSXElementName::Name(RSXIdentifier("MenuItem".into())),
                            RSXAttributes(vec![]),
                            RSXChildren(vec![
                                RSXChild::Text(RSXText("Do".into())),
                                RSXChild::CodeBlock(ArbitraryCodeRegion::RuntimeValue(
                                    ComputedValue(Box::new("Something Fun!"))
                                )),
                            ])
                        ))
                    })))),
                ])
            ))),
        ])
    ));

    assert_eq!(ast.to_string(), expected.to_string());
}

#[cfg(feature = "rsx-macro")]
#[test]
fn test_rsx_to_node() {
    struct Props {
        visible: bool,
        menu: MenuProps
    }

    struct MenuProps {
        icon: String
    }

    fn should_do_something_fun() -> bool {
        true
    }

    fn what_fun() -> String {
        "Something Fun!".to_string()
    }

    fn what_else() -> String {
        "Something Else".to_string()
    }

    let props = Props {
        visible: true,
        menu: MenuProps {
            icon: "icon.png".to_string()
        }
    };

    let node: Node = rsx! {
        <Dropdown show={props.visible}>
            A dropdown list
            <Menu icon={props.menu.icon}>
                <MenuItem>Do Something</MenuItem>
                {
                    if should_do_something_fun() {
                        <MenuItem>Do{ what_fun() }</MenuItem>
                    } else {
                        <MenuItem>Do{ what_else() }</MenuItem>
                    }
                }
            </Menu>
        </Dropdown>
    };

    let expected = Node::Normal {
        name: NodeName::Simple(Cow::from("Dropdown")),
        attributes: vec![
            Attribute(
                AttributeName::Simple(Cow::from("show")),
                RuntimeValue::Boolean(true)
            ),
        ],
        children: vec![
            Node::Text {
                contents: Cow::from("A dropdown list")
            },
            Node::Normal {
                name: NodeName::Simple(Cow::from("Menu")),
                attributes: vec![
                    Attribute(
                        AttributeName::Simple(Cow::from("icon")),
                        RuntimeValue::Str(Cow::from("icon.png"))
                    ),
                ],
                children: vec![
                    Node::Normal {
                        name: NodeName::Simple(Cow::from("MenuItem")),
                        attributes: vec![],
                        children: vec![
                            Node::Text {
                                contents: Cow::from("Do Something")
                            },
                        ]
                    },
                    Node::Normal {
                        name: NodeName::Simple(Cow::from("MenuItem")),
                        attributes: vec![],
                        children: vec![
                            Node::Text {
                                contents: Cow::from("Do")
                            },
                            Node::Text {
                                contents: Cow::from("Something Fun!")
                            },
                        ]
                    },
                ]
            },
        ]
    };

    assert_eq!(node, expected);
}

#[cfg(feature = "css-macro")]
#[test]
fn test_css_to_stylesheet_1() {
    let stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet(vec![
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".foo".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]
        },
    ]);

    assert_eq!(stylesheet, expected);
}

#[cfg(feature = "css-macro")]
#[test]
fn test_css_to_stylesheet_2() {
    let stylesheet = css! {
        .foo, .bar {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let expected = Stylesheet(vec![
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".foo".into(),
                    specificity: 1024u32
                },
                StyleSelector {
                    selector: ".bar".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
            ]
        },
    ]);

    assert_eq!(stylesheet, expected);
}

#[cfg(feature = "css-macro")]
#[test]
fn test_css_to_stylesheet_3() {
    let stylesheet = css! {
        .foo, .bar-baz {
            margin: 0 auto;
            padding: 10px;
            flex-wrap: nowrap;
            flex-direction: row-reverse;
        }
    };

    let expected = Stylesheet(vec![
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".foo".into(),
                    specificity: 1024u32
                },
                StyleSelector {
                    selector: ".bar-baz".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0f32.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10f32.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexWrap(Wrap::NoWrap)),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::RowReverse)),
            ]
        },
    ]);

    assert_eq!(stylesheet, expected);
}

#[cfg(feature = "css-macro")]
#[test]
fn test_css_to_stylesheet_4() {
    let stylesheet = css!("tests/fixtures/test_1.css");

    let expected = Stylesheet(vec![
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".root".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
            ]
        },
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".image".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
            ]
        },
        StyleRule {
            selectors: vec![
                StyleSelector {
                    selector: ".text".into(),
                    specificity: 1024u32
                },
            ],
            declarations: vec![
                StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
            ]
        },
    ]);

    assert_eq!(stylesheet, expected);
}

#[cfg(all(feature = "rsx-macro", feature = "css-macro"))]
#[test]
fn test_rsx_and_css_1() {
    let mut stylesheet = css! {
        .foo {
            margin: 0 auto;
            padding: 10px;
        }
    };

    let node: Node = rsx! {
        <div style={stylesheet.get(".foo")}>
            Hello world!
        </div>
    };

    assert_eq!(
        node,
        Node::Normal {
            name: NodeName::Simple(Cow::from("div")),
            attributes: vec![
                Attribute(
                    AttributeName::Simple(Cow::from("style")),
                    RuntimeValue::Styles(vec![
                        StyleDeclaration::Layout(FlexStyle::MarginTop(StyleUnit::Point(0.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Auto)),
                        StyleDeclaration::Layout(FlexStyle::MarginBottom(StyleUnit::Point(0.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::MarginLeft(StyleUnit::Auto)),
                        StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(10.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(10.0.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(10f32.into()))),
                        StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(10.0.into()))),
                    ])
                ),
            ],
            children: vec![
                Node::Text {
                    contents: Cow::from("Hello world !")
                },
            ]
        }
    );
}

#[cfg(all(feature = "rsx-macro", feature = "css-macro"))]
#[test]
fn test_rsx_and_css_2() {
    let mut stylesheet = css!("tests/fixtures/test_2.css");

    let node: Node = rsx! {
        <div style={stylesheet.get(".bar")}>
            Hello world!
        </div>
    };

    assert_eq!(
        node,
        Node::Normal {
            name: NodeName::Simple(Cow::from("div")),
            attributes: vec![
                Attribute(
                    AttributeName::Simple(Cow::from("style")),
                    RuntimeValue::Styles(vec![])
                ),
            ],
            children: vec![
                Node::Text {
                    contents: Cow::from("Hello world !")
                },
            ]
        }
    );
}

#[cfg(all(feature = "rsx-macro", feature = "css-macro"))]
#[test]
fn test_example_1() {
    let mut stylesheet: Stylesheet = css! {
        .root {
            width: 500px;
            height: 120px;
            flex-direction: row;
            padding: 20px;
        }
        .image {
            width: 80px;
            margin-right: 20px;
        }
        .text {
            height: 25px;
            align-self: center;
            flex-grow: 1;
        }
    };

    let node: Node = rsx! {
        <view style={stylesheet.get(".root")}>
            <image style={stylesheet.get(".image")} src="..." />
            <text style={stylesheet.get(".text")}>
                Hello world!
            </text>
        </view>
    };

    let expected = Node::Normal {
        name: NodeName::Simple("view".into()),
        attributes: vec![
            Attribute(
                AttributeName::Simple("style".into()),
                RuntimeValue::Styles(vec![
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                ])
            ),
        ],
        children: vec![
            Node::Normal {
                name: NodeName::Simple("image".into()),
                attributes: vec![
                    Attribute(
                        AttributeName::Simple("style".into()),
                        RuntimeValue::Styles(vec![
                            StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                            StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                        ])
                    ),
                    Attribute(
                        AttributeName::Simple("src".into()),
                        RuntimeValue::Str("...".into())
                    ),
                ],
                children: vec![]
            },
            Node::Normal {
                name: NodeName::Simple("text".into()),
                attributes: vec![
                    Attribute(
                        AttributeName::Simple("style".into()),
                        RuntimeValue::Styles(vec![
                            StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                            StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                            StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                        ])
                    ),
                ],
                children: vec![
                    Node::Text {
                        contents: "Hello world !".into()
                    },
                ]
            },
        ]
    };

    assert_eq!(node, expected);
}

#[cfg(all(feature = "rsx-macro", feature = "css-macro"))]
#[test]
fn test_example_2() {
    fn greeting_str(name: &str) -> String {
        format!("Hello {}!", name)
    }

    fn render_greeting(name: &str) -> NodeFragment {
        let mut stylesheet = css!("tests/fixtures/test_1.css");

        rsx! {
            <text style={stylesheet.get(".text".into())}>
                { greeting_str(name) }
            </text>
        }
    }

    fn render_children(name: Option<&str>, image: NodeFragment) -> NodeFragment {
        rsx! {
            <view>
                { image }
                {
                    match name {
                        Some(ref n) => render_greeting(n),
                        None => <text>No greetings!</text>
                    }
                }
            </view>
        }
    }

    fn render_root() -> NodeFragment {
        let mut stylesheet = css!("tests/fixtures/test_1.css");

        rsx! {
            <view style={stylesheet.get(".root".into())}>
                {
                    let name = Some("world");
                    let image = <image style={stylesheet.get(".image")} src="..." />;
                    render_children(name, image)
                }
            </view>
        }
    }

    let node: Node = render_root().into();

    let expected = Node::Normal {
        name: NodeName::Simple("view".into()),
        attributes: vec![
            Attribute(
                AttributeName::Simple("style".into()),
                RuntimeValue::Styles(vec![
                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(500.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(120.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::FlexDirection(FlexDirection::Row)),
                    StyleDeclaration::Layout(FlexStyle::PaddingTop(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingRight(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingBottom(StyleUnit::Point(20.0.into()))),
                    StyleDeclaration::Layout(FlexStyle::PaddingLeft(StyleUnit::Point(20.0.into()))),
                ])
            ),
        ],
        children: vec![
            Node::Normal {
                name: NodeName::Simple("view".into()),
                attributes: vec![],
                children: vec![
                    Node::Normal {
                        name: NodeName::Simple("image".into()),
                        attributes: vec![
                            Attribute(
                                AttributeName::Simple("style".into()),
                                RuntimeValue::Styles(vec![
                                    StyleDeclaration::Layout(FlexStyle::Width(StyleUnit::Point(80.0.into()))),
                                    StyleDeclaration::Layout(FlexStyle::MarginRight(StyleUnit::Point(20.0.into()))),
                                ])
                            ),
                            Attribute(
                                AttributeName::Simple("src".into()),
                                RuntimeValue::Str("...".into())
                            ),
                        ],
                        children: vec![]
                    },
                    Node::Normal {
                        name: NodeName::Simple("text".into()),
                        attributes: vec![
                            Attribute(
                                AttributeName::Simple("style".into()),
                                RuntimeValue::Styles(vec![
                                    StyleDeclaration::Layout(FlexStyle::Height(StyleUnit::Point(25.0.into()))),
                                    StyleDeclaration::Layout(FlexStyle::AlignSelf(Align::Center)),
                                    StyleDeclaration::Layout(FlexStyle::FlexGrow(1.0.into())),
                                ])
                            ),
                        ],
                        children: vec![
                            Node::Text {
                                contents: "Hello world!".into()
                            },
                        ]
                    },
                ]
            },
        ]
    };

    assert_eq!(node, expected);
}
