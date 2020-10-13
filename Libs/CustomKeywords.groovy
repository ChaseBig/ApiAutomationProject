
/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */

import java.lang.String

import com.kms.katalon.core.testobject.TestObject

import com.applitools.eyes.RectangleSize

import com.applitools.eyes.selenium.Eyes



def static "api_Basic_Onboarding.api_basic_onboarding.createCardProduct"(
    	String start_date	
     , 	String name	
     , 	String payment_instrument	
     , 	boolean ecommerce	
     , 	boolean activate_upon_issue	
     , 	boolean atm	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createCardProduct(
        	start_date
         , 	name
         , 	payment_instrument
         , 	ecommerce
         , 	activate_upon_issue
         , 	atm)
}


def static "api_Basic_Onboarding.api_basic_onboarding.createCardFundingSource"(
    	String program_name	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createCardFundingSource(
        	program_name)
}


def static "api_Basic_Onboarding.api_basic_onboarding.createUser"(
    	String first_name	
     , 	String last_name	
     , 	boolean active	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createUser(
        	first_name
         , 	last_name
         , 	active)
}


def static "api_Basic_Onboarding.api_basic_onboarding.createCard"(
    	String user_token	
     , 	String card_product_token	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createCard(
        	user_token
         , 	card_product_token)
}


def static "api_Basic_Onboarding.api_basic_onboarding.pingService"() {
    (new api_Basic_Onboarding.api_basic_onboarding()).pingService()
}


def static "api_Basic_Onboarding.api_basic_onboarding.createInvalidUser"(
    	String first_name	
     , 	String last_name	
     , 	boolean active	
     , 	String nationality	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createInvalidUser(
        	first_name
         , 	last_name
         , 	active
         , 	nationality)
}


def static "api_Basic_Onboarding.api_basic_onboarding.createInvalidCard"(
    	String user_token	
     , 	String card_product_token	
     , 	String additionalProp1	) {
    (new api_Basic_Onboarding.api_basic_onboarding()).createInvalidCard(
        	user_token
         , 	card_product_token
         , 	additionalProp1)
}


def static "detailed_Onboarding.detailed_onboarding.createCardProduct"(
    	String start_date	
     , 	String name	
     , 	String payment_instrument	
     , 	boolean ecommerce	
     , 	boolean activate_upon_issue	) {
    (new detailed_Onboarding.detailed_onboarding()).createCardProduct(
        	start_date
         , 	name
         , 	payment_instrument
         , 	ecommerce
         , 	activate_upon_issue)
}


def static "detailed_Onboarding.detailed_onboarding.updateCardProduct"(
    	String card_product_token	) {
    (new detailed_Onboarding.detailed_onboarding()).updateCardProduct(
        	card_product_token)
}


def static "detailed_Onboarding.detailed_onboarding.createCardFundingSource"(
    	String program_name	) {
    (new detailed_Onboarding.detailed_onboarding()).createCardFundingSource(
        	program_name)
}


def static "detailed_Onboarding.detailed_onboarding.createDetailedUser"(
    	String user_first_name	
     , 	String user_middle_name	
     , 	String user_last_name	
     , 	boolean active	
     , 	String user_password	
     , 	String user_phone	
     , 	String gender	
     , 	String user_email	
     , 	String ssn_expiration_date	
     , 	String user_id_type	
     , 	String user_address1	
     , 	String city	
     , 	String state	
     , 	String country	
     , 	String birth_date	
     , 	boolean corporate_card_holder	
     , 	String user_ssn	
     , 	String id_card_number	
     , 	String id_card_expiration_date	
     , 	String nationality	
     , 	String company	
     , 	boolean uses_parent_account	
     , 	String user_postal_code	) {
    (new detailed_Onboarding.detailed_onboarding()).createDetailedUser(
        	user_first_name
         , 	user_middle_name
         , 	user_last_name
         , 	active
         , 	user_password
         , 	user_phone
         , 	gender
         , 	user_email
         , 	ssn_expiration_date
         , 	user_id_type
         , 	user_address1
         , 	city
         , 	state
         , 	country
         , 	birth_date
         , 	corporate_card_holder
         , 	user_ssn
         , 	id_card_number
         , 	id_card_expiration_date
         , 	nationality
         , 	company
         , 	uses_parent_account
         , 	user_postal_code)
}


