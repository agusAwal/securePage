package connetionDB

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
import java.sql.Connection
import java.sql.DriverManager
import java.sql.ResultSet
import java.sql.Statement
import internal.GlobalVariable

public class connPostgre {

	private static Connection connection = null;

	//Connect Database
	@Keyword
	def connectDB(String url, String port, String dbname, String username, String password){
		//Load driver class for your specific database type

		String conn = "jdbc:postgresql://" + url + ":" + port + "/" + dbname

		if(connection != null && !connection.close()){
			connection.close()
		}

		connection  = DriverManager.getConnection(conn, username, password)
		return connection
	}

	//Close Connection
	@Keyword
	def closeDatabaseConnection() {
		if(connection != null && !connection.isClosed()){
			connection.close()
		}
		connection = null
	}

	@Keyword

	def executeQuery(String queryString) {

		Statement stm = connection.createStatement()

		ResultSet resultQuery = stm.executeQuery(queryString)

		return resultQuery

	}

}
