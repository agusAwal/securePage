<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>payment</name>
   <tag></tag>
   <elementGuidId>48ab37e6-08e2-4746-9e85-1e9c5697ff55</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;CompanyCode\&quot;: \&quot;${companyCode}\&quot;,\n    \&quot;CustomerNumber\&quot;: \&quot;${customerNumber}\&quot;,\n    \&quot;RequestID\&quot;: \&quot;${requestId}\&quot;,\n    \&quot;ChannelType\&quot;: \&quot;6017\&quot;,\n    \&quot;CustomerName\&quot;: \&quot;Test from SP\&quot;,\n    \&quot;CurrencyCode\&quot;: \&quot;IDR\&quot;,\n    \&quot;PaidAmount\&quot;: \&quot;${amount}\&quot;,\n    \&quot;TotalAmount\&quot;: \&quot;${amount}\&quot;,\n    \&quot;TransactionDate\&quot;: \&quot;21/03/2022 11:05:17\&quot;,\n    \&quot;SubCompany\&quot;: \&quot;00000\&quot;,\n    \&quot;Reference\&quot;: \&quot;2020\&quot;,\n    \&quot;DetailBills\&quot;: [\n        null\n    ],\n    \&quot;FlagAdvice\&quot;: \&quot;N\&quot;,\n    \&quot;AdditionalData\&quot;: \&quot;agus\&quot;\n}&quot;,
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
      <value>Bearer ${accessToken}</value>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://13.228.25.85:8387/v1.0/va/payments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.requestId</defaultValue>
      <description></description>
      <id>1a5ca1a9-6846-4386-bde8-d2798ccc8767</id>
      <masked>false</masked>
      <name>requestId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.amount</defaultValue>
      <description></description>
      <id>4519c1cf-efa2-49ca-889f-66a4ebea922f</id>
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

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
