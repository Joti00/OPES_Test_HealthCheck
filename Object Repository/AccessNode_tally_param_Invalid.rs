<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AccessNode_tally_param_Invalid</name>
   <tag></tag>
<<<<<<< HEAD
   <elementGuidId>1b013674-d142-4709-8e35-13187a7e4901</elementGuidId>
=======
   <elementGuidId>34c56c9f-3cc5-4588-9c9a-95d0cab00289</elementGuidId>
>>>>>>> 8d345b7ebc916e292692e78c9e1912254c316aab
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
<<<<<<< HEAD
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>X-API-Version</name>
      <type>Main</type>
      <value>2.0</value>
   </httpHeaderProperties>
=======
>>>>>>> 8d345b7ebc916e292692e78c9e1912254c316aab
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://opes-accessnode1.qa.objectcomputing-opes-one.com/accessnode/tally/params</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
<<<<<<< HEAD


WS.verifyResponseStatusCode(response, 401)

assertThat(response.getStatusCode()).isEqualTo(401)</verificationScript>
=======
</verificationScript>
>>>>>>> 8d345b7ebc916e292692e78c9e1912254c316aab
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
