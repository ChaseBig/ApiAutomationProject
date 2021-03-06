import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.logging.KeywordLogger as KeywordLogger
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

'Create CardProduct'
String card_product_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCardProduct'(GlobalVariable.start_date, 
    GlobalVariable.card_product_name, GlobalVariable.payment_instrument, GlobalVariable.ecommerce, GlobalVariable.activate_upon_issue, 
    GlobalVariable.atm)

'Create Program Funding Source'
String program_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCardFundingSource'(GlobalVariable.program_name)

'Create Basic User'
String user_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createUser'(GlobalVariable.first_name, GlobalVariable.last_name, 
    GlobalVariable.active)

'Create Card'
String card_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCard'(user_token, card_product_token)

