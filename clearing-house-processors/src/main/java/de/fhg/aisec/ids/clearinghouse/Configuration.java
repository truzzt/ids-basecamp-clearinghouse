package de.fhg.aisec.ids.clearinghouse;

import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements;

import java.nio.file.Paths;

class Configuration {

    private static final String DAPS_ENV_VARIABLE = "TC_DAPS_URL";
    private static final String KEYSTORE_PASS_ENV_VARIABLE = "TC_KEYSTORE_PW";
    private static final String TRUSTSTORE_PASS_ENV_VARIABLE = "TC_TRUSTSTORE_PW";

    static AisecDapsDriverConfig createDapsConfig(SecurityRequirements securityRequirements){
        var dapsUrl = System.getenv(DAPS_ENV_VARIABLE);
        var keystorePassword = System.getenv(KEYSTORE_PASS_ENV_VARIABLE);
        var truststorePassword = System.getenv(TRUSTSTORE_PASS_ENV_VARIABLE);
        var dapsConfigBuilder = new AisecDapsDriverConfig.Builder()
                .setKeyStorePath(Paths.get("/root/etc/keystore.p12"))
                .setTrustStorePath(Paths.get("/root/etc/truststore.p12"))
                .setKeyAlias("1")
                .setSecurityRequirements(securityRequirements);

        if (dapsUrl != null){
            dapsConfigBuilder.setDapsUrl(dapsUrl);
        }

        if (keystorePassword != null){
            dapsConfigBuilder.setKeyStorePassword(keystorePassword.toCharArray());
        }

        if (truststorePassword != null){
            dapsConfigBuilder.setTrustStorePassword(truststorePassword.toCharArray());
        }

        return dapsConfigBuilder.build();
    }
}