def static "detailed_Onboarding.detailed_onboarding.createCard"(
    	String user_token	
     , 	String card_product_token	) {
    (new detailed_Onboarding.detailed_onboarding()).createCard(
        	user_token
         , 	card_product_token)
}


def static "detailed_Onboarding.detailed_onboarding.createGpaOrder"(
    	String user_token	
     , 	String gpaamount	
     , 	String currency_code	
     , 	String funding_source_token	) {
    (new detailed_Onboarding.detailed_onboarding()).createGpaOrder(
        	user_token
         , 	gpaamount
         , 	currency_code
         , 	funding_source_token)
}


def static "detailed_Onboarding.detailed_onboarding.createTransAuth"(
    	String card_token	
     , 	String trans_amount1	
     , 	String mid	
     , 	String beeceptor_create_transaction	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransAuth(
        	card_token
         , 	trans_amount1
         , 	mid
         , 	beeceptor_create_transaction
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createTransAuthAdvice"(
    	String original_transaction_token	
     , 	String advice_amount	
     , 	String mid	
     , 	String beeceptor_create_auth_advice	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransAuthAdvice(
        	original_transaction_token
         , 	advice_amount
         , 	mid
         , 	beeceptor_create_auth_advice
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createTransFinancialWithdrawal"(
    	String card_token	
     , 	String trans_amount1	
     , 	String mid	
     , 	String beeceptor_create_financial_withdrawal	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransFinancialWithdrawal(
        	card_token
         , 	trans_amount1
         , 	mid
         , 	beeceptor_create_financial_withdrawal
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createTransClearing"(
    	String original_transaction_token	
     , 	String trans_amount1	
     , 	String mid	
     , 	boolean force_post	
     , 	String beeceptor_settle_transaction	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransClearing(
        	original_transaction_token
         , 	trans_amount1
         , 	mid
         , 	force_post
         , 	beeceptor_settle_transaction
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createTransClearingRefund"(
    	String original_transaction_token	
     , 	String trans_amount1	
     , 	String mid	
     , 	boolean is_refund	
     , 	String beeceptor_create_refund	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransClearingRefund(
        	original_transaction_token
         , 	trans_amount1
         , 	mid
         , 	is_refund
         , 	beeceptor_create_refund
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createTransAuthDeclined"(
    	String card_token	
     , 	String trans_amount1	
     , 	String mid	
     , 	String beeceptor_declined_transaction	
     , 	String beeceptor_username	
     , 	String beeceptor_password	) {
    (new detailed_Onboarding.detailed_onboarding()).createTransAuthDeclined(
        	card_token
         , 	trans_amount1
         , 	mid
         , 	beeceptor_declined_transaction
         , 	beeceptor_username
         , 	beeceptor_password)
}


def static "detailed_Onboarding.detailed_onboarding.createCardTransition"(
    	String card_token	
     , 	String state	
     , 	String channel	
     , 	String reason_code	) {
    (new detailed_Onboarding.detailed_onboarding()).createCardTransition(
        	card_token
         , 	state
         , 	channel
         , 	reason_code)
}


def static "com.kms.katalon.keyword.applitools.BasicKeywords.checkWindow"(
    	String testName	) {
    (new com.kms.katalon.keyword.applitools.BasicKeywords()).checkWindow(
        	testName)
}


def static "com.kms.katalon.keyword.applitools.BasicKeywords.checkTestObject"(
    	TestObject testObject	
     , 	String testName	) {
    (new com.kms.katalon.keyword.applitools.BasicKeywords()).checkTestObject(
        	testObject
         , 	testName)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesOpen"(
    	String testName	
     , 	RectangleSize viewportSize	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesOpen(
        	testName
         , 	viewportSize)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesInit"() {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesInit()
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesClose"(
    	Eyes eyes	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesClose(
        	eyes)
}


def static "com.kms.katalon.keyword.applitools.EyesKeywords.eyesOpenWithBaseline"(
    	String baselineName	
     , 	String testName	
     , 	RectangleSize viewportSize	) {
    (new com.kms.katalon.keyword.applitools.EyesKeywords()).eyesOpenWithBaseline(
        	baselineName
         , 	testName
         , 	viewportSize)
}
