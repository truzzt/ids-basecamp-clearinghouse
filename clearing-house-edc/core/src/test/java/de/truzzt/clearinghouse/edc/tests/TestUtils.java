package de.truzzt.clearinghouse.edc.tests;

import com.auth0.jwt.JWT;
import com.fasterxml.jackson.databind.ObjectMapper;
import de.fraunhofer.iais.eis.Message;
import de.truzzt.clearinghouse.edc.app.message.*;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import de.truzzt.clearinghouse.edc.types.Paging;
import de.truzzt.clearinghouse.edc.app.types.Header;
import de.truzzt.clearinghouse.edc.app.types.SecurityToken;
import okhttp3.*;
import org.eclipse.edc.spi.EdcException;

import java.io.File;
import java.io.IOException;
import java.time.LocalDate;
import java.util.UUID;

public class TestUtils extends BaseTestUtils {

    public static final String TEST_BASE_URL = "http://localhost:8000";

    private static final String TEST_PAYLOAD = "Hello World";
    private static final String TEST_CREATE_PROCCESS_PAYLOAD = "{ \"owners\": [\"1\", \"2\"]}";
    private static final String VALID_HEADER_JSON = "headers/valid-header.json";
    private static final String VALID_QUERY_MESSAGE_HEADER_JSON = "headers/valid-query-message-header.json";
    private static final String VALID_CREATE_PROCESS_HEADER_JSON = "headers/valid-create-process-header.json";
    private static final String INVALID_HEADER_JSON = "headers/invalid-header.json";
    private static final String INVALID_TYPE_HEADER_JSON = "headers/invalid-type.json";
    private static final String INVALID_TOKEN_HEADER_JSON = "headers/invalid-token.json";

    public static Message getValidHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_HEADER_JSON);
    }

    public static Message getValidQueryMessageHeader(ObjectMapper mapper) {
        return parseFile(mapper, Message.class, VALID_QUERY_MESSAGE_HEADER_JSON);
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

    public static Paging getValidpaging(){
        return Paging.Builder.newInstance()
                .page(1)
                .size(1)
                .sort(Paging.Sort.ASC)
                .dateFrom(LocalDate.now())
                .dateTo(LocalDate.now())
                .build();
    }

    public static Paging getValidpagingOnlySize(){
        return Paging.Builder.newInstance()
                .size(1)
                .build();
    }
    public static Paging getValidpagingOnlySort(){
        return Paging.Builder.newInstance()
                .sort(Paging.Sort.ASC)
                .build();
    }

    public static Paging getValidpagingOnlyDateFrom(){
        return Paging.Builder.newInstance()
                .dateFrom(LocalDate.now())
                .build();
    }

    public static Paging getValidpagingOnlyDateTo(){
        return Paging.Builder.newInstance()
                .dateTo(LocalDate.now())
                .build();
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

    public static QueryMessageResponse getValidQueryMessageResponse(String url, ObjectMapper mapper) {
        try {
            return mapper.readValue(getValidResponse(url).body().byteStream(), QueryMessageResponse.class);

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

        var requestHeader = createRequestHeader(handlerRequest);

        return new LoggingMessageRequest(requestHeader, handlerRequest.getPayload());
    }

    public static QueryMessageRequest getValidQueryMessageRequest(HandlerRequest handlerRequest) {

        var requestHeader = createRequestHeader(handlerRequest);

        return new QueryMessageRequest(requestHeader);
    }

    public static CreateProcessRequest getValidCreateProcessRequest(HandlerRequest handlerRequest) {

        var requestHeader = createRequestHeader(handlerRequest);

        return new CreateProcessRequest(requestHeader, handlerRequest.getPayload());
    }

    private static Header createRequestHeader(HandlerRequest handlerRequest) {
        var header = handlerRequest.getHeader();


        var multipartSecurityToken = header.getSecurityToken();
        var securityToken = SecurityToken.Builder.newInstance().
                type(multipartSecurityToken).
                id(multipartSecurityToken.getId()).
                tokenValue(multipartSecurityToken.getTokenValue()).
                build();

        var requestHeader = Header.Builder.newInstance()
                .id(header.getId())
                .type(header)
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return requestHeader;
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

    public static HandlerRequest getValidHandlerQueryMessageRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidQueryMessageHeader(mapper))
                .payload(TEST_PAYLOAD)
                .paging(getValidpaging()).build();
    }

    public static HandlerRequest getValidHandlerQueryMessageOnlySizeRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidQueryMessageHeader(mapper))
                .payload(TEST_PAYLOAD)
                .paging(getValidpagingOnlySize()).build();
    }

    public static HandlerRequest getValidHandlerQueryMessageOnlySortRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidQueryMessageHeader(mapper))
                .payload(TEST_PAYLOAD)
                .paging(getValidpagingOnlySort()).build();
    }

    public static HandlerRequest getValidHandlerQueryMessageOnlyDateFromRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidQueryMessageHeader(mapper))
                .payload(TEST_PAYLOAD)
                .paging(getValidpagingOnlyDateFrom()).build();
    }

    public static HandlerRequest getValidHandlerQueryMessageOnlyDateToRequest(ObjectMapper mapper){
        return HandlerRequest.Builder.newInstance()
                .pid(UUID.randomUUID().toString())
                .header(getValidQueryMessageHeader(mapper))
                .payload(TEST_PAYLOAD)
                .paging(getValidpagingOnlyDateTo()).build();
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

    public static AppSenderRequest getValidQueryAppSenderRequest(ObjectMapper mapper){
        return new AppSenderRequest(TEST_BASE_URL+ "/headers/query/" + UUID.randomUUID(),
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
