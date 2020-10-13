<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create cardtransitions2</name>
   <tag></tag>
   <elementGuidId>f7007fa6-0ae3-4a10-a594-1c9e369dc348</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;card_token\&quot;: \&quot;${card_token}\&quot;,\n  \&quot;state\&quot;: \&quot;${transition_state}\&quot;,\n  \&quot;channel\&quot;: \&quot;${channel}\&quot;,\n  \&quot;reason_code\&quot;: \&quot;${transition_reason_code}\&quot;\n}&quot;,
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
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}/cardtransitions</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.card_token</defaultValue>
      <description></description>
      <id>95b244d9-91d9-458c-b674-69b6f89d9031</id>
      <masked>false</masked>
      <name>card_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transition_state</defaultValue>
      <description></description>
      <id>b352e68d-67e0-4d0b-8c03-6112a1670c5e</id>
      <masked>false</masked>
      <name>state</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.channel</defaultValue>
      <description></description>
      <id>953b05a7-340c-425d-b4fc-ea83845c1c5e</id>
      <masked>false</masked>
      <name>channel</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.url</defaultValue>
      <description></description>
      <id>c9b25a06-06df-46fb-bc5c-f0c6e97fcb6d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.transition_reason_code</defaultValue>
      <description></description>
      <id>26ab5637-5930-41b3-99bb-aa76ff8df307</id>
      <masked>false</masked>
      <name>reason_code</name>
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
transitionState = jsonResponse.state
log.logInfo('-----> transitionState = ' + transitionState)
WS.verifyElementPropertyValue(response, 'state', 'SUSPENDED')
assert jsonResponse.token != null
println jsonResponse.token
card_token = jsonResponse.token
log.logInfo('-----> card_token = ' + card_token)
GlobalVariable.card_token = card_token
log.logInfo('----> GlobalVariable card_token = ' + GlobalVariable.card_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
