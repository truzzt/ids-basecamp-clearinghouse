package de.fhg.aisec.ids.clearinghouse;

import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig;
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import java.nio.file.Paths;

class Configuration {

    private final static Logger LOG = LoggerFactory.getLogger(Configuration.class);

    private static final String DAPS_ENV_VARIABLE = "TC_DAPS_URL";
    private static final String KEYSTORE_PASS_ENV_VARIABLE = "TC_KEYSTORE_PW";
    private static final String TRUSTSTORE_PASS_ENV_VARIABLE = "TC_TRUSTSTORE_PW";
    // keep this in sync with libraryVersions.yaml
    private static final String INFOMODEL_VERSION = "4.1.0";
    private static final String SENDER_AGENT = "TC_CH_AGENT";
    private static final String ISSUER_CONNECTOR = "TC_CH_ISSUER_CONNECTOR";

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

    static String getInfomodelVersion(){
        return INFOMODEL_VERSION;
    }

    static String getSenderAgent(){
        return getEnvVariable(SENDER_AGENT);
    }

    static String getIssuerConnector(){
        return getEnvVariable(ISSUER_CONNECTOR);
    }

    private static String getEnvVariable(String envVariable){
        var value = System.getenv(envVariable);
        if (value == null) {
            LOG.error("Configuration invalid: Missing {}", envVariable);
            return "";
        }
        else{
            return value;
        }
    }
}
