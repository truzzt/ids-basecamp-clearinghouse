package de.truzzt.clearinghouse.edc.app.message;

import com.fasterxml.jackson.annotation.JsonProperty;
import de.fraunhofer.iais.eis.LogMessage;
import org.jetbrains.annotations.NotNull;

import java.util.List;

public class QueryMessageResponse extends AbstractResponse {

    @JsonProperty("date_from")
    @NotNull
    private String dateFrom;

    @JsonProperty("date_to")
    @NotNull
    private String dateTo;

    @JsonProperty("page")
    @NotNull
    private Integer page;

    @JsonProperty("size")
    @NotNull
    private Integer size;

    @JsonProperty("order")
    @NotNull
    private String order;

    @JsonProperty("documents")
    @NotNull
    private List<LogMessage> documents;

    public QueryMessageResponse() {
    }
    public QueryMessageResponse(int httpStatus) {
        super(httpStatus);
    }

    public String getDateFrom() {
        return dateFrom;
    }

    public String getDateTo() {
        return dateTo;
    }

    public Integer getPage() {
        return page;
    }

    public Integer getSize() {
        return size;
    }

    public String getOrder() {
        return order;
    }

    public List<LogMessage> getDocuments() {
        return documents;
    }
}
