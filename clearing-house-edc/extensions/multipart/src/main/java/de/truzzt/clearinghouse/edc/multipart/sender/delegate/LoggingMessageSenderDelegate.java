package de.truzzt.clearinghouse.edc.multipart.sender.delegate;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.Context;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.LoggingMessageRequest;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.RequestHeader;
import de.truzzt.clearinghouse.edc.multipart.types.clearinghouse.TokenFormat;
import okhttp3.ResponseBody;

public class LoggingMessageSenderDelegate implements SenderDelegate<LoggingMessageRequest, LoggingMessageResponse> {

    private final TypeManagerUtil typeManagerUtil;

    public LoggingMessageSenderDelegate(TypeManagerUtil typeManagerUtil) {
        this.typeManagerUtil = typeManagerUtil;
    }

    @Override
    public String buildRequestUrl(String baseUrl, MultipartRequest multipartRequest) {
        return baseUrl + "/messages/log/" + multipartRequest.getPid();
    }

    @Override
    public LoggingMessageRequest buildRequestBody(MultipartRequest multipartRequest) {
        var header = multipartRequest.getHeader();

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

        var requestHeader = RequestHeader.Builder.newInstance()
                .context(context)
                .id(header.getId())
                .type(header.getType())
                .securityToken(securityToken)
                .issuerConnector(header.getIssuerConnector())
                .modelVersion(header.getModelVersion())
                .issued(header.getIssued())
                .senderAgent(header.getSenderAgent())
                .build();

        return new LoggingMessageRequest(requestHeader, multipartRequest.getPayload());
    }

    @Override
    public LoggingMessageResponse parseResponseBody(ResponseBody responseBody) {
        return typeManagerUtil.parse(responseBody.byteStream(), LoggingMessageResponse.class);
    }
}
