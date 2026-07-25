use crate::idl::IdlAst;

pub struct SchemaGenerator;

impl Default for SchemaGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl SchemaGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_protobuf(&self, ast: &IdlAst) -> String {
        let mut out = String::from("syntax = \"proto3\";\n\npackage uds.v1;\n\n");
        for msg in &ast.messages {
            out.push_str(&format!("message {} {{\n", msg.name));
            for (i, field) in msg.fields.iter().enumerate() {
                let pb_type = self.type_to_protobuf(&field.field_type);
                out.push_str(&format!("  {} {} = {};\n", pb_type, field.name, i + 1));
            }
            out.push_str("}\n\n");
        }
        for svc in &ast.services {
            out.push_str(&format!("service {} {{\n", svc.name));
            for method in &svc.methods {
                out.push_str(&format!(
                    "  rpc {}({}) returns ({});\n",
                    method.name, method.params_type, method.return_type
                ));
            }
            out.push_str("}\n\n");
        }
        out
    }

    pub fn generate_flatbuffers(&self, ast: &IdlAst) -> String {
        let mut out = String::from("// Generated FlatBuffers schema\n\n");
        out.push_str("namespace UDS.V1;\n\n");
        for msg in &ast.messages {
            out.push_str(&format!("table {} {{\n", msg.name));
            for field in &msg.fields {
                let fb_type = self.type_to_flatbuffers(&field.field_type);
                out.push_str(&format!("  {}:{};\n", field.name, fb_type));
            }
            out.push_str("}\n\n");
        }
        out
    }

    fn type_to_protobuf<'a>(&self, t: &'a str) -> &'a str {
        match t {
            "string" => "string",
            "uint32" => "uint32",
            "uint16" => "uint32",
            "uint8" => "uint32",
            "int32" => "sint32",
            "float" => "float",
            "double" => "double",
            "bool" => "bool",
            "bytes" => "bytes",
            _ => t,
        }
    }

    fn type_to_flatbuffers<'a>(&self, t: &'a str) -> &'a str {
        match t {
            "string" => "string",
            "uint32" => "uint32",
            "uint16" => "uint16",
            "uint8" => "ubyte",
            "int32" => "int32",
            "float" => "float",
            "double" => "double",
            "bool" => "bool",
            "bytes" => "[ubyte]",
            _ => t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::idl::IdlParser;

    #[test]
    fn test_protobuf_generation() {
        let input = r#"
message Status {
    string device_name;
    uint32 uptime_seconds;
}
"#;
        let ast = IdlParser::new().parse(input).unwrap();
        let gen = SchemaGenerator::new();
        let output = gen.generate_protobuf(&ast);
        assert!(output.contains("message Status"));
        assert!(output.contains("string device_name = 1;"));
        assert!(output.contains("uint32 uptime_seconds = 2;"));
    }
}
