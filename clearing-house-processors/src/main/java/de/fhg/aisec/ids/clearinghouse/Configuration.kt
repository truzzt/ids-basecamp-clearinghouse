package de.fhg.aisec.ids.clearinghouse

import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig.Builder
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.SecurityRequirements
import de.fhg.aisec.ids.idscp2.default_drivers.daps.aisec_daps.AisecDapsDriverConfig
import org.slf4j.LoggerFactory
import java.nio.file.Paths

internal object Configuration {
    private val LOG = LoggerFactory.getLogger(Configuration::class.java)
    private const val DAPS_ENV_VARIABLE = "TC_DAPS_URL"
    private const val KEYSTORE_PASS_ENV_VARIABLE = "TC_KEYSTORE_PW"
    private const val TRUSTSTORE_PASS_ENV_VARIABLE = "TC_TRUSTSTORE_PW"

    // keep this in sync with libraryVersions.yaml
    const val infomodelVersion = "4.1.0"
    private const val SENDER_AGENT = "TC_CH_AGENT"
    private const val ISSUER_CONNECTOR = "TC_CH_ISSUER_CONNECTOR"

    fun createDapsConfig(securityRequirements: SecurityRequirements): AisecDapsDriverConfig {
        val dapsUrl = System.getenv(DAPS_ENV_VARIABLE)
        val keystorePassword = System.getenv(KEYSTORE_PASS_ENV_VARIABLE)
        val truststorePassword = System.getenv(TRUSTSTORE_PASS_ENV_VARIABLE)
        val dapsConfigBuilder: Builder = Builder()
            .setKeyStorePath(Paths.get("/root/etc/keystore.p12"))
            .setTrustStorePath(Paths.get("/root/etc/truststore.p12"))
            .setKeyAlias("1")
            .setSecurityRequirements(securityRequirements)
        if (dapsUrl != null) {
            dapsConfigBuilder.setDapsUrl(dapsUrl)
        }
        if (keystorePassword != null) {
            dapsConfigBuilder.setKeyStorePassword(keystorePassword.toCharArray())
        }
        if (truststorePassword != null) {
            dapsConfigBuilder.setTrustStorePassword(truststorePassword.toCharArray())
        }
        return dapsConfigBuilder.build()
    }

    val senderAgent: String
        get() = getEnvVariable(SENDER_AGENT)
    val issuerConnector: String
        get() = getEnvVariable(ISSUER_CONNECTOR)

    private fun getEnvVariable(envVariable: String): String {
        val value = System.getenv(envVariable)
        return if (value == null) {
            LOG.error("Configuration invalid: Missing {}", envVariable)
            ""
        } else {
            value
        }
    }
}