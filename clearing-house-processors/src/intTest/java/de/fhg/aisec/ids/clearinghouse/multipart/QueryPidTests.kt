package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.CONNECTOR_1
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.multipart.CreatePidTests.Companion.succCreatePid
import de.fhg.aisec.ids.clearinghouse.multipart.LogMessageTests.Companion.succLogMessage
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.client
import de.fraunhofer.iais.eis.RejectionMessage
import de.fraunhofer.iais.eis.ResultMessage
import okhttp3.MultipartReader
import org.junit.Assert
import org.junit.jupiter.api.Test

class QueryPidTests {
    @Test
    fun queryPid1(){
        val pid = formatId("mp-qpid1")

        // create Pid
        succCreatePid(pid, null)

        // Test: query existing Pid with no documents
        val result = succQueryPid(pid)
        Assert.assertEquals("Should receive empty JSON array!", "[]", result)

    }

    @Test
    fun queryPid2(){
        val pid = formatId("mp-qpid2")

        // create Pid with three messages
        val messages = listOf("This is the first message", "This is the second message", "This is the third message")
        messages.forEach{
            succLogMessage(pid, it)
        }

        // Test: query existing Pid with three documents
        val docs = succQueryPid(pid)
        println("body: $docs")
        //TODO: test that we have 3 items in the json
    }

    @Test
    fun queryPid3(){
        val pid = formatId("mp-qpid3")
        val owners = listOf(CONNECTOR_1)

        // create Pid with other user, but user 1 is also authorized
        succCreatePid(pid, owners, client=2)

        // add message
        succLogMessage(pid, "This message is logged", c=2)

        // Test: query existing Pid with user (who did not create pid, but is authorized)
        val docs = succQueryPid(pid)
        println("body: $docs")
        //TODO: test that we have 3 items in the json
    }

    @Test
    fun queryPid4(){
        val pid = formatId("mp-qpid4")

        // Test: query non-existing Pid
        failQueryPid(pid, 404)
    }

    @Test
    fun queryPid5(){
        val pid = formatId("mp-qpid5")

        // create Pid with other user
        succCreatePid(pid, null, client=2)

        // Test: query existing Pid with user (for which he is not authorized)
        failQueryPid(pid, 403)
    }

    companion object{
        fun failQueryPid(pid: String, code: Int){
            val call = client.newCall(MultipartClient.queryMessage(pid, null, ""))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", code, response.code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
            response.close()
        }

        fun succQueryPid(pid: String): String{
            val call = client.newCall(MultipartClient.queryMessage(pid, null, ""))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", 200, response.code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, ResultMessage::class.java)
            //TODO: can't serialize bc json array is of type "message + payload + payload type"
            response.close()
            return parts.second
        }
    }
}