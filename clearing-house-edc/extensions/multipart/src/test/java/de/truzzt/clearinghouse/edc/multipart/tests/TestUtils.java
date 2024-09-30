package de.truzzt.clearinghouse.edc.multipart.tests;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.tests.BaseTestUtils;
import de.truzzt.clearinghouse.edc.types.ids.Message;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

public class TestUtils extends BaseTestUtils {

    public static final String VALID_HEADER_JSON = "headers/valid-header.json";
    public static final String VALID_CREATE_PROCESS_HEADER_JSON = "headers/valid-create-process-header.json";
    public static final String INVALID_HEADER_JSON = "headers/invalid-header.json";
    public static final String INVALID_TYPE_HEADER_JSON = "headers/invalid-type.json";
    public static final String INVALID_TOKEN_HEADER_JSON = "headers/invalid-token.json";
    public static final String MISSING_FIELDS_HEADER_JSON = "headers/missing-fields.json";
    public static final String MISSING_TOKEN_HEADER_JSON = "headers/missing-token.json";
    public static final String VALID_RESPONSE_HEADER_JSON = "headers/valid-response.json";

    public static final String VALID_RESPONSE_PAYLOAD_JSON = "payloads/valid-response.json";

    public static InputStream getHeaderInputStream(String path) {
        var json = TestUtils.readFile(path);
        return new ByteArrayInputStream(json.getBytes());
    }

    public static Message getResponseHeader(ObjectMapper mapper, String path) {
        return parseFile(mapper, Message.class, path);
    }

    public static Message getValidResponseHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_RESPONSE_HEADER_JSON);
    }

    public static LoggingMessageResponse getValidResponsePayload(ObjectMapper mapper) {
        return parseFile(mapper, LoggingMessageResponse.class, VALID_RESPONSE_PAYLOAD_JSON);
    }
}
