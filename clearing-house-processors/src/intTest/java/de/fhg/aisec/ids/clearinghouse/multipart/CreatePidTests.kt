package de.fhg.aisec.ids.clearinghouse.multipart

import de.fhg.aisec.ids.clearinghouse.OwnerList
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.client
import de.fhg.aisec.ids.clearinghouse.multipart.MultipartEndpointTest.Companion.otherClient
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessage
import de.fraunhofer.iais.eis.RejectionMessage
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import okhttp3.MultipartReader
import okhttp3.Response
import org.junit.Assert
import org.junit.jupiter.api.Test

class CreatePidTests {

    @Test
    fun createPid1(){
        val pid = formatId("mp-pid1")
        val owners = null

        // Test: createPid with no extra owners
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid2(){
        val pid = formatId("mp-pid2")
        val owners = listOf(Utility.CONNECTOR_2)

        // Test: createPid with an extra owner
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid3(){
        val pid = formatId("mp-pid3")
        val owners = listOf(Utility.CONNECTOR_1, Utility.CONNECTOR_2)

        // Test: createPid with duplicate self in owner list
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid4(){
        val pid = formatId("mp-pid4")
        val owners = listOf(Utility.CONNECTOR_2, Utility.CONNECTOR_2)

        // Test: createPid with duplicate other owner in owner list
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid5(){
        val pid = formatId("mp-pid5")
        val owners = null

        // Preparation: create PID
        succCreatePid(pid, owners)

        // Test: Try to create existing PID (to which user has access)
        failCreatePid(pid, owners, 400)
    }

    @Test
    fun createPid6(){
        val pid = formatId("mp-pid6")
        val owners = null

        // Preparation: create PID
        succCreatePid(pid, owners, client=2)

        // Test: Try to create existing PID (to which user has access)
        failCreatePid(pid, owners, 403)
    }

    @Test
    fun createPid7(){
        val pid = formatId("mp-pid7")
        val owners = "{\"owners\": [\"${Utility.CONNECTOR_2}\",]}"

        // Test: createPid with invalid owner list
        val call = client.newCall(MultipartClient.pidMessage(pid, owners))
        val response = call.execute()

        // check http status code
        Assert.assertEquals("Unexpected http status code!", 400, response.code)
        // check IDS message type
        val parts = Utility.getParts(MultipartReader(response.body!!))
        Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
        response.close()
    }

    @Test
    fun createPid8(){
        val pid = formatId("mp-pid8")

        // Test: Create Pid without matching aki:ski in certificate
        failEarlyCreatePid(pid, null, 401)
    }


    companion object{

        fun succCreatePid(pid: String, owners: List<String>?, client: Int = 1){
            val response = callCreatePid(pid, owners, client)
            val parts = Utility.getParts(MultipartReader(response.body!!))
            // check http status code
            Assert.assertEquals("Unexpected http status code!", 201, response.code)
            // check IDS message type
            Utility.checkIdsMessage(parts.first, MessageProcessedNotificationMessage::class.java)
            // createPid returns the created PID, but in quotes
            val createdPid = parts.second.substring(1, parts.second.length-1)
            Assert.assertEquals("Returned PID does not match given PID!", pid, createdPid)
            response.close()
        }

        fun failCreatePid(pid: String, owners: List<String>?, code: Int){
            val response = callCreatePid(pid, owners)
            val parts = Utility.getParts(MultipartReader(response.body!!))
            // check http status code
            Assert.assertEquals("Unexpected http status code!", code, response.code)
            // check IDS message type
            Utility.checkIdsMessage(parts.first, RejectionMessage::class.java)
            response.close()
        }

        private fun callCreatePid(pid: String, owners: List<String>?, c: Int = 1): Response {
            var list = ""
            if (owners != null) {
                list = Json.encodeToString(OwnerList(owners))
            }
            val call = when (c) {
                1 -> client.newCall(MultipartClient.pidMessage(pid, list, client=c))
                else -> otherClient.newCall(MultipartClient.pidMessage(pid, list, client=c))
            }
            return call.execute()
        }

        fun failEarlyCreatePid(pid: String, owners: List<String>?, code: Int){
            var list = ""
            if (owners != null) {
                list = Json.encodeToString(OwnerList(owners))
            }
            val call = client.newCall(MultipartClient.pidMessage(pid, list, client=2))
            val response = call.execute()
            // check http status code and message
            Assert.assertEquals("Unexpected http status code!", code, response.code)
            Assert.assertEquals("Unexpected message", "Unauthorized", response.message)
        }
    }
}
