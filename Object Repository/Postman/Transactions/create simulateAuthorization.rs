<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create simulateAuthorization</name>
   <tag></tag>
   <elementGuidId>6a853103-a2e6-4f03-a531-5fd8e1b375cc</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;card_token\&quot;: \&quot;${card_token}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;mid\&quot;: \&quot;${mid}\&quot;,\n  \&quot;webhook\&quot;: {\n    \&quot;endpoint\&quot;: \&quot;${beeceptor_create_transaction}\&quot;,\n    \&quot;username\&quot;: \&quot;${beeceptor_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${beeceptor_password}\&quot;\n  }\n}&quot;,
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
   <restUrl>https://${url}/simulate/authorization</restUrl>
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
      <id>fe44264e-7d4d-482e-b3c8-fe2e014b680e</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.card_token</defaultValue>
      <description></description>
      <id>e77c70e2-6165-4a66-ad81-e06164f7b82d</id>
      <masked>false</masked>
      <name>card_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trans_amount1</defaultValue>
      <description></description>
      <id>7ede3d74-30ff-4c55-a85e-1c141384ef2f</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mid</defaultValue>
      <description></description>
      <id>080c9fb9-4a90-43fd-93e7-77fd6e06feef</id>
      <masked>false</masked>
      <name>mid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_create_transaction</defaultValue>
      <description></description>
      <id>e642d5af-e685-4b29-a52f-a036f092232f</id>
      <masked>false</masked>
      <name>beeceptor_create_transaction</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_username</defaultValue>
      <description></description>
      <id>e07b9574-08e9-4d9f-b8a3-86eb849f5685</id>
      <masked>false</masked>
      <name>beeceptor_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_password</defaultValue>
      <description></description>
      <id>383cecbf-a6a7-4fea-b3e9-d9eb1ac151d5</id>
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
original_transaction_token = jsonResponse.transaction.token
log.logInfo('-----> original_transaction_token = ' + original_transaction_token)
GlobalVariable.original_transaction_token = original_transaction_token
log.logInfo('----> GlobalVariable original_transaction_token = ' + GlobalVariable.original_transaction_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
