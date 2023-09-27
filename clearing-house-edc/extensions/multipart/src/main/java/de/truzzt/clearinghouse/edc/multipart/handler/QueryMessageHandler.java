package de.truzzt.clearinghouse.edc.multipart.handler;

import de.truzzt.clearinghouse.edc.multipart.message.MultipartRequest;
import de.truzzt.clearinghouse.edc.multipart.message.MultipartResponse;
import de.truzzt.clearinghouse.edc.multipart.sender.ClearingHouseAppSender;
import org.eclipse.edc.protocol.ids.spi.types.IdsId;
import org.eclipse.edc.spi.EdcException;
import org.eclipse.edc.spi.monitor.Monitor;
import org.jetbrains.annotations.NotNull;

public class QueryMessageHandler implements Handler {

    private final Monitor monitor;
    private final IdsId connectorId;
    private final ClearingHouseAppSender clearingHouseAppSender;

    public QueryMessageHandler(Monitor monitor,
                               IdsId connectorId,
                               ClearingHouseAppSender clearingHouseAppSender) {
        this.monitor = monitor;
        this.connectorId = connectorId;
        this.clearingHouseAppSender = clearingHouseAppSender;
    }

    @Override
    public boolean canHandle(@NotNull MultipartRequest multipartRequest) {
        return multipartRequest.getHeader().getType().equals("ids:QueryMessage");
    }

    @Override
    public @NotNull MultipartResponse handleRequest(@NotNull MultipartRequest multipartRequest) {
        throw new EdcException("Handler not implemented !");
    }
}
