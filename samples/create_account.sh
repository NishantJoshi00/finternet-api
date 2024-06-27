#!/bin/bash

user_id=$1

curl -X POST -H "Content-Type: application/json" -d '{
  "account_name": "User1Acc",
  "public_key": "0x1231231",
  "ua_addr": "user1"
}' http://localhost:8080/v1/users/$user_id/accounts -v
