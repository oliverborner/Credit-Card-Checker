# Credit Card Checker
### A credit card checker written in Rust  

![alt text](https://github.com/oliverborner/Credit-Card-Checker/blob/main/screenshot.png)

---
Checks if a card number is valid with the help of the  
[Luhn algorithm] (https://en.wikipedia.org/wiki/Luhn_algorithm)  

checks also for the card type with regex  
[Payment Card Number (IIN)](https://en.wikipedia.org/wiki/Payment_card_number#Issuer_identification_number_(IIN)) 
[Stackoverflow: How to detect credit card based on number](https://stackoverflow.com/questions/72768/how-do-you-detect-credit-card-type-based-on-number)  
<br />


```
cargo run  
```

### Dependencies  
- colored = "2.0.0"  
- regex = "1"  
