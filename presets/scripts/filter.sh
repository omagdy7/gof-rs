#!/bin/sh

for file in $(ls ./patterns/) 
do
  lines=$(cat "./patterns/$file" | wc -l)
  if [[ $lines > 42 ]];
  then
    rm "./patterns/$file"
  fi
done
