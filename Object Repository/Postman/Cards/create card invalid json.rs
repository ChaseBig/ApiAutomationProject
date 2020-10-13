<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create card invalid json</name>
   <tag></tag>
   <elementGuidId>d4698a2e-60f3-4e2c-b7d7-d908a3f227d7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;user_token\&quot;: \&quot;${user_token}\&quot;,\n  \&quot;card_product_token\&quot;: \&quot;${card_product_token}\&quot;,\n  \&quot;expedite\&quot;: ${expedite},\n  \&quot;metadata\&quot;: {\n    \&quot;additionalProp1\&quot;: \&quot;${additionalProp1}\&quot;\n  }\n}&quot;,
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
      <id>40f54aab-746a-4ee0-86cc-9cff23d74c04</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.user_token</defaultValue>
      <description></description>
      <id>99320aae-b748-46fb-aa22-5a96c374fbb1</id>
      <masked>false</masked>
      <name>user_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.card_product_token</defaultValue>
      <description></description>
      <id>e1941f0b-cf2a-4382-9a1b-ac7a75c1d5d7</id>
      <masked>false</masked>
      <name>card_product_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.expedite</defaultValue>
      <description></description>
      <id>501793d3-0163-4e28-abff-c6af8f1d594c</id>
      <masked>false</masked>
      <name>expedite</name>
   </variables>
   <variables>
      <defaultValue>'😜'</defaultValue>
      <description></description>
      <id>87820bb9-5e22-456c-ae3a-fd4fab2defa7</id>
      <masked>false</masked>
      <name>additionalProp1</name>
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
def jsonResponse = jsonSlurper.parseText(response.getResponseText())
WS.verifyResponseStatusCode(response, 500)

assertThat(response.getStatusCode()).isEqualTo(500)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
