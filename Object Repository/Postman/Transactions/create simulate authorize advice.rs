<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create simulate authorize advice</name>
   <tag></tag>
   <elementGuidId>8e730f05-7de7-49d7-968a-d90faf594989</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;original_transaction_token\&quot;: \&quot;${original_transaction_token}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;webhook\&quot;: {\n    \&quot;endpoint\&quot;: \&quot;${beeceptor_create_auth_advice}\&quot;,\n    \&quot;username\&quot;: \&quot;${beeceptor_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${beeceptor_password}\&quot;\n  }\n}&quot;,
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
   <restUrl>https://${url}/simulate/authorization/advice</restUrl>
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
      <id>1be08213-9196-489c-b1f6-557304f717d8</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.original_transaction_token</defaultValue>
      <description></description>
      <id>3cfc357f-ec08-4ce1-9be5-1aeefb9880fe</id>
      <masked>false</masked>
      <name>original_transaction_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.gpaamount</defaultValue>
      <description></description>
      <id>4007f2f1-f491-4e3e-a1dd-7e42bc7a7e38</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_create_auth_advice</defaultValue>
      <description></description>
      <id>f79e7f0e-4a1c-4f19-8364-7f227fd03b18</id>
      <masked>false</masked>
      <name>beeceptor_create_auth_advice</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_username</defaultValue>
      <description></description>
      <id>ca452be7-1682-42af-84c5-af72d6b5d81f</id>
      <masked>false</masked>
      <name>beeceptor_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_password</defaultValue>
      <description></description>
      <id>68197135-2cac-4fea-aff9-5af48809083b</id>
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
advice_transaction_token = jsonResponse.transaction.token
log.logInfo('-----> advice_transaction_token = ' + advice_transaction_token)
GlobalVariable.advice_transaction_token = advice_transaction_token
log.logInfo('----> GlobalVariable advice_transaction_token = ' + GlobalVariable.advice_transaction_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
