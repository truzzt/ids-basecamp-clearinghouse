package de.truzzt.clearinghouse.edc.tests;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Context;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import de.truzzt.clearinghouse.edc.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.types.clearinghouse.TokenFormat;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import okhttp3.Headers;
import okhttp3.MediaType;
import okhttp3.Protocol;
import okhttp3.Request;
import okhttp3.Response;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;

import java.io.File;
import java.io.IOException;
import java.net.URI;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.UUID;

public class TestUtils {

    public static final String TEST_BASE_URL = "http://localhost:8000";

    private static final String TEST_PAYLOAD = "Hello World";
    public static final String VALID_HEADER_JSON = "headers/valid-header.json";
    public static final String INVALID_HEADER_JSON = "headers/invalid-header.json";
    public static final String INVALID_TYPE_HEADER_JSON = "headers/invalid-type.json";
    public static final String INVALID_TOKEN_HEADER_JSON = "headers/invalid-token.json";
    public static final String MISSING_FIELDS_HEADER_JSON = "headers/missing-fields.json";
    public static final String MISSING_TOKEN_HEADER_JSON = "headers/missing-token.json";
    public static final String VALID_RESPONSE_JSON = "headers/valid-response.json";

    private static <T> T parseFile(ObjectMapper mapper, Class<T> type, String path) {

        ClassLoader classLoader = TestUtils.class.getClassLoader();
        var jsonResource = classLoader.getResource(path);

        if (jsonResource == null) {
            throw new EdcException("Header json file not found: " + path);
        }

        URI jsonUrl;
        try {
            jsonUrl = jsonResource.toURI();
        } catch (URISyntaxException e) {
            throw new EdcException("Error finding json file on classpath", e);
        }

        Path filePath = Path.of(jsonUrl);
        if (!Files.exists(filePath)) {
            throw new EdcException("Header json file not found: " + path);
        }

        T object = null;
        try {
            var jsonContents = Files.readAllBytes(filePath);
            object = mapper.readValue(jsonContents, type);

        } catch (IOException e){
            throw new EdcException("Error parsing json file", e);
        }

        return object;
    }

    private static Path getFile(String path) {

        ClassLoader classLoader = TestUtils.class.getClassLoader();
        var jsonResource = classLoader.getResource(path);

        if (jsonResource == null) {
            throw new EdcException("Header json file not found: " + path);
        }

        URI jsonUrl;
        try {
            jsonUrl = jsonResource.toURI();
        } catch (URISyntaxException e) {
            throw new EdcException("Error finding json file on classpath", e);
        }

        Path filePath = Path.of(jsonUrl);
        if (!Files.exists(filePath)) {
            throw new EdcException("Header json file not found: " + path);
        }

        return filePath;
    }

    public static String readFile(String path) {
        var file = getFile(path);

        try {
            return Files.readString(file);
        } catch (IOException e) {
            throw new EdcException("Error reading file contents", e);
        }
    }

    public static Message getValidHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_HEADER_JSON);
    }

    public static Message getInvalidTokenHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, INVALID_TOKEN_HEADER_JSON);
    }

    public static Message getNotLogMessageValidHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, INVALID_TYPE_HEADER_JSON);
    }

    public static Message getValidResponseHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_RESPONSE_JSON);
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

    public static LoggingMessageResponse getValidLoggingMessageResponse(String url) {
        try {
            ObjectMapper mapper = new ObjectMapper();

            return mapper.readValue(getValidResponse(url).body().byteStream(), LoggingMessageResponse.class);

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

    public static HandlerRequest getInvalidTokenHandlerRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getInvalidTokenHeader(mapper))
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getInvalidHandlerRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getNotLogMessageValidHeader(mapper) )
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
