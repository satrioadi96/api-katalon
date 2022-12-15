<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PUT_UpdateBooking</name>
   <tag></tag>
   <elementGuidId>ed58c6f9-5920-47e6-8a59-ba32a873c007</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cbooking\u003e\n    \u003cfirstname\u003eJames\u003c/firstname\u003e\n    \u003clastname\u003eBrown\u003c/lastname\u003e\n    \u003ctotalprice\u003e111\u003c/totalprice\u003e\n    \u003cdepositpaid\u003etrue\u003c/depositpaid\u003e\n    \u003cbookingdates\u003e\n      \u003ccheckin\u003e2018-01-01\u003c/checkin\u003e\n      \u003ccheckout\u003e2019-01-01\u003c/checkout\u003e\n    \u003c/bookingdates\u003e\n    \u003cadditionalneeds\u003eBreakfast\u003c/additionalneeds\u003e\n  \u003c/booking\u003e&quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
      <webElementGuid>94f963d0-2230-41c2-883e-6538410bf350</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://restful-booker.herokuapp.com/booking/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyResponseStatusCode(response, 403)

assertThat(response.getStatusCode()).isEqualTo(403)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
