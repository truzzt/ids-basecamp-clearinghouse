package de.truzzt.clearinghouse.edc.app.delegate;

import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.QueryMessageRequest;
import de.truzzt.clearinghouse.edc.dto.QueryMessageResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Context;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import de.truzzt.clearinghouse.edc.types.clearinghouse.SecurityToken;
import de.truzzt.clearinghouse.edc.types.clearinghouse.TokenFormat;
import okhttp3.ResponseBody;

import java.time.format.DateTimeFormatter;

public class QueryMessageDelegate implements AppSenderDelegate<QueryMessageResponse> {
    private final TypeManagerUtil typeManagerUtil;

    private final DateTimeFormatter dateFormat = DateTimeFormatter.ofPattern("yyyy-MM-dd");

    public QueryMessageDelegate(TypeManagerUtil typeManagerUtil) {
        this.typeManagerUtil = typeManagerUtil;
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {

        String queryParameters = "";
        if (handlerRequest.getPagging() != null) {
            var pagging = handlerRequest.getPagging();

            if (pagging.getPage() != null) {
                queryParameters += "?page=" + pagging.getSize();
            }

            if (pagging.getSize() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?size=" + pagging.getSize();
                else
                    queryParameters += "&size=" + pagging.getSize();
            }

            if (pagging.getSort() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?sort=" + pagging.getSort().toString().toLowerCase();
                else
                    queryParameters += "&sort=" + pagging.getSort().toString().toLowerCase();
            }

            if (pagging.getDateFrom() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?dateFrom=" + dateFormat.format(pagging.getDateFrom());
                else
                    queryParameters += "&dateFrom=" + dateFormat.format(pagging.getDateFrom());
            }

            if (pagging.getDateTo() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?dateTo=" + dateFormat.format(pagging.getDateTo());
                else
                    queryParameters += "&dateTo=" + dateFormat.format(pagging.getDateTo());
            }
        }

        return baseUrl + "/messages/query/" + handlerRequest.getPid() + queryParameters;
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
