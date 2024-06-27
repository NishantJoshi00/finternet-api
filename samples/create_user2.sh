#!/bin/bash


curl -X POST -H "Content-Type: application/json" -d '{
  "email": "123@abc.xyz",
  "name": "John Doe",
  "public_key": "1111111",
  "ua_addr": "user2"
}' http://localhost:8080/v1/users -v
