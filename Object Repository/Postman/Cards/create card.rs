<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create card</name>
   <tag></tag>
   <elementGuidId>20489fc3-ceef-4e73-91e2-4daa5cd827e3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;user_token\&quot;: \&quot;${user_token}\&quot;,\n  \&quot;card_product_token\&quot;: \&quot;${card_product_token}\&quot;\n}&quot;,
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
   <restUrl>https://${url}/cards</restUrl>
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
      <id>e21bdf58-f31b-4bfe-a6dd-12a5d839005e</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.user_token</defaultValue>
      <description></description>
      <id>6d5d4cf5-fbf2-48d1-bf68-69fa5fa1cb2e</id>
      <masked>false</masked>
      <name>user_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.card_product_token</defaultValue>
      <description></description>
      <id>ae1d3cdb-95c0-423a-8e30-d8a290cb76ab</id>
      <masked>false</masked>
      <name>card_product_token</name>
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
assert jsonResponse.token != null
println jsonResponse.token
card_token = jsonResponse.token
log.logInfo('-----> card_token = ' + card_token)
GlobalVariable.card_token = card_token
log.logInfo('----> GlobalVariable card_token = ' + GlobalVariable.card_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
