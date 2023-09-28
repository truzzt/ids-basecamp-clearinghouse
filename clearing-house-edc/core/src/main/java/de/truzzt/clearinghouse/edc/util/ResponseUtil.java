package de.truzzt.clearinghouse.edc.util;

import de.truzzt.clearinghouse.edc.dto.HandlerResponse;
import de.truzzt.clearinghouse.edc.types.ids.Message;
import de.truzzt.clearinghouse.edc.types.ids.RejectionMessage;
import de.truzzt.clearinghouse.edc.types.ids.RejectionReason;
import org.eclipse.edc.protocol.ids.spi.domain.IdsConstants;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.protocol.ids.spi.types.IdsType;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import javax.xml.datatype.DatatypeConfigurationException;
import javax.xml.datatype.DatatypeFactory;
import javax.xml.datatype.XMLGregorianCalendar;
import java.net.URI;
import java.time.ZonedDateTime;
import java.util.GregorianCalendar;
import java.util.UUID;

public class ResponseUtil {

    private static final String PROCESSED_NOTIFICATION_TYPE = "ids:MessageProcessedNotificationMessage";

    public static HandlerResponse createMultipartResponse(@NotNull Message header, @NotNull Object payload) {
        return HandlerResponse.Builder.newInstance()
                .header(header)
                .payload(payload)
                .build();
    }

    public static Message messageProcessedNotification(@NotNull Message correlationMessage,
                                                       @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        Message message =  new Message(messageId);
        message.setContext(correlationMessage.getContext());
        message.setType(PROCESSED_NOTIFICATION_TYPE);
        message.setSecurityToken(correlationMessage.getSecurityToken());
        message.setIssuerConnector(connectorId.toUri());
        message.setModelVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        message.setIssued(gregorianNow());
        message.setSenderAgent(connectorId.toUri());

        return message;
    }

    @NotNull
    public static RejectionMessage notAuthenticated(@NotNull Message correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.NOT_AUTHENTICATED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage malformedMessage(@Nullable Message correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MALFORMED_MESSAGE);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage messageTypeNotSupported(@NotNull Message correlationMessage,
                                                           @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MESSAGE_TYPE_NOT_SUPPORTED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage badParameters(@NotNull Message correlationMessage,
                                                 @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage =  createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.BAD_PARAMETERS);

        return rejectionMessage;
    }

    @NotNull
    private static RejectionMessage createRejectionMessage(@Nullable Message correlationMessage,
                                                           @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        var rejectionMessage = new RejectionMessage(messageId);
        rejectionMessage.setModelVersion(IdsConstants.INFORMATION_MODEL_VERSION);
        rejectionMessage.setIssued(gregorianNow());
        rejectionMessage.setIssuerConnector(connectorId.toUri());
        rejectionMessage.setSenderAgent(connectorId.toUri());

        return rejectionMessage;
    }

    private static URI getMessageId() {
        return IdsId.Builder.newInstance().value(UUID.randomUUID().toString()).type(IdsType.MESSAGE).build().toUri();
    }

    public static XMLGregorianCalendar gregorianNow() {
        try {
            GregorianCalendar gregorianCalendar = GregorianCalendar.from(ZonedDateTime.now());
            return DatatypeFactory.newInstance().newXMLGregorianCalendar(gregorianCalendar);
        } catch (DatatypeConfigurationException e) {
            throw new RuntimeException(e);
        }
    }
}
