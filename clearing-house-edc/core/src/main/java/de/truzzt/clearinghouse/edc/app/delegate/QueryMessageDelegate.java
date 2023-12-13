package de.truzzt.clearinghouse.edc.app.delegate;

import de.truzzt.clearinghouse.edc.dto.*;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Context;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import de.truzzt.clearinghouse.edc.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.types.clearinghouse.TokenFormat;
import okhttp3.ResponseBody;

public class QueryMessageDelegate implements AppSenderDelegate<QueryMessageResponse> {
    private final TypeManagerUtil typeManagerUtil;

    public QueryMessageDelegate(TypeManagerUtil typeManagerUtil) {
        this.typeManagerUtil = typeManagerUtil;
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {
        return baseUrl + "/messages/query/" + handlerRequest.getPid();
    }

    public QueryMessageRequest buildRequestBody(HandlerRequest handlerRequest) {
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

        return new QueryMessageRequest(requestHeader);
    }

    @Override
    public QueryMessageResponse parseResponseBody(ResponseBody responseBody) {
        return typeManagerUtil.parse(responseBody.byteStream(), QueryMessageResponse.class);
    }
}
