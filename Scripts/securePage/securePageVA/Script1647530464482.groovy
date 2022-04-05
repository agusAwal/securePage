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

//WebUI.callTestCase(findTestCase('securePage/generateUrl'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.openBrowser('')

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.url)

WebUI.waitForElementPresent(findTestObject('Object Repository/landingPage/assertObjRingkasan'), 50)

WebUI.verifyTextPresent('Ringkasan Pembayaran', false)

WebUI.click(findTestObject('Object Repository/landingPage/btn-va'))

WebUI.click(findTestObject('Object Repository/landingPage/label-'+GlobalVariable.bankName))

WebUI.click(findTestObject('Object Repository/landingPage/btn-lanjutkan'))

WebUI.waitForElementPresent(findTestObject('landingPage/assertObjVa'+GlobalVariable.bankName), 50)

WebUI.verifyTextPresent(GlobalVariable.bankName.toUpperCase()+' Virtual Account', false)

def nomerVA = WebUI.getText(findTestObject('Object Repository/landingPage/label-noVa'))

print(nomerVA)

def nomerVaLeng = nomerVA.length()

print(nomerVaLeng)

def companyCode = nomerVA[(0..4)]

print(companyCode)

def customerNumber = nomerVA[(5..nomerVaLeng - 1)]

print(customerNumber)

def response = WS.sendRequest(findTestObject('paymentVa/token'))

print(response)

def slurper = new groovy.json.JsonSlurper()

def accessTK = slurper.parseText(response.getResponseBodyContent())

print(accessTK)

def accessToken = accessTK.access_token

print(accessToken)

def inquery = WS.sendRequest(findTestObject('paymentVa/inquery', [('accessToken') : accessToken, ('companyCode') : companyCode
            , ('customerNumber') : customerNumber, ('requestID') : GlobalVariable.requestId]))

print(inquery)

def resPayment =  WS.sendRequest(findTestObject('paymentVa/payment', [('accessToken') : accessToken, ('companyCode') : companyCode
            , ('customerNumber') : customerNumber, ('requestID') : GlobalVariable.requestId, ('amount') : GlobalVariable.amount]))

print(resPayment)



