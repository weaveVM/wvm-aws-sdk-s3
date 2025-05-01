use xmlwriter::{Options, XmlWriter};

pub struct ErrorXmlFactory {
    code: String,
    message: String,
    resource: String,
    request_id: String,
}

impl ErrorXmlFactory {
    pub fn new() -> Self {
        Self {
            code: String::new(),
            message: String::new(),
            resource: String::new(),
            request_id: String::new(),
        }
    }

    pub fn unauthorized() -> Self {
        Self::new().code("NoSuchKey").message(
            "Request could not be authenticated. Make sure you own a load.network valid API key",
        )
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = value.into();
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = value.into();
        self
    }

    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = value.into();
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = value.into();
        self
    }

    pub fn build(self) -> String {
        let mut writer = XmlWriter::new(Options::default());

        writer.start_element("Error");

        // <Code>...</Code>
        writer.start_element("Code");
        writer.write_text(&self.code);
        writer.end_element();

        // <Message>...</Message>
        writer.start_element("Message");
        writer.write_text(&self.message);
        writer.end_element();

        // <Resource>...</Resource>
        writer.start_element("Resource");
        writer.write_text(&self.resource);
        writer.end_element();

        // <RequestId>...</RequestId>
        writer.start_element("RequestId");
        writer.write_text(&self.request_id);
        writer.end_element();

        writer.end_element(); // </Error>
        writer.end_document()
    }
}

#[cfg(test)]
mod xml_test {
    use crate::error::ErrorXmlFactory;

    #[test]
    fn test_xml_error_writer() {
        let xml = ErrorXmlFactory::new()
            .request_id("A")
            .message("B")
            .code("C")
            .resource("D")
            .build();

        println!("{}", xml);
    }
}
