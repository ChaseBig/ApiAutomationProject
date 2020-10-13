<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create cardproducts</name>
   <tag></tag>
   <elementGuidId>8a970fb7-7570-4ae0-b356-f947a9373ca7</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;start_date\&quot;: \&quot;${start_date}\&quot;,\n  \&quot;name\&quot;: \&quot;${name}\&quot;,\n  \&quot;config\&quot;: {\n    \&quot;fulfillment\&quot;: {\n      \&quot;payment_instrument\&quot;:\&quot;${payment_instrument}\&quot;\n     },\n    \&quot;poi\&quot;: {\n      \&quot;ecommerce\&quot;: ${ecommerce},\n      \&quot;atm\&quot;: \&quot;${atm}\&quot;\n    },\n    \&quot;card_life_cycle\&quot;: {\n      \&quot;activate_upon_issue\&quot;: ${activate_upon_issue}\n    }\n  }\n}&quot;,
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
   <restUrl>https://${url}/cardproducts</restUrl>
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
      <id>41dbf157-2bb3-4445-9657-ab8b4f7a5e1d</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.start_date</defaultValue>
      <description></description>
      <id>a8d3b3d7-dfe5-4ac6-bc0d-c27b4c33e74e</id>
      <masked>false</masked>
      <name>start_date</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.card_product_name</defaultValue>
      <description></description>
      <id>0214eca3-d169-473c-832b-be5143333761</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.payment_instrument</defaultValue>
      <description></description>
      <id>2b878296-6343-49e0-80e4-82986a103ea9</id>
      <masked>false</masked>
      <name>payment_instrument</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ecommerce</defaultValue>
      <description></description>
      <id>9e0ec626-0211-46ed-83c3-32246fbeccab</id>
      <masked>false</masked>
      <name>ecommerce</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.atm</defaultValue>
      <description></description>
      <id>7f883608-9248-408f-9db7-93b6db6909c1</id>
      <masked>false</masked>
      <name>atm</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.activate_upon_issue</defaultValue>
      <description></description>
      <id>816fbd62-d314-4fe5-a5c4-4c75cd654223</id>
      <masked>false</masked>
      <name>activate_upon_issue</name>
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
card_product_token = jsonResponse.token
log.logInfo('-----> card_product_token = ' + card_product_token)
GlobalVariable.card_product_token = card_product_token
log.logInfo('----> GlobalVariable card_product_token = ' + GlobalVariable.card_product_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
