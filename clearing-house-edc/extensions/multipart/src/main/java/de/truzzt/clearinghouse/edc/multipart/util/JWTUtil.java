package de.truzzt.clearinghouse.edc.multipart.util;

import java.time.LocalDateTime;
import java.time.ZoneId;
import java.util.Date;

public class JWTUtil {

    public static Date convertLocalDateTime(LocalDateTime localDateTime) {
        return Date.from(localDateTime.atZone(ZoneId.systemDefault()).toInstant());
    }
}
