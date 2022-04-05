import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys


//CustomKeywords.'connetionDB.connPostgre.connectDB'('13.228.23.160', '5432', 'ottopg', 'ottoagcfg', 'dTj*&56$es')
//
//String ottoPgOrderId = CustomKeywords.'connetionDB.connPostgre.executeQuery'("select tt5.order_id from t_transaction tt5 where tt5.order_id like '%-20%' order by order_id asc limit 1")
//
//CustomKeywords.'connetionDB.connPostgre.closeDatabaseConnection'()
//
//def dataOrderId = ottoPgOrderId.getString("order_id")
//
//print('your order id is ' + ottoPgOrderId)
//
//String[] result = ottoPgOrderId.split('-')
//
//int resultint = Integer.parseInt(result[1])
//
//resultint = (resultint + 1)
//
//def orderid = ((result[0]) + '-') + String.valueOf(resultint)

def response = WS.sendRequest(findTestObject('generateUrl/signature', [('orderId') : GlobalVariable.orderId, ('apiKey') : GlobalVariable.apiKey
            , ('merchantName') : GlobalVariable.merchantName, ('amount') : GlobalVariable.amount]))

print(GlobalVariable.orderId)

print(GlobalVariable.merchantName)

print(GlobalVariable.amount)

print(GlobalVariable.merchantId)

print(GlobalVariable.apiKey)

def slurper = new groovy.json.JsonSlurper()

def signature = slurper.parseText(response.getResponseBodyContent())

print(signature)

getUrl = WS.sendRequest(findTestObject('generateUrl/getUrl', [('signature') : signature, ('orderId') : GlobalVariable.orderId
            , ('merchantName') : GlobalVariable.merchantName, ('amount') : GlobalVariable.amount, ('merchantId') : GlobalVariable.merchantId]))

def getUrlSp = slurper.parseText(getUrl.getResponseBodyContent())

print(getUrlSp)

def baseUrl = getUrlSp.responseData.endpointUrl

GlobalVariable.url = baseUrl

print(baseUrl)


