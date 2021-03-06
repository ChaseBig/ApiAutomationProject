<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>create merchant</name>
   <tag></tag>
   <elementGuidId>dd70dc7f-9af6-497a-92aa-7db55ede7ba9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;active\&quot;: ${active},\n    \&quot;contact\&quot;: \&quot;${contact}\&quot;,\n    \&quot;contact_email\&quot;: \&quot;${contact_email}\&quot;,\n    \&quot;address1\&quot;: \&quot;${address1}\&quot;,\n    \&quot;city\&quot;: \&quot;${city}\&quot;,\n    \&quot;state\&quot;: \&quot;${state}\&quot;,\n    \&quot;zip\&quot;: \&quot;${zip}\&quot;,\n    \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n    \&quot;country\&quot;: \&quot;${country}\&quot;,\n    \&quot;partial_auth_flag\&quot;: ${partial_auth_flag}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>accept</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic NmI4MGEzODMtNTZhYi00ZjRkLWFkYzItYTk1ZDFiNzVlODgxOjZlMGUzMTQ3LTljZWEtNDIwNS1hOWUzLTUyMDBiZGZjMTM4Zg==</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://${url}/merchants</restUrl>
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
      <id>a930ccc3-e06e-4dcd-bba6-0e4a6aea5f13</id>
      <masked>false</masked>
      <name>url</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.name</defaultValue>
      <description></description>
      <id>faadd178-2cfd-40cb-8cb1-890d400559e4</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.active</defaultValue>
      <description></description>
      <id>44464d0f-1f5a-441e-bb3f-0d55ea03ce81</id>
      <masked>false</masked>
      <name>active</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.contact</defaultValue>
      <description></description>
      <id>1bae8090-5086-426f-a398-b21662ed3f64</id>
      <masked>false</masked>
      <name>contact</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.contact_email</defaultValue>
      <description></description>
      <id>2c5dc047-86df-4587-866e-9f50148bb514</id>
      <masked>false</masked>
      <name>contact_email</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.address1</defaultValue>
      <description></description>
      <id>326b869c-634e-4b77-83d1-618891be7965</id>
      <masked>false</masked>
      <name>address1</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.city</defaultValue>
      <description></description>
      <id>73c95536-dc61-486a-8018-94ad0b737623</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.state</defaultValue>
      <description></description>
      <id>3117a85d-b329-4496-a35b-9829630ea1ab</id>
      <masked>false</masked>
      <name>state</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.zip</defaultValue>
      <description></description>
      <id>eea2c622-8c6a-43c2-b6e6-1d117e3e0e6a</id>
      <masked>false</masked>
      <name>zip</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.phone</defaultValue>
      <description></description>
      <id>af23034e-3853-433b-a282-826dc718b789</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.country</defaultValue>
      <description></description>
      <id>e6b16ab0-998c-418b-a9ae-f055774ddcc9</id>
      <masked>false</masked>
      <name>country</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.partial_auth_flag</defaultValue>
      <description></description>
      <id>bdc6209a-3467-4fc8-9a10-40b96c96fe20</id>
      <masked>false</masked>
      <name>partial_auth_flag</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
