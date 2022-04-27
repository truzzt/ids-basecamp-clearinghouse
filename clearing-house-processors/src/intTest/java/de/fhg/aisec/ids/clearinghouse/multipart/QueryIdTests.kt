package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.multipart.CreatePidTests.Companion.succCreatePid
import de.fhg.aisec.ids.clearinghouse.multipart.LogMessageTests.Companion.succLogMessage
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.client
import de.fraunhofer.iais.eis.RejectionMessage
import de.fraunhofer.iais.eis.ResultMessage
import okhttp3.MultipartReader
import org.junit.Assert
import org.junit.jupiter.api.Test

class QueryIdTests {

    @Test
    fun queryId1(){
        val pid = formatId("mp-qid1")

        // create Pid with one document
        val message = "This is the first message"
        val receipt = succLogMessage(pid, message)

        // Test: query existing document
        succQueryId(pid, receipt.document_id)
    }

    @Test
    fun queryId2(){
        val pid = formatId("mp-qid2")

        // create Pid with one document
        val message = "This is the first message"
        succLogMessage(pid, message)

        // Test: query non-existing document
        failQueryId(pid, "unknown-id", 404)
    }

    @Test
    fun queryId3(){
        val pid1 = formatId("mp-qid2_with_doc")
        val pid2 = formatId("mp-qid2_without_doc")

        // create one Pid with one document and another with no documents
        val message = "This is the first message"
        val receipt = succLogMessage(pid1, message)
        succCreatePid(pid2, null)

        // Test: query existing document in wrong pid
        failQueryId(pid2, receipt.document_id, 404)
    }

    companion object{
        fun failQueryId(pid: String, id: String, code: Int){
            val call = client.newCall(MultipartClient.queryMessage(pid, id, ""))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", response.code, code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
            response.close()
        }

        fun succQueryId(pid: String, id: String): String{
            val call = client.newCall(MultipartClient.queryMessage(pid, id, ""))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", response.code, 200)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, ResultMessage::class.java)
            //TODO: can't serialize json array is of type "message + payload + payload type"
            response.close()
            return parts.second
        }
    }
}