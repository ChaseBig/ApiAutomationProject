<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create user detailed</name>
   <tag></tag>
   <elementGuidId>0239e110-8e49-4234-a9dc-5a2c977ad5f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;active\&quot;: true,\n  \&quot;notes\&quot;: \&quot;${notes}\&quot;,\n  \&quot;password\&quot;: \&quot;${password}\&quot;,\n  \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n  \&quot;identifications\&quot;: [\n    {\n      \&quot;type\&quot;: \&quot;${type}\&quot;,\n      \&quot;expiration_date\&quot;: \&quot;${expiration_date}\&quot;\n    }\n  ],\n  \&quot;gender\&quot;: \&quot;${gender}\&quot;,\n  \&quot;first_name\&quot;: \&quot;${first_name}\&quot;,\n  \&quot;middle_name\&quot;: \&quot;${middle_name}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${last_name}\&quot;,\n  \&quot;email\&quot;: \&quot;${email}\&quot;,\n  \&quot;address1\&quot;: \&quot;${address1}\&quot;,\n  \&quot;city\&quot;: \&quot;${city}\&quot;,\n  \&quot;state\&quot;: \&quot;${state}\&quot;,\n  \&quot;country\&quot;: \&quot;${country}\&quot;,\n  \&quot;birth_date\&quot;: \&quot;${birth_date}\&quot;,\n  \&quot;corporate_card_holder\&quot;: false,\n  \&quot;ssn\&quot;: \&quot;${ssn}\&quot;,\n  \&quot;id_card_number\&quot;: \&quot;${id_card_number}\&quot;,\n  \&quot;id_card_expiration_date\&quot;: \&quot;${id_card_expiration_date}\&quot;,\n  \&quot;nationality\&quot;: \&quot;${nationality}\&quot;,\n  \&quot;company\&quot;: \&quot;${company}\&quot;,\n  \&quot;uses_parent_account\&quot;: false,\n  \&quot;postal_code\&quot;: \&quot;${postal_code}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic NmI4MGEzODMtNTZhYi00ZjRkLWFkYzItYTk1ZDFiNzVlODgxOjZlMGUzMTQ3LTljZWEtNDIwNS1hOWUzLTUyMDBiZGZjMTM4Zg==</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}/users</restUrl>
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
      <id>4a84651f-16d1-4a40-b417-9d386084be0f</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.notes</defaultValue>
      <description></description>
      <id>ffb0ca2d-67a4-4912-afa3-7428f83874e9</id>
      <masked>false</masked>
      <name>notes</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.password</defaultValue>
      <description></description>
      <id>612ecd90-bbdc-4b38-8e81-21b2f4447658</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.phone</defaultValue>
      <description></description>
      <id>e11a0cda-ee9f-415b-b92c-e379b9a163b8</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.type</defaultValue>
      <description></description>
      <id>cad3ff39-98c6-42b1-87b9-7daf0d0ab54a</id>
      <masked>false</masked>
      <name>type</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.expiration_date</defaultValue>
      <description></description>
      <id>9ec4986e-60c8-4b8f-b787-112e3685fa19</id>
      <masked>false</masked>
      <name>expiration_date</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.gender</defaultValue>
      <description></description>
      <id>c6174e4c-4a84-4241-bd51-10aebc3eca97</id>
      <masked>false</masked>
      <name>gender</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.first_name</defaultValue>
      <description></description>
      <id>8004bd6e-e0f1-42f6-a89b-97ca65514772</id>
      <masked>false</masked>
      <name>first_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.middle_name</defaultValue>
      <description></description>
      <id>0ffc7dad-85f1-4fba-817e-79e3d98e6080</id>
      <masked>false</masked>
      <name>middle_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.last_name</defaultValue>
      <description></description>
      <id>6b1bcc5b-08cb-451c-959d-19fb75b4bc47</id>
      <masked>false</masked>
      <name>last_name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.email</defaultValue>
      <description></description>
      <id>152dde9f-a87a-4070-9740-560b16d168fe</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.address1</defaultValue>
      <description></description>
      <id>ef8ae46e-e477-4ccc-a3ae-0e65090b9e6e</id>
      <masked>false</masked>
      <name>address1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.city</defaultValue>
      <description></description>
      <id>a9c68ecb-9e2a-4cd2-8443-39487ac830a2</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.state</defaultValue>
      <description></description>
      <id>64aa0207-76c2-4a15-b07c-cce633eea273</id>
      <masked>false</masked>
      <name>state</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.country</defaultValue>
      <description></description>
      <id>c02889f4-a18b-4ee0-964c-2b5bd404b280</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.birth_date</defaultValue>
      <description></description>
      <id>2860bd1d-3940-4edb-9fd5-72f8f18e948e</id>
      <masked>false</masked>
      <name>birth_date</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.ssn</defaultValue>
      <description></description>
      <id>af5f16f5-6e88-4074-b8ec-e7037a994bf6</id>
      <masked>false</masked>
      <name>ssn</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_card_number</defaultValue>
      <description></description>
      <id>67215d6a-d860-4400-a469-1a093f20cb6d</id>
      <masked>false</masked>
      <name>id_card_number</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.id_card_expiration_date</defaultValue>
      <description></description>
      <id>9b999fc1-9d1b-4baf-b5f5-475540f38f9a</id>
      <masked>false</masked>
      <name>id_card_expiration_date</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.nationality</defaultValue>
      <description></description>
      <id>af5f8a3f-c14c-4be2-8473-674b86a57084</id>
      <masked>false</masked>
      <name>nationality</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.company</defaultValue>
      <description></description>
      <id>9166c4c9-de30-4465-a139-7d6da30d81ab</id>
      <masked>false</masked>
      <name>company</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.postal_code</defaultValue>
      <description></description>
      <id>a114f696-a8fe-40cd-8a1e-0d4eb99e2671</id>
      <masked>false</masked>
      <name>postal_code</name>
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
user_token = jsonResponse.token
log.logInfo('-----> user_token = ' + user_token)
GlobalVariable.user_token = user_token
log.logInfo('----> GlobalVariable user_token = ' + GlobalVariable.user_token)
assert response.getStatusCode() == 201</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
