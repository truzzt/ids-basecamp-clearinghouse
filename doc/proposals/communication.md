# Communication Proposal

Für den aktuellen Betrieb des MDS würden wir auf die Clearinghouse Specification des IDS RAM 4.0 setzen. 
Dabie kann das bestehende Clearinghouse angepasst und verbessert werden durch die folgenden Punkte:

Austausch des Trusted Connectors mittels EDC
Zusammenführung der MS zur CH-APP
Austausch des Webservers Rocket durch Axum
Wartung und Optimierungen
Stabilität durch Mutex
Update der Dependencies
Dadurch ist das Clearinghouse IDS RAM 4.0 complient und rückwärts kompatibel mit EDC MS8

### Offene Entscheidungen:

Blockchain
Masterkey
Future
Im DSP wird es kein Clearinghouse wie es in der IDS RAM 4.0 spezifiziert mehr geben. 
Das Clearinghouse wird vom DSP ledeglich als Teilnehmer gesehen.
Dabei werden die Logs der Connectoren dezentral nur im jeweiligen Connector liegen. 
Das Clearinghouse im bereich Logging könnte somit einen Vertrag mit allen Connectoren schließen um diese Logs anzufragen.

### Clearinghouse und DAPS
In Hinblick auf die anstehende Migration zu did:web bietet das Clearnghouse einen sinnvollen Ersatz für den DAPS.
Das Clearinghouse könnte Verifiable Credentials ausstellen, sobald die Teilnehmer den Vertrag mit diesem eingegangen sind und die Grundvorraussetzungen um am Dataspace zu partizipieren erfüllt sind. Jeder Teilnehmer darf nur mit Mitgliedern des Dataspaces interagieren, die dieses Verifiable Credential vorweisen können.
Dadurch wird sichergestellt das alle Teilnehmer am Datenraum das Clearinghouse akzeptieren.

## Aktuelle Implementierung
Der Endpunkt ```POST /messages/log/:PID``` wird mit einer zufällig generierten PID aufgerufen. Das hat einige Nachteile:
- Es wird für jede Transaktion ein neuer Prozess angelegt.
- Transaktionen können nicht gruppiert (einem Vertrag zugeordnet) werden.
- Transaktionen von anderen Connectoren können nicht zur gleichen Transaktion gefiltert werden.

## Optimierter Ansatz
Bevor eine Transaktion stattfindet, wird ein Vertrag geschlossen. In diesem Schritt könnte der Prozess im Clearinghouse bereits angelegt werden. Hierbei ist es auch möglich, mehrere Connector IDs anzugeben, um festzulegen, wer Lese- und Schreibrechte besitzt.
- Die erstellte PID muss mit allen Connectoren geteilt werden.
- Die Connectoren können auf die gleiche PID loggen, um die Transaktionen nach Verträgen zu gruppieren.
- Der MDS kann seinen eigenen Connector als Standard festlegen, um Zugriff auf alle Transaktionen zu erhalten.

## Ablauf
### CreateLogMessage
![](../images/CreateLogMessage.png)

### CreatePID
![](../images/CreatePid.png)