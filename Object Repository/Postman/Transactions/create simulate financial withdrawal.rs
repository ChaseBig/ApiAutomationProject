<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create simulate financial withdrawal</name>
   <tag></tag>
   <elementGuidId>c8b69208-03d3-4b8f-879c-bf25b6d2a7f2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;card_token\&quot;: \&quot;${card_token}\&quot;,\n  \&quot;amount\&quot;: \&quot;${amount}\&quot;,\n  \&quot;mid\&quot;: \&quot;${mid}\&quot;,\n  \&quot;account_type\&quot;: \&quot;${accountType}\&quot;,\n  \&quot;webhook\&quot;: {\n    \&quot;endpoint\&quot;: \&quot;${beeceptor_create_financial_withdrawal}\&quot;,\n    \&quot;username\&quot;: \&quot;${beeceptor_username}\&quot;,\n    \&quot;password\&quot;: \&quot;${beeceptor_password}\&quot;\n  }\n}&quot;,
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
   <restUrl>https://${url}/simulate/financial/withdrawal</restUrl>
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
      <id>0fb0b5c5-80d0-474f-bb84-b8b3122cadca</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.card_token</defaultValue>
      <description></description>
      <id>cd88ee95-21cb-4f2b-becd-2b731e1678da</id>
      <masked>false</masked>
      <name>card_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trans_amount1</defaultValue>
      <description></description>
      <id>70b3e5cc-4b72-46d1-b1df-8c6043c8431b</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.mid</defaultValue>
      <description></description>
      <id>86b429f3-6161-474a-910d-fd9d3b0d1d4f</id>
      <masked>false</masked>
      <name>mid</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.accountType</defaultValue>
      <description></description>
      <id>550952ea-5ad9-4fc3-a8b9-c05539c3ab99</id>
      <masked>false</masked>
      <name>accountType</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_create_financial_withdrawal</defaultValue>
      <description></description>
      <id>c099cf8b-ba6b-4880-8956-e7b6370f14aa</id>
      <masked>false</masked>
      <name>beeceptor_create_financial_withdrawal</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_username</defaultValue>
      <description></description>
      <id>94e83380-f043-4924-be27-2537f8f808d4</id>
      <masked>false</masked>
      <name>beeceptor_username</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.beeceptor_password</defaultValue>
      <description></description>
      <id>6649786d-05b1-4221-85fc-8cdb670a63e9</id>
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
