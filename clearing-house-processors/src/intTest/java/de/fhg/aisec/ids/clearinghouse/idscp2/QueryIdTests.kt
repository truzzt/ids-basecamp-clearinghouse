package de.fhg.aisec.ids.clearinghouse.idscp2

import de.fhg.aisec.ids.clearinghouse.Utility.Companion.STATUS_404
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.idscp2.LogMessageTests.Companion.succLogMessage
import de.fraunhofer.iais.eis.RejectionMessage
import de.fraunhofer.iais.eis.ResultMessage
import org.junit.Assert
import org.junit.jupiter.api.Test

class QueryIdTests {

    @Test
    fun queryId1(){
        val pid = formatId("idscp-qid1")

        // create Pid with one document
        val message = "This is the first message"
        val receipt = LogMessageTests.succLogMessage(pid, message)

        // Test: query existing document
        succQueryId(pid, receipt.document_id)
    }

    @Test
    fun queryId2(){
        val pid = formatId("idscp-qid2")

        // create Pid with one document
        val message = "This is the first message"
        succLogMessage(pid, message)

        // Test: query non-existing document
        failQueryId(pid, "unknown-id", STATUS_404)
    }

    @Test
    fun queryId3(){
        val pid1 = formatId("idscp-qid2_with_doc")
        val pid2 = formatId("idscp-qid2_without_doc")

        // create one Pid with one document and another with no documents
        val message = "This is the first message"
        val receipt = succLogMessage(pid1, message)
        CreatePidTests.succCreatePid(pid2, null)

        // Test: query existing document in wrong pid
        failQueryId(pid2, receipt.document_id, STATUS_404)
    }

    companion object{

        fun failQueryId(pid: String, id: String?, em: String) {
            val (resultMessage, resultPayload, resultHeaders) = Idscp2EndpointTest.queryMessage(pid, id, "")
            // check IDS message type
            Assert.assertTrue(resultMessage is RejectionMessage)
            // payload = http status code message
            val p = String(resultPayload!!)
            Assert.assertEquals("Unexpected status code message", em, p)
        }

        fun succQueryId(pid: String, id: String): String {
            val (resultMessage, resultPayload, resultHeaders) = Idscp2EndpointTest.queryMessage(pid, id, "")
            // check IDS message type
            Assert.assertTrue(resultMessage is ResultMessage)
            //TODO: can't serialize json. array is of type "message + payload + payload type"
            val p = String(resultPayload!!)
            return p
        }
    }
}
