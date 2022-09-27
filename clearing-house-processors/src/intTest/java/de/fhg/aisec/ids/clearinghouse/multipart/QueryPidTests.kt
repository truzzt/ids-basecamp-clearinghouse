package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.QueryResult
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.CONNECTOR_1
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.parseQueryResult
import de.fhg.aisec.ids.clearinghouse.multipart.CreatePidTests.Companion.succCreatePid
import de.fhg.aisec.ids.clearinghouse.multipart.LogMessageTests.Companion.succLogMessage
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.client
import de.fraunhofer.iais.eis.RejectionMessage
import de.fraunhofer.iais.eis.ResultMessage
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.jsonObject
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
        Assert.assertEquals("Should receive empty array!", 0, result.documents.size)
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
        val result = succQueryPid(pid)
        Assert.assertEquals("Should receive empty array!", 3, result.documents.size)
    }

    @Test
    fun queryPid3(){
        val pid = formatId("mp-qpid3")
        val owners = listOf(CONNECTOR_1)

        // create Pid with other user, but user 1 is also authorized
        succCreatePid(pid, owners, client=2)

        // add three messages
        val messages = listOf("This is the first message", "This is the second message", "This is the third message")
        messages.forEach{
            succLogMessage(pid, it, c=2)
        }

        // Test: query existing Pid with user (who did not create pid, but is authorized)
        val result = succQueryPid(pid)
        Assert.assertEquals("Should receive empty array!", 3, result.documents.size)
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

    @Test
    fun queryPid6(){
        val pid = formatId("mp-qpid6")

        // create Pid
        succLogMessage(pid, "This is the log message!")

        // Test: query non existing page results in empty array
        val result = succQueryPid(pid, 2)
        Assert.assertEquals("Should receive empty array!", 0, result.documents.size)
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

        fun succQueryPid(pid: String, page: Int = 1, size: Int = 100, sort: String = "desc"): QueryResult{
            val call = client.newCall(MultipartClient.queryMessage(pid, null, "", page=page, size=size, sort=sort))
            val response = call.execute()
            // check http status code
            Assert.assertEquals("Unexpected http status code!", 200, response.code)
            // check IDS message type
            val parts = Utility.getParts(MultipartReader(response.body!!))
            Utility.checkIdsMessage(parts.first, ResultMessage::class.java)
            val result = parseQueryResult(parts.second)
            response.close()
            return result
        }
    }
}