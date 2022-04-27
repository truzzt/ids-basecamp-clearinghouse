package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.ChJwt
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.multipart.CreatePidTests.Companion.succCreatePid
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.client
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessage
import de.fraunhofer.iais.eis.RejectionMessage
import okhttp3.MultipartReader
import org.junit.Assert
import org.junit.jupiter.api.Test

class LogMessageTests {
    @Test
    fun logMessage1(){
        val pid = formatId("mp-log1")
        val payload = "This message is logged"

        // create Pid
        succCreatePid(pid, null)

        // test: Logging to existing Pid
        succLogMessage(pid, payload)
    }

    @Test
    fun logMessage2(){
        val pid = formatId("mp-log2")
        val payload = "This message is logged"

        // test: Logging to non-existing Pid
        succLogMessage(pid, payload)
    }

    @Test
    fun logMessage3(){
        val pid = formatId("mp-log3")
        val payload = ""

        // test: Logging an empty payload
        failLogMessage(pid, payload, 400)
    }

    @Test
    fun logMessage4(){
        val pid = formatId("mp-log4")
        val payload = "This message is logged"

        // create Pid
        succCreatePid(pid, null, client=2)

        // test: Logging to existing Pid
        failLogMessage(pid, payload, 403)
    }

    @Test
    fun logMessage5(){
        val pid = formatId("mp-log5")
        val payload = "This message is logged"

        // test: Logging an empty payload
        failEarlyLogMessage(pid, payload, 401)
    }

    companion object{

        fun failEarlyLogMessage(pid: String, payload: String, code: Int){
            val call = client.newCall(MultipartClient.logMessage(pid, payload, false))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", code, response.code)
            // TODO: check that there is no IDS header
            println(response.body!!.string())
            Assert.assertTrue(false)
            //val parts = Utility.getParts(MultipartReader(response.body!!))
            //Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
        }

        fun failLogMessage(pid: String, payload: String, code: Int){
            val call = client.newCall(MultipartClient.logMessage(pid, payload))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", code, response.code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
        }

        fun succLogMessage(pid: String, payload: String, c: Int = 1): ChJwt {
            val call = client.newCall(MultipartClient.logMessage(pid, payload, client=c))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", 201, response.code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, MessageProcessedNotificationMessage::class.java)
            // check the pid from receipt in the payload. Does pid match with the given pid?
            val receipt = Utility.parseJwt(parts.second)
            Assert.assertEquals("Returned PID does not match given PID!", pid, receipt.process_id)
            response.close()
            return receipt
        }
    }

}