package de.truzzt.clearinghouse.edc.app;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.dto.AppSenderRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import okhttp3.Headers;
import okhttp3.MediaType;
import okhttp3.Protocol;
import okhttp3.Request;
import okhttp3.Response;
import okhttp3.ResponseBody;

import java.io.File;
import java.io.IOException;
import java.util.UUID;

public class TestUtils {

    public static final String TEST_PAYLOAD = "Hello World";
    public static final String TEST_BASE_URL = "http://localhost:8000";

    public static Message getValidHeader() {
        try {
            ObjectMapper mapper = new ObjectMapper();

            File file = new File("src/test/java/de/truzzt/clearinghouse/edc/app/logMessage.json");
            file.createNewFile();

            Message message = mapper.readValue(file, Message.class);

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
}
