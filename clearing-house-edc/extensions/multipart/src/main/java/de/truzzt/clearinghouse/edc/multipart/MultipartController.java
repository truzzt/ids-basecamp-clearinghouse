package de.truzzt.clearinghouse.edc.multipart;

import de.fraunhofer.iais.eis.DynamicAttributeTokenBuilder;
import de.truzzt.clearinghouse.edc.handler.Handler;
import de.truzzt.clearinghouse.edc.dto.HandlerRequest;
import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.TypeManagerUtil;
import de.truzzt.clearinghouse.edc.types.ids.Message;

import de.truzzt.clearinghouse.edc.types.ids.SecurityToken;
import de.truzzt.clearinghouse.edc.types.ids.TokenFormat;
import jakarta.ws.rs.Consumes;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.PathParam;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

import jakarta.ws.rs.core.Response;
import org.eclipse.edc.protocol.ids.spi.service.DynamicAttributeTokenService;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.monitor.Monitor;
import org.glassfish.jersey.media.multipart.FormDataBodyPart;
import org.glassfish.jersey.media.multipart.FormDataParam;
import org.glassfish.jersey.media.multipart.FormDataMultiPart;
import org.jetbrains.annotations.NotNull;

import java.io.InputStream;
import java.util.List;

import static de.truzzt.clearinghouse.edc.util.ResponseUtil.internalRecipientError;
import static de.truzzt.clearinghouse.edc.util.ResponseUtil.malformedMessage;
import static de.truzzt.clearinghouse.edc.util.ResponseUtil.messageTypeNotSupported;
import static de.truzzt.clearinghouse.edc.util.ResponseUtil.notAuthenticated;
import static java.lang.String.format;

@Consumes({MediaType.MULTIPART_FORM_DATA})
@Produces({MediaType.MULTIPART_FORM_DATA})
@Path("/")
public class MultipartController {

    private static final String HEADER = "header";
    private static final String PAYLOAD = "payload";
    private static final String PID = "pid";
    private static final String LOG_ID = "InfrastructureController";

    private final Monitor monitor;
    private final IdsId connectorId;
    private final TypeManagerUtil typeManagerUtil;
    private final DynamicAttributeTokenService tokenService;
    private final String idsWebhookAddress;
    private final List<Handler> multipartHandlers;

