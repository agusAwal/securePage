<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getUrl</name>
   <tag></tag>
   <elementGuidId>867409a9-1133-4d89-9f13-69a2dd7f18f7</elementGuidId>
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
      <name>Signature</name>
      <type>Main</type>
      <value>${signature}</value>
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
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic ${merchantId}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://54.169.81.53:8955/payment-services/v2.1.0/api/token</restUrl>
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
      <id>f6c625ed-9f28-4890-8449-7a4e0d2870d8</id>
      <masked>false</masked>
      <name>orderId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantName</defaultValue>
      <description></description>
      <id>7b024fc6-39e8-48fd-9acd-e0e5abbf0e34</id>
      <masked>false</masked>
      <name>merchantName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.amount</defaultValue>
      <description></description>
      <id>09a5f494-718b-462b-a0dd-8d2480609700</id>
      <masked>false</masked>
      <name>amount</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.merchantId</defaultValue>
      <description></description>
      <id>4089427b-456f-4ba0-882d-9df3f9fb4f62</id>
      <masked>false</masked>
      <name>merchantId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
