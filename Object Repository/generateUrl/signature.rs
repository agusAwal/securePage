<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>signature</name>
   <tag></tag>
   <elementGuidId>4a942746-b189-490e-9076-32d293340d32</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customerDetails\&quot;: {\n        \&quot;email\&quot;: \&quot;agus.awaludin@ottodigital.id\&quot;,\n        \&quot;firstName\&quot;: \&quot;agus\&quot;,\n        \&quot;lastName\&quot;: \&quot;awal\&quot;,\n        \&quot;phone\&quot;: \&quot;081386133023\&quot;\n    },\n    \&quot;transactionDetails\&quot;: {\n        \&quot;amount\&quot;: ${amount},\n        \&quot;currency\&quot;: \&quot;IDR\&quot;,\n        \&quot;merchantName\&quot;: \&quot;${merchantName}\&quot;,\n        \&quot;orderId\&quot;: \&quot;${orderId}\&quot;,\n        \&quot;vaOrderId\&quot;: \&quot;\&quot;,\n        \&quot;promoCode\&quot;: \&quot;\&quot;,\n        \&quot;vabca\&quot;: \&quot;\&quot;,\n        \&quot;valain\&quot;: \&quot;\&quot;,\n        \&quot;vamandiri\&quot;: \&quot;\&quot;\n    }\n}&quot;,
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
      <name>Timestamp</name>
      <type>Main</type>
      <value>1614070898</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Apikey</name>
      <type>Main</type>
      <value>${apiKey}</value>
   </httpHeaderProperties>
   <katalonVersion>8.1.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://54.169.81.53:8888/corepg/api/general/createSignature/</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.orderId</defaultValue>
      <description></description>
      <id>4bd0d8e5-3840-4301-9a8b-7fb75117799a</id>
      <masked>false</masked>
      <name>orderId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.apiKey</defaultValue>
      <description></description>
      <id>cd421231-7b17-4030-877a-a8b6721da1c3</id>
      <masked>false</masked>
      <name>apiKey</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantName</defaultValue>
      <description></description>
      <id>7051a055-ea35-40d9-9593-34344224be47</id>
      <masked>false</masked>
      <name>merchantName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.amount</defaultValue>
      <description></description>
      <id>3ff58330-41ec-4393-9156-757b34f7bda3</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject responseToken = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(responseToken, 200)

assertThat(responseToken.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
