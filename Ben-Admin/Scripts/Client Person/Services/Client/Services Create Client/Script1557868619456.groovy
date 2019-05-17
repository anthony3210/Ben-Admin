import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

response = WS.sendRequest(findTestObject('Client Person/Services/Client/V1/CLIENT', [('legalClientName') : legalClientName, ('clientName') : clientName
            , ('primaryContactEmail') : 'hsanderstest@test.com', ('parentClientUUID') : parentClientUUID
            , ('sicCode') : sicCode, ('clientEffectiveDate') : clientEffectiveDate, ('clientTerminationDate') : clientTerminationDate, ('streetAddressUUID') : streetAddressUUID
            , ('billingAddressUUID') : billingAddressUUID, ('subDomain') : subDomain, ('memberUniqueIdentifier') : memberUniqueIdentifier
            , ('inviteKey') : inviteKey, ('billingContactEmail') : billingContactEmail, ('configurationDemo') : configurationDemo, ('configurationInActive') : configurationInActive
            , ('configurationTemplate') : configurationTemplate, ('primaryContactName') : primaryContactName, ('primaryContactPhone') :primaryContactPhone
            , ('billingContactName') : billingContactName, ('billingContactPhone') : billingContactPhone, ('clientUUID') : '', ('memberEmailCommunication') : memberEmailCommunication]))

WS.verifyResponseStatusCode(response, 201)

WS.verifyElementPropertyValue(response, 'legalClientName', legalClientName)