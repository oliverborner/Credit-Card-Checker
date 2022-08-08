# Credit Card Checker
#### a credit card checker written in Rust  
![alt text](https://github.com/oliverborner/Credit-Card-Checker/blob/main/screenshot.png)

---
checks if a card number is valid with the help of the  
Luhn algorithm https://en.wikipedia.org/wiki/Luhn_algorithm  

checks also for the card type with regex  
https://en.wikipedia.org/wiki/Payment_card_number  
https://stackoverflow.com/questions/72768/how-do-you-detect-credit-card-type-based-on-number  

---

```
cargo run  
```
---

#### dependencies:  
colored = "2.0.0"  
regex = "1"  
