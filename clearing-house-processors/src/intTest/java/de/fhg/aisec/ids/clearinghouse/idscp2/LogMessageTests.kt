package de.fhg.aisec.ids.clearinghouse.idscp2

import de.fhg.aisec.ids.clearinghouse.ChJwt
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.STATUS_400
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.STATUS_403
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.idscp2.CreatePidTests.Companion.succCreatePid
import de.fraunhofer.iais.eis.*
import org.junit.Assert
import org.junit.jupiter.api.Test

class LogMessageTests {
    @Test
    fun logMessage1(){
        val pid = formatId("idscp-log1")
        val payload = "This message is logged"

        // create Pid
        succCreatePid(pid, null)

        // test: Logging to existing Pid
        succLogMessage(pid, payload)
    }

    @Test
    fun logMessage2() {
        val pid = formatId("idscp-log2")
        val payload = "This message is logged"

        succLogMessage(pid, payload)
    }

    @Test
    fun logMessage3(){
        val pid = formatId("idscp-log3")
        val payload = ""

        // test: Logging an empty payload
        failLogMessage(pid, payload, STATUS_400)
    }

    @Test
    fun logMessage4(){
        val pid = formatId("idscp-log4")
        val payload = "This message is logged"

        // create Pid
        succCreatePid(pid, null, client = 2)

        // test: Logging to existing Pid
        failLogMessage(pid, payload, STATUS_403)
    }

    companion object{

        fun failLogMessage(pid: String, payload: String, em: String) {
            val (resultMessage, resultPayload, _) = Idscp2EndpointTest.logMessage(pid, payload)
            // check IDS message type
            Assert.assertTrue(resultMessage is RejectionMessage)
            // payload = http status code message
            val p = String(resultPayload!!)
            Assert.assertEquals("Unexpected status code message", em, p)
        }

        fun succLogMessage(pid: String, payload: String, c: Int = 1): ChJwt {
            val (resultMessage, resultPayload, resultHeaders) = Idscp2EndpointTest.logMessage(pid, payload)
            // check IDS message type
            Assert.assertTrue(resultMessage is MessageProcessedNotificationMessage)
            // check the pid from receipt in the payload. Does pid match with the given pid?
            val receipt = Utility.parseJwt(String(resultPayload!!))
            Assert.assertEquals("Returned PID does not match given PID!", pid, receipt.process_id)
            return receipt
        }
    }

}