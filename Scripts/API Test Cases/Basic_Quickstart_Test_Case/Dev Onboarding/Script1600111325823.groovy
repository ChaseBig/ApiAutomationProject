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

CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.pingService'()

String card_product_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCardProduct'(start_date, name, 
    GlobalVariable.payment_instrument, GlobalVariable.ecommerce, GlobalVariable.activate_upon_issue, GlobalVariable.atm)

String program_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCardFundingSource'(GlobalVariable.program_name)

String user_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createUser'(first_name, last_name, GlobalVariable.active)

String card_token = CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createCard'(user_token, card_product_token)

//CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createInvalidUser'(GlobalVariable.first_name, GlobalVariable.last_name, 
//    GlobalVariable.active, '😜')
//
//CustomKeywords.'api_Basic_Onboarding.api_basic_onboarding.createInvalidCard'(user_token, card_product_token, '😜')

