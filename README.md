**WIP**

To execute automated suite in EC2 hosted Jenkins Pipeline:
- Navigate to [this EC2 Hosted Jenkins Pipeline](http://34.217.39.76:8080/job/MarqetaAutomationProj-JenkinsOnEC2)
- Run Automation Build by clicking "Build Now" in the left-hand menu
- Executed Jenkins builds will run the current completed testcases below
- If build passes, then tests are passing.
- All simulated transaction requests are communicating with beeceptor via specific webhook endpoints 

Current Progress:
- [x] Ping Sandbox Env to confirm successful heartbeat 
- [x] Create Basic CardProduct Testcase 1 
- [x] Create Basic Program FundingSource Testcase 1 
- [x] Create Basic User Testcase 1 
- [x] Create Basic Card Testcase 1
- [x] Create Basic User Testcase 2
- [x] Create Basic Card Testcase 2
- [x] Create Detailed User Testcase 1
- [x] Create Detailed Card Testcase 1
- [x] Create Detailed User Testcase 2
- [x] Create Detailed Card Testcase 2
- [x] Create Detailed User Testcase 3
- [x] Create Detailed Card Testcase 3
- [x] Create Basic Card Transaction Authorization Testcase 1
- [x] Create Basic Card Transaction Advice Testcase 1
- [x] Create Basic Card Transaction Clearing Testcase 1
- [x] Create Detailed Card Transaction Authorization Testcase 2
- [x] Create Detailed Card Transaction Advice Testcase 2
- [x] Create Detailed Card Transaction Clearing Refund Testcase 2
- [x] Create Detailed User Testcase 4
- [x] Create Detailed Card Testcase 4
- [x] Create CardTransition Testcase (set card to SUSPENDED)
- [x] Create Card Transaction Testcase, transaction expected DECLINED (Due to cardTransition setting card status to SUSPENDED)
- [X] Create Financial Withdrawal Transaction Testcase 1 (atm withdrawal, expect first attempt to decline due to cardproduct having 'atm' set to false)
- [x] Update created cardproduct (set 'atm' to true)
- [X] Create Financial Withdrawal Transaction Testcase 2 (atm withdrawal, succeeds after cardproduct updated to allow atm withdrawals)
- [x] Create Transaction Clearing Testcase for Financial Withdrawal
- [x] Create Transaction Clearing Refund Testcase for Financial Withdrawal

# mq-chase
## SDET Take Home Assessment

Marqeta's core API provides our partners with easy to integrate endpoints for their business models. The creation of cards and using them for transactions. Using the Marqeta Sandbox to simulate the creation of cards and transactions.
Create a new sandbox environment by following the instructions and guide here: https://www.marqeta.com/docs/developer-guides/core-api-quick-start.
- Write at least 5 test cases for card creation, user creation.
- Write at least 2 test cases for transactions.
- Develop a test automation framework for ALL the test cases above.
- Integrate with a CI (Optional)
