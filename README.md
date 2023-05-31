1. Generate `key.pem` and `cert.pem` using the below command
```
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256
```
2. To remove the password, copy `nopass.pem` to `key.pem` using the below command
```
openssl rsa -in key.pem -out nopass.pem
```
3. Place all the keys inside a directory called `certificates` which is in the root of your project along with `src` directory