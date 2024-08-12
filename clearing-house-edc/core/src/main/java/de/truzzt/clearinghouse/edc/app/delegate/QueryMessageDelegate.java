package de.truzzt.clearinghouse.edc.app.delegate;

import com.fasterxml.jackson.databind.ObjectMapper;
import de.truzzt.clearinghouse.edc.app.message.LoggingMessageResponse;
import de.truzzt.clearinghouse.edc.types.HandlerRequest;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageRequest;
import de.truzzt.clearinghouse.edc.app.message.QueryMessageResponse;
import de.truzzt.clearinghouse.edc.app.types.Header;
import de.truzzt.clearinghouse.edc.app.types.SecurityToken;
import okhttp3.ResponseBody;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;

import java.io.IOException;
import java.time.format.DateTimeFormatter;

public class QueryMessageDelegate extends AppSenderDelegate<QueryMessageResponse> {

    private final Monitor monitor;

    private final DateTimeFormatter dateFormat = DateTimeFormatter.ofPattern("yyyy-MM-dd");

    public QueryMessageDelegate(Monitor monitor, ObjectMapper objectMapper) {
        super(objectMapper);
        this.monitor = monitor;
    }

    public String buildRequestUrl(String baseUrl, HandlerRequest handlerRequest) {

        String queryParameters = "";
        if (handlerRequest.getPaging() != null) {
            var paging = handlerRequest.getPaging();

            if (paging.getPage() != null) {
                queryParameters += "?page=" + paging.getSize();
            }

            if (paging.getSize() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?size=" + paging.getSize();
                else
                    queryParameters += "&size=" + paging.getSize();
            }

            if (paging.getSort() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?sort=" + paging.getSort().toString().toLowerCase();
                else
                    queryParameters += "&sort=" + paging.getSort().toString().toLowerCase();
            }

            if (paging.getDateFrom() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?dateFrom=" + dateFormat.format(paging.getDateFrom());
                else
                    queryParameters += "&dateFrom=" + dateFormat.format(paging.getDateFrom());
            }

            if (paging.getDateTo() != null) {
                if (queryParameters.isEmpty())
                    queryParameters += "?dateTo=" + dateFormat.format(paging.getDateTo());
                else
                    queryParameters += "&dateTo=" + dateFormat.format(paging.getDateTo());
            }
        }

        return baseUrl + "/messages/query/" + handlerRequest.getPid() + queryParameters;
    }

    public QueryMessageRequest buildRequestBody(HandlerRequest handlerRequest) {
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

        return new QueryMessageRequest(requestHeader);
    }

    @Override
    public QueryMessageResponse buildSuccessResponse(ResponseBody responseBody) {
        return parseSuccessResponse(responseBody, QueryMessageResponse.class);
    }

    @Override
    public QueryMessageResponse buildErrorResponse(int httpStatus) {
        return new QueryMessageResponse(httpStatus);
    }
}
