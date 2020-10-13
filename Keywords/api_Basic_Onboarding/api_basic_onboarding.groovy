package api_Basic_Onboarding
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.util.KeywordUtil
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.logging.KeywordLogger
import java.lang.StringBuilder as StringBuilder
import com.kms.katalon.core.testobject.RequestObject
import internal.GlobalVariable as GlobalVariable

import groovy.json.JsonSlurper


class api_basic_onboarding {
	public static KeywordLogger log = new KeywordLogger()
	public static JsonSlurper jsonSlurper = new JsonSlurper()

	@Keyword
	def createCardProduct(String start_date, String name, String payment_instrument, boolean ecommerce, boolean activate_upon_issue, boolean atm) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/CardProducts/create cardproducts",
				[('start_date') : start_date, 'name' : name, 'payment_instrument' : payment_instrument, 'ecommerce' : ecommerce, 'activate_upon_issue' : activate_upon_issue, 'atm' : atm]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def card_product_token = jsonResponse.token
		log.logInfo('-----> card_product_token = ' + card_product_token)
		GlobalVariable.card_product_token = card_product_token
		log.logInfo('----> GlobalVariable card_product_token = ' + GlobalVariable.card_product_token)
		return jsonResponse.token
	}

	@Keyword
	def createCardFundingSource(String program_name) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/FundingSources/create fundingsources program",
				['name' : program_name]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def funding_token = jsonResponse.token
		log.logInfo('-----> funding_token = ' + funding_token)
		GlobalVariable.funding_token = funding_token
		log.logInfo('----> GlobalVariable card_product_token = ' + GlobalVariable.funding_token)
		return jsonResponse.token
	}

	@Keyword
	def createUser(String first_name, String last_name, boolean active) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Users/create user basic",
				['first_name' : first_name, 'last_name' : last_name, 'active' : active]))

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
	def pingService() {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Ping/ping"))
		
		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def success_value = jsonResponse.success
		log.logInfo('-----> success_value = ' + success_value)
	}
	
	@Keyword
	def createInvalidUser(String first_name, String last_name, boolean active, String nationality) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Users/create user basic",
				['first_name' : first_name, 'last_name' : last_name, 'active' : active, 'nationality' : nationality]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def user_error_message = jsonResponse.error_message
		log.logInfo('-----> user_error_message = ' + user_error_message)
		return jsonResponse.error_message
	}

	@Keyword
	def createInvalidCard(String user_token, String card_product_token, String additionalProp1) {
		def response = WS.sendRequestAndVerify(findTestObject("Postman/Cards/create card invalid json",
				['user_token' : user_token, 'card_product_token' : card_product_token, 'additionalProp1' : additionalProp1]))

		def jsonResponse = jsonSlurper.parseText(response.getResponseText())
		def card_error_message = jsonResponse.error_message
		log.logInfo('-----> card_error_message = ' + card_error_message)
		return jsonResponse.error_message
	}
}