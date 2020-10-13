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
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable

String card_product_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createCardProduct'(GlobalVariable.start_date, 
    GlobalVariable.card_product_name, GlobalVariable.payment_instrument, GlobalVariable.ecommerce, GlobalVariable.activate_upon_issue)

String funding_source_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createCardFundingSource'(GlobalVariable.program_name)

String user_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createDetailedUser'(GlobalVariable.user_first_name, 
    GlobalVariable.user_middle_name, GlobalVariable.user_last_name, GlobalVariable.active, GlobalVariable.user_password, 
    GlobalVariable.phone, GlobalVariable.gender, GlobalVariable.user_email, GlobalVariable.ssn_expiration_date, GlobalVariable.user_id_type, 
    GlobalVariable.user_address1, GlobalVariable.city, GlobalVariable.user_state, GlobalVariable.country, GlobalVariable.birth_date, 
    GlobalVariable.corporate_card_holder, GlobalVariable.user_ssn, GlobalVariable.id_card_number, GlobalVariable.id_card_expiration_date, 
    GlobalVariable.nationality, GlobalVariable.company, GlobalVariable.uses_parent_account, GlobalVariable.user_postal_code)

String card_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createCard'(user_token, card_product_token)

String gpa_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createGpaOrder'(user_token, GlobalVariable.gpaamount, 
    GlobalVariable.currency_code, funding_source_token)

CustomKeywords.'detailed_Onboarding.detailed_onboarding.createCardTransition'(card_token, 'SUSPENDED', GlobalVariable.channel, 
    '01')

String original_transaction_token = CustomKeywords.'detailed_Onboarding.detailed_onboarding.createTransAuthDeclined'(card_token, 
    GlobalVariable.trans_amount1, GlobalVariable.mid, GlobalVariable.beeceptor_declined_transaction, GlobalVariable.beeceptor_username, 
    GlobalVariable.beeceptor_password)

