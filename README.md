# :credit_card: Credit Card Checker
[![Actions Status](https://github.com/williamdes/mariadb-mysql-kbs/workflows/Run%20tests/badge.svg)](https://github.com/oliverborner/Credit-Card-Checker/actions) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://github.com/oliverborner/Credit-Card-Checker/blob/main/LICENSE)

A credit card checker written in Rust  

![alt text](https://github.com/oliverborner/Credit-Card-Checker/blob/main/screenshot.png)

Checks if a card number is valid with the help of the [Luhn algorithm](https://en.wikipedia.org/wiki/Luhn_algorithm)  
and checks also for the card type via regex.  

## Useful links
- [Payment Card Number (IIN)](https://en.wikipedia.org/wiki/Payment_card_number#Issuer_identification_number_(IIN))  
- [Stackoverflow: How to detect credit card based on number](https://stackoverflow.com/questions/72768/how-do-you-detect-credit-card-type-based-on-number)  


## Usage
```
cargo run  
```
### Testnumbers
<table>
  <tbody>
    <tr>
      <td>371449635398431</td>
      <td>American Express</td>
    </tr>
    <tr>
      <td>30569309025904</td>
      <td>Diners Club</td>
    </tr>
    <tr>
      <td>3530111333300000</td>
      <td>JCB</td>
    </tr>
    <tr>
      <td>5555555555554444</td>
      <td>Mastercard</td>
    </tr>
    <tr>
      <td>4111111111111111</td>
      <td>Visa</td>
    </tr>
  </tbody>
</table>	

## Dependencies  
- colored = "2.0.0"  
- regex = "1"  
<br />


