package detailed_Onboarding

import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows

import internal.GlobalVariable

import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.logging.KeywordLogger
import java.lang.StringBuilder as StringBuilder
import com.kms.katalon.core.testobject.RequestObject

import groovy.json.JsonSlurper


public class detailed_onboarding {
	public static KeywordLogger log = new KeywordLogger()
	public static JsonSlurper jsonSlurper = new JsonSlurper()

	@Keyword
	def createCardProduct(String start_date, String name, String payment_instrument, boolean ecommerce, boolean activate_upon_issue) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/CardProducts/create cardproducts",
				[('start_date') : start_date, 'name' : name, 'payment_instrument' : payment_instrument, 'ecommerce' : ecommerce, 'activate_upon_issue' : activate_upon_issue]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def card_product_token = jsonResponse.token
		log.logInfo('-----> card_product_token = ' + card_product_token)
		GlobalVariable.card_product_token = card_product_token
		log.logInfo('----> GlobalVariable card_product_token = ' + GlobalVariable.card_product_token)
		return jsonResponse.token
	}

	@Keyword
	def updateCardProduct(String card_product_token) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/CardProducts/update cardProducts",
				['token' : card_product_token]))
	}

	@Keyword
	def createCardFundingSource(String program_name) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/FundingSources/create fundingsources program",
				['name' : program_name]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def funding_source_token = jsonResponse.token
		log.logInfo('-----> funding_source_token = ' + funding_source_token)
		GlobalVariable.funding_source_token = funding_source_token
		log.logInfo('----> GlobalVariable funding_source_token = ' + GlobalVariable.funding_source_token)
		return jsonResponse.token
	}

	@Keyword
	def createDetailedUser(String user_first_name, String user_middle_name, String user_last_name, boolean active, String user_password, String user_phone, String gender, String user_email, String ssn_expiration_date, String user_id_type,
			String user_address1, String city, String state, String country, String birth_date, boolean corporate_card_holder, String user_ssn, String id_card_number, String id_card_expiration_date,
			String nationality, String company, boolean uses_parent_account, String user_postal_code) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Users/create user basic",
				['first_name' : user_first_name, 'middle_name' : user_middle_name, 'last_name' : user_last_name, 'active' : active,
					'password' : user_password, 'phone' : user_phone, 'gender' : gender, 'email' : user_email, 'expiration_date' : ssn_expiration_date, 'type' : user_id_type,
					'address1' : user_address1, 'city' : city, 'state' : state, 'country' : country, 'birth_date' : birth_date,
					'corporate_card_holder' : corporate_card_holder, 'ssn' : user_ssn, 'id_card_number' : id_card_number, 'id_card_expiration_date' : id_card_expiration_date,
					'nationality' : nationality, 'company' : company, 'uses_parent_account' : uses_parent_account, 'postal_code' : user_postal_code
				]
				)
				)

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def user_token = jsonResponse.token
		log.logInfo('-----> user_token = ' + user_token)
		GlobalVariable.user_token = user_token
		log.logInfo('----> GlobalVariable user_token = ' + GlobalVariable.user_token)
		return jsonResponse.token
	}

	@Keyword
	def createCard(String user_token, String card_product_token) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Cards/create card",
				['user_token' : user_token, 'card_product_token' : card_product_token]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def card_token = jsonResponse.token
		log.logInfo('-----> card_token = ' + card_token)
		GlobalVariable.card_token = card_token
		log.logInfo('----> GlobalVariable card_token = ' + GlobalVariable.card_token)
		return jsonResponse.token
	}

	@Keyword
	def createGpaOrder(String user_token, String gpaamount, String currency_code, String funding_source_token) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Gpa/create gpaorder",
				['user_token' : user_token, 'amount' : gpaamount, 'currency_code' : currency_code, 'funding_source_token' : funding_source_token]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def gpa_token = jsonResponse.token
		log.logInfo('-----> gpa_token = ' + gpa_token)
		GlobalVariable.gpa_token = gpa_token
		log.logInfo('----> GlobalVariable gpa_token = ' + GlobalVariable.gpa_token)
		return jsonResponse.token
	}

	@Keyword
	def createTransAuth(String card_token, String trans_amount1, String mid, String beeceptor_create_transaction, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create simulateAuthorization",
				['card_token' : card_token, 'amount' : trans_amount1, 'mid' : mid, 'endpoint' : beeceptor_create_transaction, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def original_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> original_transaction_token = ' + original_transaction_token)
		GlobalVariable.original_transaction_token = original_transaction_token
		log.logInfo('----> GlobalVariable original_transaction_token = ' + GlobalVariable.original_transaction_token)
		return jsonResponse.transaction.token
	}

	@Keyword
	def createTransAuthAdvice(String original_transaction_token, String advice_amount, String mid, String beeceptor_create_auth_advice, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create simulate authorize advice",
				['original_transaction_token' : original_transaction_token, 'amount' : advice_amount, 'mid' : mid, 'endpoint' : beeceptor_create_auth_advice, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def advice_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> advice_transaction_token = ' + advice_transaction_token)
		GlobalVariable.advice_transaction_token = advice_transaction_token
		log.logInfo('----> GlobalVariable advice_transaction_token = ' + GlobalVariable.advice_transaction_token)
		return jsonResponse.transaction.token
	}
	
	@Keyword
	def createTransFinancialWithdrawal(String card_token, String trans_amount1, String mid, String beeceptor_create_financial_withdrawal, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create simulate financial withdrawal",
				['card_token' : card_token, 'amount' : trans_amount1, 'mid' : mid, 'endpoint' : beeceptor_create_financial_withdrawal, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def original_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> original_transaction_token = ' + original_transaction_token)
		GlobalVariable.original_transaction_token = original_transaction_token
		log.logInfo('----> GlobalVariable original_transaction_token = ' + GlobalVariable.original_transaction_token)
		return jsonResponse.transaction.token
	}

	@Keyword
	def createTransClearing(String original_transaction_token, String trans_amount1, String mid, boolean force_post, String beeceptor_settle_transaction, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create simulate clearing",
				['original_transaction_token' : original_transaction_token, 'amount' : trans_amount1, 'mid' : mid, 'endpoint' : beeceptor_settle_transaction,
					'force_post' : force_post, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def cleared_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> cleared_transaction_token = ' + cleared_transaction_token)
		GlobalVariable.cleared_transaction_token = cleared_transaction_token
		log.logInfo('----> GlobalVariable cleared_transaction_token = ' + GlobalVariable.cleared_transaction_token)
		return jsonResponse.transaction.token
	}

	@Keyword
	def createTransClearingRefund(String original_transaction_token, String trans_amount1, String mid, boolean is_refund, String beeceptor_create_refund, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create simulate clearing refund",
				['original_transaction_token' : original_transaction_token, 'amount' : trans_amount1, 'mid' : mid, 'endpoint' : beeceptor_create_refund,
					'is_refund' : is_refund, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def refund_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> refund_transaction_token = ' + refund_transaction_token)
		GlobalVariable.refund_transaction_token = refund_transaction_token
		log.logInfo('----> GlobalVariable refund_transaction_token = ' + GlobalVariable.refund_transaction_token)
		return jsonResponse.transaction.token
	}

	@Keyword
	def createTransAuthDeclined(String card_token, String trans_amount1, String mid, String beeceptor_declined_transaction, String beeceptor_username, String beeceptor_password) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Transactions/create declined simulateAuthorization",
				['card_token' : card_token, 'amount' : trans_amount1, 'mid' : mid, 'endpoint' : beeceptor_declined_transaction, 'username' : beeceptor_username, 'password' : beeceptor_password]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def original_transaction_token = jsonResponse.transaction.token
		log.logInfo('-----> original_transaction_token = ' + original_transaction_token)
		GlobalVariable.original_transaction_token = original_transaction_token
		log.logInfo('----> GlobalVariable original_transaction_token = ' + GlobalVariable.original_transaction_token)
		return jsonResponse.transaction.token
	}

	@Keyword
	def createCardTransition(String card_token, String state, String channel, String reason_code) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/CardTransitions/create cardtransitions",
				['card_token' : card_token, 'state' : state, 'channel' : channel, 'reason_code' : reason_code]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def transition_reason = jsonResponse.reason
		log.logInfo('-----> transition_reason = ' + transition_reason)
		def transitionState = jsonResponse.state
		log.logInfo('-----> transitionState = ' + transitionState)
		return jsonResponse.reason
	}
}