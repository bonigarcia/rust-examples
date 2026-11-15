from selenium import webdriver

def hello_world_python():
    driver = webdriver.Chrome()
    driver.get("https://www.selenium.dev/")
    driver.quit()