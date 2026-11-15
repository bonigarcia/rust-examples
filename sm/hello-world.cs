using OpenQA.Selenium;
using OpenQA.Selenium.Chrome;

public class HelloWorldCSharp
    static void Main(string[] args) {
        IWebDriver driver = new ChromeDriver();
        driver.Navigate().GoToUrl("https://www.selenium.dev/");
        driver.Quit();
    }
}