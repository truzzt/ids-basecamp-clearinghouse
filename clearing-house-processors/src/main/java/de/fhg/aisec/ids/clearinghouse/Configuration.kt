package de.fhg.aisec.ids.clearinghouse

import org.slf4j.LoggerFactory

internal object Configuration {
    private val LOG = LoggerFactory.getLogger(Configuration::class.java)
    private const val LOGGING_SERVICE_ID = "SERVICE_ID_LOG"
    private const val TC_SERVICE_ID = "SERVICE_ID_TC"
    private const val SERVICE_SHARED_SECRET = "SERVICE_SHARED_SECRET"

    val serviceIdTc: String
        get() = getEnvVariable(TC_SERVICE_ID)
    val serviceIdLog: String
        get() = getEnvVariable(LOGGING_SERVICE_ID)
    val serviceSecret: String
        get() = getEnvVariable(SERVICE_SHARED_SECRET)


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
