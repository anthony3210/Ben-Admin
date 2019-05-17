<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>This is an HTTP POST request that creates a client.</description>
   <name>CLIENT</name>
   <tag></tag>
   <elementGuidId>0d0663fc-d1de-46c9-a161-b74c0953c677</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n\&quot;parentClientUUID\&quot;:\&quot;${parentClientUUID}\&quot;,\n\&quot;legalClientName\&quot;:\&quot;${legalClientName}\&quot;,\n\&quot;clientName\&quot;:\&quot;${clientName}\&quot;,\n\&quot;sicCode\&quot;:\&quot;${sicCode}\&quot;,\n\&quot;clientEffectiveDate\&quot;:\&quot;${clientEffectiveDate}\&quot;,\n\&quot;clientTerminationDate\&quot;:\&quot;${clientTerminationDate}\&quot;,\n\&quot;streetAddressUUID\&quot;:\&quot;${streetAddressUUID}\&quot;,\n\&quot;billingAddressUUID\&quot;:\&quot;${billingAddressUUID}\&quot;,\n\&quot;subDomain\&quot;:\&quot;${subDomain}\&quot;,\n\&quot;memberUniqueIdentifier\&quot;:\&quot;${memberUniqueIdentifier}\&quot;,\n\&quot;memberEmailCommunication\&quot;:\&quot;${memberEmailCommunication}\&quot;,\n\&quot;inviteKey\&quot;:\&quot;${inviteKey}\&quot;,\n\&quot;billingContactEmail\&quot;:\&quot;${billingContactEmail}\&quot;,\n\&quot;configurationDemo\&quot;:\&quot;${configurationDemo}\&quot;,\n\&quot;configurationInActive\&quot;:\&quot;${configurationInActive}\&quot;,\n\&quot;configurationTemplate\&quot;:\&quot;${configurationTemplate}\&quot;,\n\&quot;primaryContactName\&quot;:\&quot;${primaryContactName}\&quot;,\n\&quot;primaryContactPhone\&quot;:\&quot;${primaryContactPhone}\&quot;,\n\&quot;primaryContactEmail\&quot;:\&quot;${primaryContactEmail}\&quot;,\n\&quot;billingContactName\&quot;:\&quot;${billingContactName}\&quot;,\n\&quot;billingContactPhone\&quot;:\&quot;${billingContactPhone}\&quot;\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://api.benadmin.qa.maestroedgy.com/api/clientperson/v1/client/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>'Harriet'</defaultValue>
      <description></description>
      <id>3198db23-540a-4af2-b539-79b67a2ced30</id>
      <masked>false</masked>
      <name>legalClientName</name>
   </variables>
   <variables>
      <defaultValue>'Sanders'</defaultValue>
      <description></description>
      <id>15116a70-4848-4222-a5e1-aee1c724cca5</id>
      <masked>false</masked>
      <name>clientName</name>
   </variables>
   <variables>
      <defaultValue>'hsanderstest@test.com'</defaultValue>
      <description></description>
      <id>040fce40-8cbf-4934-8175-11d626cce7ca</id>
      <masked>false</masked>
      <name>primaryContactEmail</name>
   </variables>
   <variables>
      <defaultValue>'ec7052a6-7b02-4110-b0f9-288284956dc2'</defaultValue>
      <description></description>
      <id>61b0546b-aaec-4c45-834b-b994c7fda76e</id>
      <masked>false</masked>
      <name>parentClientUUID</name>
   </variables>
   <variables>
      <defaultValue>'1234'</defaultValue>
      <description></description>
      <id>3262063e-c424-43ab-b171-a9bbdcd895f2</id>
      <masked>false</masked>
      <name>sicCode</name>
   </variables>
   <variables>
      <defaultValue>'2019-01-01'</defaultValue>
      <description></description>
      <id>66a7d24d-8745-49dd-a6bc-4e528be84147</id>
      <masked>false</masked>
      <name>clientEffectiveDate</name>
   </variables>
   <variables>
      <defaultValue>'2020-12-31'</defaultValue>
      <description></description>
      <id>73273b23-a945-4f4c-a5d9-c3220eed023b</id>
      <masked>false</masked>
      <name>clientTerminationDate</name>
   </variables>
   <variables>
      <defaultValue>'11d231c2-5f06-4387-ab4d-09d5ffbfe23c'</defaultValue>
      <description></description>
      <id>e8d43a8b-e2b4-4b60-a2bc-8ae27e709cf6</id>
      <masked>false</masked>
      <name>streetAddressUUID</name>
   </variables>
   <variables>
      <defaultValue>'11d231c2-5f06-4387-ab4d-09d5ffbfe23c'</defaultValue>
      <description></description>
      <id>30b7b265-29db-4c0d-8eb7-d3cce504408b</id>
      <masked>false</masked>
      <name>billingAddressUUID</name>
   </variables>
   <variables>
      <defaultValue>'sub'</defaultValue>
      <description></description>
      <id>465a9dff-c264-4179-b464-92ad2bf39dc3</id>
      <masked>false</masked>
      <name>subDomain</name>
   </variables>
   <variables>
      <defaultValue>'member_uuid'</defaultValue>
      <description></description>
      <id>f9a5c760-8958-47f0-8567-5cf7c9e02aa0</id>
      <masked>false</masked>
      <name>memberUniqueIdentifier</name>
   </variables>
   <variables>
      <defaultValue>'123'</defaultValue>
      <description></description>
      <id>cede480d-d26d-4e27-b5a5-53c18cf22f17</id>
      <masked>false</masked>
      <name>inviteKey</name>
   </variables>
   <variables>
      <defaultValue>'123@gmail.com'</defaultValue>
      <description></description>
      <id>a162edaa-6ccc-483a-ad37-4e976294967f</id>
      <masked>false</masked>
      <name>billingContactEmail</name>
   </variables>
   <variables>
      <defaultValue>'false'</defaultValue>
      <description></description>
      <id>3cfc019d-ec49-4ff9-9e04-329efe4296a0</id>
      <masked>false</masked>
      <name>configurationDemo</name>
   </variables>
   <variables>
      <defaultValue>'false'</defaultValue>
      <description></description>
      <id>70c0f245-e98d-4299-aa0e-b65411f02afc</id>
      <masked>false</masked>
      <name>configurationInActive</name>
   </variables>
   <variables>
      <defaultValue>'false'</defaultValue>
      <description></description>
      <id>9bfa3911-0023-4735-a381-55d6c0ac00a9</id>
      <masked>false</masked>
      <name>configurationTemplate</name>
   </variables>
   <variables>
      <defaultValue>'Test'</defaultValue>
      <description></description>
      <id>3023062b-14b7-4ce9-b4e0-6e65875a0461</id>
      <masked>false</masked>
      <name>primaryContactName</name>
   </variables>
   <variables>
      <defaultValue>'3125551212'</defaultValue>
      <description></description>
      <id>ee7ddb5c-774e-4f7e-b375-2cd3365721de</id>
      <masked>false</masked>
      <name>primaryContactPhone</name>
   </variables>
   <variables>
      <defaultValue>'Test'</defaultValue>
      <description></description>
      <id>7cd93fbf-4c9b-4e72-a3ec-7f4cadea80a3</id>
      <masked>false</masked>
      <name>billingContactName</name>
   </variables>
   <variables>
      <defaultValue>'3125551212'</defaultValue>
      <description></description>
      <id>ae00e66a-94a7-4ccd-b88e-406f5901a33d</id>
      <masked>false</masked>
      <name>billingContactPhone</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>bf5fb3d8-450e-4dff-919a-7c1a60df9d47</id>
      <masked>false</masked>
      <name>clientUUID</name>
   </variables>
   <variables>
      <defaultValue>'true'</defaultValue>
      <description></description>
      <id>a119515f-1298-42d6-b88f-05ff0e0c9896</id>
      <masked>false</masked>
      <name>memberEmailCommunication</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()




</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
