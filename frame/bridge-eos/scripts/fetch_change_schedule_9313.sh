#!/bin/bash

echo "["
for(( i = 9313; i <= 9481; i = i + 12 ))
  do
    cleos -u http://jungle2.cryptolions.io:80 get block $i
    echo ","
    sleep 0.01
  done
echo "]"
