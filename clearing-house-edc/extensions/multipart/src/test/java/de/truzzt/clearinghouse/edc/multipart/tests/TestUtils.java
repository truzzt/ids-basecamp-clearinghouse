package de.truzzt.clearinghouse.edc.multipart.tests;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.AppAvailableMessage;
import de.fraunhofer.iais.eis.LogMessageImpl;
import de.fraunhofer.iais.eis.RequestMessageImpl;
import de.truzzt.clearinghouse.edc.app.message.CreateProcessResponse;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.tests.BaseTestUtils;

import java.io.ByteArrayInputStream;
import java.io.InputStream;

public class TestUtils extends BaseTestUtils {

    public static final String VALID_CREATE_PROCESS_HEADER_JSON = "headers/valid-create-process-header.json";
    public static final String VALID_CREATE_PROCESS_RESPONSE_PAYLOAD_JSON = "payloads/valid-create-process-response-payload.json";
    public static final String VALID_LOG_MESSAGE_HEADER_JSON = "headers/valid-log-message-header.json";
    public static final String VALID_LOG_MESSAGE_RESPONSE_PAYLOAD_JSON = "payloads/valid-log-message-response-payload.json";
    public static final String INVALID_HEADER_JSON = "headers/invalid-header.json";
    public static final String INVALID_TYPE_HEADER_JSON = "headers/invalid-type.json";
    public static final String INVALID_TOKEN_HEADER_JSON = "headers/invalid-token.json";
    public static final String MISSING_FIELDS_HEADER_JSON = "headers/missing-fields.json";
    public static final String MISSING_TOKEN_HEADER_JSON = "headers/missing-token.json";

    public static InputStream getHeaderInputStream(String path) {
        var json = TestUtils.readFile(path);
        return new ByteArrayInputStream(json.getBytes());
    }

    public static RequestMessageImpl getCreateProcessResponseHeader(ObjectMapper mapper) {
        return parseFile(mapper, RequestMessageImpl.class, VALID_CREATE_PROCESS_HEADER_JSON);
    }

    public static CreateProcessResponse getValidCreateProcessResponsePayload(ObjectMapper mapper) {
        return parseFile(mapper, CreateProcessResponse.class, VALID_CREATE_PROCESS_RESPONSE_PAYLOAD_JSON);
    }

    public static LogMessageImpl getValidLogMessageHeader(ObjectMapper mapper) {
        return parseFile(mapper, LogMessageImpl.class, VALID_LOG_MESSAGE_HEADER_JSON);
    }

    public static LoggingMessageResponse getValidLogMessageResponsePayload(ObjectMapper mapper) {
        return parseFile(mapper, LoggingMessageResponse.class, VALID_LOG_MESSAGE_RESPONSE_PAYLOAD_JSON);
    }

    public static AppAvailableMessage getInvalidTypeMessageResponsePayload(ObjectMapper mapper) {
        return parseFile(mapper, AppAvailableMessage.class, INVALID_TYPE_HEADER_JSON);
    }
}
