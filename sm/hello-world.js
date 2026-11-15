const { Builder } = require('selenium-webdriver');

async function helloWorldJavaScript() {
    let driver = await new Builder().forBrowser('chrome').build();
    await driver.get('https://www.selenium.dev/');
    await driver.quit();
}