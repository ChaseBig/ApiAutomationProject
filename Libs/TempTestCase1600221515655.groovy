import com.kms.katalon.core.main.TestCaseMain
import com.kms.katalon.core.logging.KeywordLogger
import com.kms.katalon.core.testcase.TestCaseBinding
import com.kms.katalon.core.driver.internal.DriverCleanerCollector
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.webui.contribution.WebUiDriverCleaner
import com.kms.katalon.core.mobile.contribution.MobileDriverCleaner
import com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner
import com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner
import com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner


DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.webui.contribution.WebUiDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.mobile.contribution.MobileDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.cucumber.keyword.internal.CucumberDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.windows.keyword.contribution.WindowsDriverCleaner())
DriverCleanerCollector.getInstance().addDriverCleaner(new com.kms.katalon.core.testng.keyword.internal.TestNGDriverCleaner())


RunConfiguration.setExecutionSettingFile('/var/folders/7w/4sgm67v15656fcfvt5x_l9b40000gp/T/Katalon/Test Cases/API Test Cases/Quickstart_with_transactions/Transaction Tests/Financial Settled Transaction/Detailed_onboarding_with_financial_transactions/20200915_205835/execution.properties')

TestCaseMain.beforeStart()

        TestCaseMain.runTestCase('Test Cases/API Test Cases/Quickstart_with_transactions/Transaction Tests/Financial Settled Transaction/Detailed_onboarding_with_financial_transactions', new TestCaseBinding('Test Cases/API Test Cases/Quickstart_with_transactions/Transaction Tests/Financial Settled Transaction/Detailed_onboarding_with_financial_transactions',[:]), FailureHandling.STOP_ON_FAILURE , false)
    
