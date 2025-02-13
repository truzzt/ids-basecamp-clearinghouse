package de.truzzt.clearinghouse.edc.tests;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.CreateProcessRequest;
import de.truzzt.clearinghouse.edc.dto.CreateProcessResponse;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Context;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import de.truzzt.clearinghouse.edc.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.types.clearinghouse.TokenFormat;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import okhttp3.*;
import org.eclipse.edc.spi.EdcException;

import java.io.File;
import java.io.IOException;
import java.util.UUID;

public class TestUtils extends BaseTestUtils {

    public static final String TEST_BASE_URL = "http://localhost:8000";

    private static final String TEST_PAYLOAD = "Hello World";
    private static final String TEST_CREATE_PROCCESS_PAYLOAD = "{ \"owners\": [\"1\", \"2\"]}";;
    private static final String VALID_HEADER_JSON = "headers/valid-header.json";
    private static final String VALID_CREATE_PROCESS_HEADER_JSON = "headers/valid-create-process-header.json";
    private static final String INVALID_HEADER_JSON = "headers/invalid-header.json";
    private static final String INVALID_TYPE_HEADER_JSON = "headers/invalid-type.json";
    private static final String INVALID_TOKEN_HEADER_JSON = "headers/invalid-token.json";

    public static Message getValidHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_HEADER_JSON);
    }

    public static Message getValidCreateProcessHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_CREATE_PROCESS_HEADER_JSON);
    }

    public static Message getInvalidTokenHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, INVALID_TOKEN_HEADER_JSON);
    }

    public static Message getInvalidTypeHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, INVALID_TYPE_HEADER_JSON);
    }

    public static Response getValidResponse(String url) {

        Request mockRequest = new Request.Builder()
                .url(url)
                .build();
        ResponseBody body = getValidResponseBody();

        Headers headers = new Headers.Builder().add("Test","Test").build();

        return new Response(mockRequest, Protocol.HTTP_2, "", 200, null,
                headers, body, null, null,
                null, 1000L, 1000L, null);
    }

    public static Response getResponseWithoutBody(String url) {

        Request mockRequest = new Request.Builder()
                .url(url)
                .build();

        Headers headers = new Headers.Builder().add("Test","Test").build();

        return new Response(mockRequest, Protocol.HTTP_2, "", 200, null,
                headers, null, null, null,
                null, 1000L, 1000L, null);
    }

    public static Response getUnsuccessfulResponse(String url) {

        Request mockRequest = new Request.Builder()
                .url(url)
                .build();
        ResponseBody body = getValidResponseBody();

        Headers headers = new Headers.Builder().add("Test","Test").build();

        return new Response(mockRequest, Protocol.HTTP_2, "Unauthorized", 401, null,
                headers, body, null, null,
                null, 1000L, 1000L, null);
    }

    public static LoggingMessageResponse getValidLoggingMessageResponse(String url, ObjectMapper mapper) {
        try {
            return mapper.readValue(getValidResponse(url).body().byteStream(), LoggingMessageResponse.class);

        } catch (IOException e) {
            throw new EdcException("Error parsing response", e);
        }
    }

    public static CreateProcessResponse getValidCreateProcessResponse(String url, ObjectMapper mapper) {
        try {
            return mapper.readValue(getValidResponse(url).body().byteStream(), CreateProcessResponse.class);

        } catch (IOException e) {
            throw new EdcException("Error parsing response", e);
        }
    }

    public static LoggingMessageRequest getValidLoggingMessageRequest(HandlerRequest handlerRequest) {

        var header = handlerRequest.getHeader();

        var multipartContext = header.getContext();
        var context = new Context(multipartContext.getIds(), multipartContext.getIdsc());

        var multipartSecurityToken = header.getSecurityToken();
        var multipartTokenFormat = multipartSecurityToken.getTokenFormat();
        var securityToken = SecurityToken.Builder.newInstance().
                type(multipartSecurityToken.getType()).
                id(multipartSecurityToken.getId()).
                tokenFormat(new TokenFormat(multipartTokenFormat.getId())).
                tokenValue(multipartSecurityToken.getTokenValue()).
                build();

        var requestHeader = Header.Builder.newInstance()
                .context(context)
                .id(header.getId())
                .type(header.getType())
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return new LoggingMessageRequest(requestHeader, handlerRequest.getPayload());
    }

    public static CreateProcessRequest getValidCreateProcessRequest(HandlerRequest handlerRequest) {

        var header = handlerRequest.getHeader();

        var multipartContext = header.getContext();
        var context = new Context(multipartContext.getIds(), multipartContext.getIdsc());

        var multipartSecurityToken = header.getSecurityToken();
        var multipartTokenFormat = multipartSecurityToken.getTokenFormat();
        var securityToken = SecurityToken.Builder.newInstance().
                type(multipartSecurityToken.getType()).
                id(multipartSecurityToken.getId()).
                tokenFormat(new TokenFormat(multipartTokenFormat.getId())).
                tokenValue(multipartSecurityToken.getTokenValue()).
                build();

        var requestHeader = Header.Builder.newInstance()
                .context(context)
                .id(header.getId())
                .type(header.getType())
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return new CreateProcessRequest(requestHeader, handlerRequest.getPayload());
    }

    public static ResponseBody getValidResponseBody(){
        return ResponseBody.create(
                MediaType.get("application/json; charset=utf-8"),
                "{}"
        );
    }

    public static HandlerRequest getValidHandlerRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidHeader(mapper))
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getValidHandlerCreateProcessRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidCreateProcessHeader(mapper))
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getInvalidTokenHandlerRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getInvalidTokenHeader(mapper))
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getInvalidHandlerRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getInvalidTypeHeader(mapper) )
                .payload(TEST_PAYLOAD).build();
    }

    public static AppSenderRequest getValidAppSenderRequest(ObjectMapper mapper){
        return new AppSenderRequest(TEST_BASE_URL+ "/headers/log/" + UUID.randomUUID(),
                JWT.create().toString(),
                getValidHandlerRequest(mapper)
        );
    }

    public static AppSenderRequest getInvalidUrlAppSenderRequest(ObjectMapper mapper){
        return new AppSenderRequest(UUID.randomUUID().toString(),
                JWT.create().toString(),
                getValidHandlerRequest(mapper)
        );
    }

    public static File getValidHeaderFile() {
        return getFile(VALID_HEADER_JSON).toFile();
    }
    public static File getInvalidHeaderFile() {
        return getFile(INVALID_HEADER_JSON).toFile();
    }
}
