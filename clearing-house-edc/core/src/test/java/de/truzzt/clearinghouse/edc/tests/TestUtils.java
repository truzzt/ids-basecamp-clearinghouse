package de.truzzt.clearinghouse.edc.tests;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.AppSender;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.handler.LogMessageHandler;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
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
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;
import org.eclipse.edc.spi.system.ServiceExtensionContext;

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
    private static final String VALID_HEADER_JSON = "messages/valid-header.json";
    private static final String INVALID_LOG_MESSAGE_HEADER_JSON = "messages/invalid-log-message-header.json";

    private static <T> T readJsonFile(ObjectMapper mapper, Class<T> type, String path) {

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

    private static File returnJonFile(String path) {

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

        return filePath.toFile();
    }

    public static Message getValidHeader(ObjectMapper mapper) {
        return readJsonFile(mapper, Message.class, VALID_HEADER_JSON);
    }

    public static Message getInvalidTokenHeader(ObjectMapper mapper) {

            Message message = readJsonFile(mapper, Message.class, VALID_HEADER_JSON);

            message.getSecurityToken().setTokenValue("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzY29wZXMiOlsiaWRzYz" +
                    "pJRFNfQ09OTkVDVE9SX0FUVFJJQlVURVNfQUxMIl0sImF1ZCI6Imlkc2M6SURTX0NPTk5FQ1RPUlNfQUxMIiwiaXNzIjo" +
                    "iaHR0cHM6Ly9kYXBzLmFpc2VjLmZyYXVuaG9mZXIuZGUiLCJuYmYiOjE2MzQ2NTA3MzksImlhdCI6MTYzNDY1MDczOSw" +
                    "ianRpIjoiTVRneE9EUXdPVFF6TXpZd05qWXlOVFExTUE9PSIsImV4cCI6MTYzNDY1NDMzOSwic2VjdXJpdHlQcm9maWx" +
                    "lIjoiaWRzYzpCQVNFX1NFQ1VSSVRZX1BST0ZJTEUiLCJyZWZlcnJpbmdDb25uZWN0b3IiOiJodHRwOi8vYnJva2VyLml" +
                    "kcy5pc3N0LmZyYXVuaG9mZXIuZGUuZGVtbyIsIkB0eXBlIjoiaWRzOkRhdFBheWxvYWQiLCJAY29udGV4dCI6Imh0dHB" +
                    "zOi8vdzNpZC5vcmcvaWRzYS9jb250ZXh0cy9jb250ZXh0Lmpzb25sZCIsInRyYW5zcG9ydENlcnRzU2hhMjU2IjoiOTc" +
                    "0ZTYzMjRmMTJmMTA5MTZmNDZiZmRlYjE4YjhkZDZkYTc4Y2M2YTZhMDU2NjAzMWZhNWYxYTM5ZWM4ZTYwMCJ9.hekZoP" +
                    "DjEWaXreQl3l0PUIjBOPQhAl0w2mH4_PdNWuA");
            return message;
    }

    public static Message getNotLogMessageValidHeader(ObjectMapper mapper) {

        Message message = readJsonFile(mapper, Message.class, VALID_HEADER_JSON);

        message.setType("ids:otherMessage");
        return message;
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

        } catch (IOException ioe) {
            ioe.printStackTrace();
            return null;
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
        return new AppSenderRequest(TEST_BASE_URL+"/messages/log/" + UUID.randomUUID(),
                JWT.create().toString(),
                getValidHandlerRequest(mapper)
        );
    }

    public static AppSenderRequest getInvalidUrlAppSenderRequest(ObjectMapper mapper){
        return new AppSenderRequest("" + UUID.randomUUID(),
                JWT.create().toString(),
                getValidHandlerRequest(mapper)
        );
    }

    public static File getValidHeaderFile() {

        return returnJonFile(VALID_HEADER_JSON);
    }

    public static File getInvalidHeaderFile() {

        return returnJonFile(INVALID_LOG_MESSAGE_HEADER_JSON);
    }


}
