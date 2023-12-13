package de.truzzt.clearinghouse.edc.dto;

import com.fasterxml.jackson.annotation.JsonProperty;
import de.truzzt.clearinghouse.edc.types.clearinghouse.Header;
import org.jetbrains.annotations.NotNull;

public class QueryMessageRequest {
    @JsonProperty("header")
    @NotNull
    private Header header;

    public QueryMessageRequest(@NotNull Header header) {
        this.header = header;
    }
}
