package de.truzzt.clearinghouse.edc;

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
import org.eclipse.edc.spi.monitor.Monitor;
import org.eclipse.edc.spi.system.ServiceExtensionContext;

import java.io.File;
import java.io.IOException;
import java.util.UUID;

public class TestUtils {

    public static final String TEST_PAYLOAD = "Hello World";
    public static final String TEST_BASE_URL = "http://localhost:8000";

    public static final String LOG_MESSAGE_JSON_PATH = "src/test/java/de/truzzt/clearinghouse/edc/logMessage.json";

    public static Message getValidHeader() {
        try {
            ObjectMapper mapper = new ObjectMapper();

            File file = new File(LOG_MESSAGE_JSON_PATH);
            file.createNewFile();

            Message message = mapper.readValue(file, Message.class);

            return message;
        } catch (IOException ioe){
            ioe.printStackTrace();
            return null;
        }
    }

    public static Message getinvalidTokenHeader() {
        try {
            ObjectMapper mapper = new ObjectMapper();

            File file = new File(LOG_MESSAGE_JSON_PATH);
            file.createNewFile();

            Message message = mapper.readValue(file, Message.class);
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
        } catch (IOException ioe){
            ioe.printStackTrace();
            return null;
        }
    }

    public static Message getNotLogMessageValidHeader() {
        try {
            ObjectMapper mapper = new ObjectMapper();

            File file = new File(LOG_MESSAGE_JSON_PATH);
            file.createNewFile();

            Message message = mapper.readValue(file, Message.class);
            message.setType("ids:otherMessage");
            return message;
        } catch (IOException ioe){
            ioe.printStackTrace();
            return null;
        }
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


    public static HandlerRequest getValidHandlerRequest(){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidHeader() )
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getInvalidTokenHandlerRequest(){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getinvalidTokenHeader())
                .payload(TEST_PAYLOAD).build();
    }

    public static HandlerRequest getInvalidHandlerRequest(){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getNotLogMessageValidHeader() )
                .payload(TEST_PAYLOAD).build();
    }

    public static AppSenderRequest getValidAppSenderRequest(){
        return new AppSenderRequest(TEST_BASE_URL+"/messages/log/" + UUID.randomUUID(),
                JWT.create().toString(),
                getValidHandlerRequest()
        );
    }

    public static AppSenderRequest getInvalidUrlAppSenderRequest(){
        return new AppSenderRequest("" + UUID.randomUUID(),
                JWT.create().toString(),
                getValidHandlerRequest()
        );
    }

    public static String getBuildJwtToken(Monitor monitor,
                                          IdsId connectorId,
                                          TypeManagerUtil typeManagerUtil,
                                          AppSender appSender,
                                          ServiceExtensionContext context,
                                          HandlerRequest handlerRequest){

        LogMessageHandler handler = new LogMessageHandler(monitor, connectorId, typeManagerUtil, appSender,context);
        return handler.buildJWTToken(handlerRequest.getHeader().getSecurityToken(), context);
    }


}
