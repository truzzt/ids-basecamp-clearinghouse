package de.truzzt.clearinghouse.edc.multipart.util;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.types.ids.LogMessage;
import de.truzzt.clearinghouse.edc.multipart.types.ids.RejectionMessage;
import de.truzzt.clearinghouse.edc.multipart.types.ids.RejectionReason;
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

import static org.eclipse.edc.protocol.ids.util.CalendarUtil.gregorianNow;

public class ResponseUtil {

    private static final String PROCESSED_NOTIFICATION_TYPE = "ids:MessageProcessedNotificationMessage";

    public static MultipartResponse createMultipartResponse(@NotNull LogMessage header) {
        return MultipartResponse.Builder.newInstance()
                .header(header)
                .build();
    }

    public static LogMessage messageProcessedNotification(@NotNull LogMessage correlationMessage,
                                                       @NotNull IdsId connectorId) {
        var messageId = getMessageId();

        LogMessage message =  new LogMessage(messageId);
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
    public static RejectionMessage notAuthenticated(@NotNull LogMessage correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.NOT_AUTHENTICATED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage malformedMessage(@Nullable LogMessage correlationMessage,
                                                    @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MALFORMED_MESSAGE);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage messageTypeNotSupported(@NotNull LogMessage correlationMessage,
                                                           @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage = createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.MESSAGE_TYPE_NOT_SUPPORTED);

        return rejectionMessage;
    }

    @NotNull
    public static RejectionMessage badParameters(@NotNull LogMessage correlationMessage,
                                                 @NotNull IdsId connectorId) {
        RejectionMessage rejectionMessage =  createRejectionMessage(correlationMessage, connectorId);
        rejectionMessage.setRejectionReason(RejectionReason.BAD_PARAMETERS);

        return rejectionMessage;
    }

    @NotNull
    private static RejectionMessage createRejectionMessage(@Nullable LogMessage correlationMessage,
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
