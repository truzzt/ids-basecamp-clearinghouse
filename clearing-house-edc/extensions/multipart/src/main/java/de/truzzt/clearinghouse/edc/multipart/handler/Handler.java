package de.truzzt.clearinghouse.edc.multipart.handler;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import org.jetbrains.annotations.NotNull;

public interface Handler {

    boolean canHandle(@NotNull MultipartRequest multipartRequest);

    @NotNull MultipartResponse handleRequest(@NotNull MultipartRequest multipartRequest);
}
