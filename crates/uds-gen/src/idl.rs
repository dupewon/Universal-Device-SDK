#[derive(Debug, Clone)]
pub struct IdlAst {
    pub services: Vec<ServiceDef>,
    pub messages: Vec<MessageDef>,
}

#[derive(Debug, Clone)]
pub struct ServiceDef {
    pub name: String,
    pub methods: Vec<MethodDef>,
}

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub name: String,
    pub params_type: String,
    pub return_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MessageDef {
    pub name: String,
    pub fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub name: String,
    pub field_type: String,
    pub is_optional: bool,
}

pub struct IdlParser;

impl IdlParser {
    pub fn new() -> Self {
        Self
    }

    /// Parse a UDS IDL string into an AST.
    /// Format:
    ///   service ServiceName {
    ///       rpc MethodName(params_type) returns (return_type);
    ///   }
    ///
    ///   message MessageName {
    ///       field_type field_name;
    ///   }
    pub fn parse(&self, input: &str) -> Result<IdlAst, String> {
        let mut services = Vec::new();
        let mut messages = Vec::new();
        let mut current_service: Option<ServiceDef> = None;
        let mut current_message: Option<MessageDef> = None;

        for (line_no, raw_line) in input.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with("//") || line.starts_with('#') {
                continue;
            }

            // Service block start
            if line.starts_with("service ") && line.ends_with('{') {
                if let Some(svc) = current_service.take() {
                    services.push(svc);
                }
                let name = line
                    .trim_start_matches("service ")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
                current_service = Some(ServiceDef {
                    name,
                    methods: Vec::new(),
                });
                continue;
            }

            // Message block start
            if line.starts_with("message ") && line.ends_with('{') {
                if let Some(svc) = current_service.take() {
                    services.push(svc);
                }
                if let Some(msg) = current_message.take() {
                    messages.push(msg);
                }
                let name = line
                    .trim_start_matches("message ")
                    .trim_end_matches('{')
                    .trim()
                    .to_string();
                current_message = Some(MessageDef {
                    name,
                    fields: Vec::new(),
                });
                continue;
            }

            // Closing brace
            if line == "}" {
                if let Some(svc) = current_service.take() {
                    services.push(svc);
                }
                if let Some(msg) = current_message.take() {
                    messages.push(msg);
                }
                continue;
            }

            // RPC method definition
            if line.starts_with("rpc ") {
                let inner = line.trim_start_matches("rpc ").trim_end_matches(';').trim();
                if let Some((name, rest)) = inner.split_once('(') {
                    if let Some((params, rest)) = rest.split_once(')') {
                        let return_type = if let Some(rt) = rest.trim().strip_prefix("returns (") {
                            rt.trim_end_matches(')').trim()
                        } else {
                            "void"
                        };
                        if let Some(ref mut svc) = current_service {
                            svc.methods.push(MethodDef {
                                name: name.trim().to_string(),
                                params_type: params.trim().to_string(),
                                return_type: return_type.to_string(),
                                description: None,
                            });
                        }
                    }
                }
                continue;
            }

            // Field definition
            if !line.starts_with('{')
                && !line.starts_with('}')
                && line.contains(' ')
                && line.ends_with(';')
            {
                let field = line.trim_end_matches(';').trim();
                if let Some((ft, fn_name)) = field.split_once(' ') {
                    let is_optional = ft.ends_with('?');
                    let ft_clean = ft.trim_end_matches('?');
                    if let Some(ref mut msg) = current_message {
                        msg.fields.push(FieldDef {
                            name: fn_name.trim().to_string(),
                            field_type: ft_clean.to_string(),
                            is_optional,
                        });
                    }
                }
            }
        }

        // Flush remaining
        if let Some(svc) = current_service {
            services.push(svc);
        }
        if let Some(msg) = current_message {
            messages.push(msg);
        }

        Ok(IdlAst { services, messages })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_idl() {
        let input = r#"
service Device {
    rpc GetStatus() returns (Status);
    rpc SetLed(bool on) returns (void);
}

message Status {
    string device_name;
    uint32 uptime_seconds;
}
"#;
        let parser = IdlParser::new();
        let ast = parser.parse(input).unwrap();
        assert_eq!(ast.services.len(), 1);
        assert_eq!(ast.services[0].name, "Device");
        assert_eq!(ast.services[0].methods.len(), 2);
        assert_eq!(ast.messages.len(), 1);
        assert_eq!(ast.messages[0].fields.len(), 2);
    }

    #[test]
    fn test_empty_idl() {
        let parser = IdlParser::new();
        let ast = parser.parse("").unwrap();
        assert!(ast.services.is_empty());
        assert!(ast.messages.is_empty());
    }
}
