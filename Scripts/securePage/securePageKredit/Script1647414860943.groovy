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

WebUI.callTestCase(findTestCase('securePage/generateUrl'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.openBrowser("")

WebUI.maximizeWindow()

WebUI.navigateToUrl(GlobalVariable.url)

WebUI.waitForElementPresent(findTestObject('Object Repository/landingPage/assertObjRingkasan'), 50)

WebUI.verifyTextPresent('Ringkasan Pembayaran', false)

WebUI.click(findTestObject('Object Repository/landingPage/btn-kredit'))

WebUI.click(findTestObject('Object Repository/landingPage/btn-pembayaraPenuh'))

WebUI.click(findTestObject('Object Repository/landingPage/btn-lanjutkan'))

WebUI.waitForElementPresent(findTestObject('Object Repository/landingPage/assertObjKredit'), 50)

WebUI.verifyTextPresent('Kartu Kredit', false)

WebUI.setText(findTestObject('Object Repository/landingPage/inpt-cardNo'), '4137198118415892')

WebUI.setText(findTestObject('Object Repository/landingPage/inpt-cardName'), 'agus')

WebUI.selectOptionByValue(findTestObject('Object Repository/landingPage/select-expiryDateMonth'), '12', false)

WebUI.selectOptionByValue(findTestObject('Object Repository/landingPage/select-expiryDateYear'), '2025', false)

WebUI.setText(findTestObject('Object Repository/landingPage/inpt-ccvNo'), '222')

WebUI.click(findTestObject('Object Repository/landingPage/btn-bayar'))

