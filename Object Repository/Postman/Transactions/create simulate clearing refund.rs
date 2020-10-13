<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create simulate clearing refund</name>
   <tag></tag>
   <elementGuidId>8806c45d-ae4b-4723-a9a3-6c8f46736fa1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;original_transaction_token\&quot;: \&quot;${original_transaction_token}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;mid\&quot;: \&quot;${mid}\&quot;,\n  \&quot;is_refund\&quot;: \&quot;${is_refund}\&quot;,\n    \&quot;webhook\&quot;: {\n    \&quot;endpoint\&quot;: \&quot;${beeceptor_create_refund}\&quot;,\n    \&quot;username\&quot;: \&quot;${beeceptor_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${beeceptor_password}\&quot;\n  }\n}&quot;,
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
      <id>7b4c1d9a-600b-4473-899c-ee700a8cd3c3</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.original_transaction_token</defaultValue>
      <description></description>
      <id>8a153f93-fbe5-4f47-aabf-248f3877667e</id>
      <masked>false</masked>
      <name>original_transaction_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trans_amount1</defaultValue>
      <description></description>
      <id>d556f1c9-2de3-4825-8a48-950e44cee8c6</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mid</defaultValue>
      <description></description>
      <id>97b7dd3f-da41-4d8c-8e78-c3ffc2b25864</id>
      <masked>false</masked>
      <name>mid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.is_refund</defaultValue>
      <description></description>
      <id>581d23da-6b3b-48a2-a801-9be42c55dc8d</id>
      <masked>false</masked>
      <name>is_refund</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_create_refund</defaultValue>
      <description></description>
      <id>2559d326-d6d1-4489-8364-95e573d04963</id>
      <masked>false</masked>
      <name>beeceptor_create_refund</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_username</defaultValue>
      <description></description>
      <id>ef4590ca-b3b7-42f4-b3ac-b786b1ce03a2</id>
      <masked>false</masked>
      <name>beeceptor_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_password</defaultValue>
      <description></description>
      <id>808d7e58-1f4a-4c7f-8032-a5fbda7317ec</id>
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
refund_transaction_token = jsonResponse.transaction.token
log.logInfo('-----> refund_transaction_token = ' + refund_transaction_token)
GlobalVariable.refund_transaction_token = refund_transaction_token
log.logInfo('----> GlobalVariable refund_transaction_token = ' + GlobalVariable.refund_transaction_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
