package de.fhg.aisec.ids.clearinghouse.idscp2

import de.fhg.aisec.ids.clearinghouse.OwnerList
import de.fhg.aisec.ids.clearinghouse.Utility
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.STATUS_400
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.STATUS_403
import de.fhg.aisec.ids.clearinghouse.Utility.Companion.formatId
import de.fraunhofer.iais.eis.Message
import de.fraunhofer.iais.eis.MessageProcessedNotificationMessage
import de.fraunhofer.iais.eis.RejectionMessage
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import org.junit.Assert
import org.junit.jupiter.api.Test

class CreatePidTests {

    @Test
    fun createPid1(){
        val pid = formatId("idscp-pid1")
        val owners = null

        // Test: createPid with no extra owners
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid2(){
        val pid = formatId("idscp-pid2")
        val owners = listOf(Utility.CONNECTOR_2)

        // Test: createPid with an extra owner
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid3(){
        val pid = formatId("idscp-pid3")
        val owners = listOf(Utility.CONNECTOR_1, Utility.CONNECTOR_2)

        // Test: createPid with duplicate self in owner list
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid4(){
        val pid = formatId("idscp-pid4")
        val owners = listOf(Utility.CONNECTOR_2, Utility.CONNECTOR_2)

        // Test: createPid with duplicate other owner in owner list
        succCreatePid(pid, owners)
    }

    @Test
    fun createPid5(){
        val pid = formatId("idscp-pid5")
        val owners = null

        // Preparation: create PID
        succCreatePid(pid, owners)

        // Test: Try to create existing PID (to which user has access)
        failCreatePid(pid, owners, STATUS_400)
    }

    @Test
    fun createPid6(){
        val pid = formatId("idscp-pid6")
        val owners = null

        // Preparation: create PID
        succCreatePid(pid, owners, client = 2)

        // Test: Try to create existing PID (to which user has access)
        failCreatePid(pid, owners, STATUS_403)
    }

    @Test
    fun createPid7(){
        val pid = formatId("idscp-pid7")
        val owners = "{\"owners\": [\"${Utility.CONNECTOR_2}\",]}"

        // Test: createPid with invalid owner list
        val (resultMessage, resultPayload, _) = Idscp2EndpointTest.pidMessage(pid, owners)

        // check IDS message type
        Assert.assertTrue(resultMessage is RejectionMessage)
        // createPid returns the created PID, but in quotes
        val p = String(resultPayload!!)
        Assert.assertEquals("Unexpected status message", STATUS_400, p)
    }

    companion object{

        fun succCreatePid(pid: String, owners: List<String>?, client: Int = 1){
            val (resultMessage, resultPayload, _) = callCreatePid(pid, owners, client)

            // check IDS message type
            Assert.assertTrue(resultMessage is MessageProcessedNotificationMessage)
            // createPid returns the created PID, but in quotes
            val p = String(resultPayload!!)
            val createdPid = p.substring(1, p.length-1)
            Assert.assertEquals("Returned PID does not match given PID!", pid, createdPid)
        }

        fun failCreatePid(pid: String, owners: List<String>?, em: String){
            val (resultMessage, resultPayload, _) = callCreatePid(pid, owners)
            // check IDS message type
            Assert.assertTrue(resultMessage is RejectionMessage)
            // payload = http status code message
            val p = String(resultPayload!!)
            Assert.assertEquals("Unexpected status code message", em, p)
        }

        private fun callCreatePid(pid: String, owners: List<String>?, c: Int = 1): Triple<Message?, ByteArray?, Map<String, String>?> {
            var list = ""
            if (owners != null) {
                list = Json.encodeToString(OwnerList(owners))
            }
            return Idscp2EndpointTest.pidMessage(pid, list, client=c)
        }
    }

}