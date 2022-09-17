# acme-proto

An asynchronous ACME (RFC 8555) library that handles the client part of the protocol. It is not a client by itself, it is meant to be used by a client.

If you are looking for a full client, have a look at [ACMEd](https://github.com/breard-r/acmed).


## Specifications compliance

acme-proto supports strict compliance to the specifications. Unfortunately, some server software does not and therefore acme-proto has to adapt. This is done via the following features:

- `opt_account_orders`: Boulder does not include the mandatory `orders` parameter in the account object (see [issue #3335](https://github.com/letsencrypt/boulder/issues/3335)).

If you have any knowledge on other non-compliant servers that should be handled, please [open an issue](https://github.com/breard-r/acme-proto/issues/new).
