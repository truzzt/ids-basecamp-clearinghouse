package de.truzzt.clearinghouse.edc.multipart.controller;

import de.truzzt.clearinghouse.edc.multipart.handler.Handler;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.multipart.types.ids.LogMessage;
import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.iam.ClaimToken;
import org.eclipse.edc.spi.monitor.Monitor;
import org.glassfish.jersey.media.multipart.FormDataBodyPart;
import org.glassfish.jersey.media.multipart.FormDataParam;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.jetbrains.annotations.NotNull;

import java.io.InputStream;
import java.util.List;

import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.malformedMessage;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.messageTypeNotSupported;
import static de.truzzt.clearinghouse.edc.multipart.util.ResponseUtil.notAuthenticated;
import static java.lang.String.format;

@Consumes({MediaType.MULTIPART_FORM_DATA})
@Produces({MediaType.MULTIPART_FORM_DATA})
@Path("/")
public class MultipartController {

    private static final String HEADER = "header";
    private static final String PAYLOAD = "payload";

    private final Monitor monitor;
    private final IdsId connectorId;

    private final List<Handler> multipartHandlers;

    private final TypeManagerUtil typeManagerUtil;

    public MultipartController(@NotNull Monitor monitor,
                               @NotNull IdsId connectorId,
                               @NotNull TypeManagerUtil typeManagerUtil,
                               @NotNull List<Handler> multipartHandlers) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
        this.multipartHandlers = multipartHandlers;
    }

    @POST
    @Path("log")
    public FormDataMultiPart request(@FormDataParam(HEADER) InputStream headerInputStream,
                                     @FormDataParam(PAYLOAD) String payload) {

        if (headerInputStream == null) {
            return createFormDataMultiPart(malformedMessage(null, connectorId));
        }

        LogMessage header;
        try {
            header = typeManagerUtil.parseMessage(headerInputStream);
        } catch (Exception e) {
            monitor.warning(format("InfrastructureController: Header parsing failed: %s", e.getMessage()));
            return createFormDataMultiPart(malformedMessage(null, connectorId));
        }

        if (header == null) {
            return createFormDataMultiPart(malformedMessage(null, connectorId));
        }

        // Check if any required header field missing
        if (header.getId() == null || header.getIssuerConnector() == null || header.getSenderAgent() == null) {
            return createFormDataMultiPart(malformedMessage(header, connectorId));
        }

        // Check if DAT present
        var dynamicAttributeToken = header.getSecurityToken();
        if (dynamicAttributeToken == null || dynamicAttributeToken.getTokenValue() == null) {
            monitor.warning("InfrastructureController: Token is missing in header");
            return createFormDataMultiPart(notAuthenticated(header, connectorId));
        }

        // Build the multipart request
        var emptyClaimToken = ClaimToken.Builder.newInstance().build();
        var multipartRequest = MultipartRequest.Builder.newInstance()
                .header(header)
                .payload(payload)
                .claimToken(emptyClaimToken)
                .build();

        var multipartResponse = multipartHandlers.stream()
                .filter(h -> h.canHandle(multipartRequest))
                .findFirst()
                .map(it -> it.handleRequest(multipartRequest))
                .orElseGet(() -> MultipartResponse.Builder.newInstance()
                        .header(messageTypeNotSupported(header, connectorId))
                        .build());

        return createFormDataMultiPart(multipartResponse.getHeader(), multipartResponse.getPayload());
    }

    private FormDataMultiPart createFormDataMultiPart(LogMessage header, Object payload) {
        var multiPart = createFormDataMultiPart(header);

        if (payload != null) {
            multiPart.bodyPart(new FormDataBodyPart(PAYLOAD, typeManagerUtil.toJson(payload), MediaType.APPLICATION_JSON_TYPE));
        }

        return multiPart;
    }

    private FormDataMultiPart createFormDataMultiPart(LogMessage header) {
        var multiPart = new FormDataMultiPart();
        if (header != null) {
            multiPart.bodyPart(new FormDataBodyPart(HEADER, typeManagerUtil.toJson(header), MediaType.APPLICATION_JSON_TYPE));
        }
        return multiPart;
    }

}