    public MultipartController(@NotNull Monitor monitor,
                               @NotNull IdsId connectorId,
                               @NotNull TypeManagerUtil typeManagerUtil,
                               @NotNull DynamicAttributeTokenService tokenService,
                               @NotNull String idsWebhookAddress,
                               @NotNull List<Handler> multipartHandlers) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.typeManagerUtil = typeManagerUtil;
        this.tokenService = tokenService;
        this.idsWebhookAddress = idsWebhookAddress;
        this.multipartHandlers = multipartHandlers;
    }

    @POST
    @Path("messages/log/{pid}")
    public Response request(@PathParam(PID) String pid,
                            @FormDataParam(HEADER) InputStream headerInputStream,
                            @FormDataParam(PAYLOAD) String payload) {

        // Check if header is missing
        if (headerInputStream == null) {
            monitor.severe(LOG_ID + ": Header is missing");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Convert header to message
        Message header;
        try {
            header = typeManagerUtil.parse(headerInputStream, Message.class);
        } catch (Exception e) {
            monitor.severe(format(LOG_ID + ": Header parsing failed: %s", e.getMessage()));
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Check if any required header field missing
        if (header.getId() == null
                || header.getType() == null
                || header.getModelVersion() == null
                || header.getIssued() == null
                || header.getIssuerConnector() == null
                || header.getSenderAgent() == null) {
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(header, connectorId)))
                    .build();
        }

        // Check if security token is present
        var securityToken = header.getSecurityToken();
        if (securityToken == null || securityToken.getTokenValue() == null) {
            monitor.severe(LOG_ID + ": Token is missing in header");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(notAuthenticated(header, connectorId)))
                    .build();
        }

        // Check the security token type
        var tokenFormat = securityToken.getTokenFormat().getId().toString();
        if (!tokenFormat.equals(TokenFormat.JWT_TOKEN_FORMAT)) {
            monitor.severe(LOG_ID + ": Invalid security token type: " + tokenFormat);
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Check if payload is missing
        if (payload == null) {
            monitor.severe(LOG_ID + ": Payload is missing");
            return Response.status(Response.Status.BAD_REQUEST)
                    .entity(createFormDataMultiPart(malformedMessage(null, connectorId)))
                    .build();
        }

        // Validate DAT
        if (!validateToken(header)) {
            return Response.status(Response.Status.FORBIDDEN)
                    .entity(createFormDataMultiPart(notAuthenticated(header, connectorId)))
                    .build();
        }

        // Build the multipart request
        var multipartRequest = HandlerRequest.Builder.newInstance()
                .pid(pid)
                .header(header)
                .payload(payload)
                .build();

        // Send to handler processing
        HandlerResponse handlerResponse;
        try {
            handlerResponse = multipartHandlers.stream()
                    .filter(h -> h.canHandle(multipartRequest))
                    .findFirst()
                    .map(it -> it.handleRequest(multipartRequest))
                    .orElseGet(() -> HandlerResponse.Builder.newInstance()
                            .header(messageTypeNotSupported(header, connectorId))
                            .build());
        } catch (Exception e) {
            monitor.severe(LOG_ID + ": Error in message handler processing", e);
            return Response.status(Response.Status.INTERNAL_SERVER_ERROR)
                    .entity(createFormDataMultiPart(internalRecipientError(header, connectorId)))
                    .build();
        }

        // Get the response token
        if (!getResponseToken(header, handlerResponse)) {
            return Response.status(Response.Status.INTERNAL_SERVER_ERROR)
                    .entity(createFormDataMultiPart(internalRecipientError(header, connectorId)))
                    .build();
        }

        // Build the response
        return Response.status(Response.Status.CREATED)
                .entity(createFormDataMultiPart(handlerResponse.getHeader(), handlerResponse.getPayload()))
                .build();
    }

    private boolean validateToken(Message header) {

        var dynamicAttributeToken = new DynamicAttributeTokenBuilder().
                _tokenValue_(header.getSecurityToken().getTokenValue()).
                _tokenFormat_(de.fraunhofer.iais.eis.TokenFormat.JWT)
                .build();

        var verificationResult = tokenService
                .verifyDynamicAttributeToken(dynamicAttributeToken, header.getIssuerConnector(), idsWebhookAddress);

        if (verificationResult.failed()) {
            monitor.warning(format("MultipartController: Token validation failed %s", verificationResult.getFailure().getMessages()));
            return false;
        } else {
            return true;
        }
    }

    private boolean getResponseToken(Message header, HandlerResponse handlerResponse) {

        handlerResponse.getHeader().setSecurityToken(header.getSecurityToken());
        return true;

        /*if ((header.getRecipientConnector() == null) || (header.getRecipientConnector().isEmpty())) {
            monitor.severe(LOG_ID + ": Recipient connector is missing");
            return false;
        }

        var recipient = header.getRecipientConnector().get(0);
        var tokenResult = tokenService.obtainDynamicAttributeToken(recipient.toString());

        if (tokenResult.succeeded()) {
            var responseToken = tokenResult.getContent();
            SecurityToken securityToken = new SecurityToken();
            securityToken.setType(header.getSecurityToken().getType());
            securityToken.setTokenFormat(header.getSecurityToken().getTokenFormat());
            securityToken.setTokenValue(responseToken.getTokenValue());

            handlerResponse.getHeader().setSecurityToken(securityToken);
            return true;

        } else {
            monitor.severe(LOG_ID + ": Failed to get response token: " + tokenResult.getFailureDetail());
            return false;
        }*/
    }

    private FormDataMultiPart createFormDataMultiPart(Message header, Object payload) {
        var multiPart = createFormDataMultiPart(header);

        if (payload != null) {
            multiPart.bodyPart(new FormDataBodyPart(PAYLOAD, typeManagerUtil.toJson(payload), MediaType.APPLICATION_JSON_TYPE));
        }

        return multiPart;
    }

    private FormDataMultiPart createFormDataMultiPart(Message header) {
        var multiPart = new FormDataMultiPart();
        if (header != null) {
            multiPart.bodyPart(new FormDataBodyPart(HEADER, typeManagerUtil.toJson(header), MediaType.APPLICATION_JSON_TYPE));
        }
        return multiPart;
    }

}
