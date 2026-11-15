require 'selenium-webdriver'

def hello_world_ruby
    driver = Selenium::WebDriver.for :chrome
    driver.navigate.to 'https://www.selenium.dev/'
    driver.quit
end