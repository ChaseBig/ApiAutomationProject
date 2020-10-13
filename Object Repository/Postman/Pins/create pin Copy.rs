<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create pin Copy</name>
   <tag></tag>
   <elementGuidId>3d980e62-e031-4192-b044-92108940f00c</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;control_token\&quot;: \&quot;${control_token}\&quot;,\n  \&quot;pin\&quot;: \&quot;${pin}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://${url}/pins/controltoken</restUrl>
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
      <id>31f776d1-f512-4da4-9e28-f6fb66a9b88c</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.control_token</defaultValue>
      <description></description>
      <id>68d535d5-73c1-4598-ae01-1b2b13f39fbd</id>
      <masked>false</masked>
      <name>control_token</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.pin</defaultValue>
      <description></description>
      <id>9ce1cd1a-bdbc-4783-880b-611088e47496</id>
      <masked>false</masked>
      <name>pin</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
