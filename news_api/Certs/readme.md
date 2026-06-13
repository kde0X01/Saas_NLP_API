# Step followed to generate self certificat

- install the tool mkcert.
- launch this command to create and import self certificat:
> mkcert -key-file key.pem -cert-file cert.pem 127.0.0.1 localhost


# Sources:
- https://github.com/actix/examples/tree/master/https-tls/openssl
- https://hackernoon.com/how-to-get-sslhttps-for-localhost-i11s3342
- https://actix.rs/docs/server/

