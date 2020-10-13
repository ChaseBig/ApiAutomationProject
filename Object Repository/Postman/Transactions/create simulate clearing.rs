<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create simulate clearing</name>
   <tag></tag>
   <elementGuidId>c9696f39-957a-4a10-acda-0b3491d9df8e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;original_transaction_token\&quot;: \&quot;${original_transaction_token}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;mid\&quot;: \&quot;${mid}\&quot;,\n  \&quot;force_post\&quot;: \&quot;${force_post}\&quot;,\n  \&quot;webhook\&quot;: {\n    \&quot;endpoint\&quot;: \&quot;${beeceptor_settle_transaction}\&quot;,\n    \&quot;username\&quot;: \&quot;${beeceptor_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${beeceptor_password}\&quot;\n  }\n}&quot;,
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
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic NmI4MGEzODMtNTZhYi00ZjRkLWFkYzItYTk1ZDFiNzVlODgxOjZlMGUzMTQ3LTljZWEtNDIwNS1hOWUzLTUyMDBiZGZjMTM4Zg==</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}/simulate/clearing</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>9c196c16-d51f-4577-95a0-db4552a8a416</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.original_transaction_token</defaultValue>
      <description></description>
      <id>fb16b38b-9941-4bce-a238-765bfe68ef46</id>
      <masked>false</masked>
      <name>original_transaction_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trans_amount1</defaultValue>
      <description></description>
      <id>caf802e0-358d-4c59-9260-c6ecc0d91c20</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mid</defaultValue>
      <description></description>
      <id>f3003973-a742-42e4-b008-18ac350c9d60</id>
      <masked>false</masked>
      <name>mid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.force_post</defaultValue>
      <description></description>
      <id>fd5fcb8b-915d-4548-9922-95340c30fb23</id>
      <masked>false</masked>
      <name>force_post</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_settle_transaction</defaultValue>
      <description></description>
      <id>08781c25-4a98-48c6-8f01-ee054d3e3322</id>
      <masked>false</masked>
      <name>beeceptor_create_transaction</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_username</defaultValue>
      <description></description>
      <id>24d36027-9b2d-484f-a8ba-301105eb7b13</id>
      <masked>false</masked>
      <name>beeceptor_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_password</defaultValue>
      <description></description>
      <id>f1825cf7-0fe0-4ec5-8d1e-758f089ba273</id>
      <masked>false</masked>
      <name>beeceptor_password</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import com.kms.katalon.core.util.KeywordUtil
import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.testobject.impl.HttpTextBodyContent

def KeywordLogger log = new KeywordLogger()
RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
println(request.restUrl.toString())
println(request.httpBody.toString())
log.logInfo(&quot;-------> Request Url= &quot; + request.restUrl.toString())
log.logInfo(&quot;-------> Request Body= &quot; + request.httpBody.toString())

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
println(response.responseBodyContent.toString())
log.logInfo(&quot;-------> Response Body= &quot; + response.responseBodyContent.toString())

JsonSlurper jsonSlurper = new JsonSlurper()
def jsonResponse = jsonSlurper.parseText(response.getResponseText())
assert jsonResponse.transaction.token != null
println jsonResponse.transaction.token
cleared_transaction_token = jsonResponse.transaction.token
log.logInfo('-----> cleared_transaction_token = ' + cleared_transaction_token)
GlobalVariable.cleared_transaction_token = cleared_transaction_token
log.logInfo('----> GlobalVariable cleared_transaction_token = ' + GlobalVariable.cleared_transaction_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
