#!/bin/sh

day=`date +%-d`
name=`printf d%02d $day`

if [ -d "$name" ]; then
  echo "Day already exists"
  exit 1
fi

cargo new $name

jq ".folders += [{\"path\":\"$name\",\"name\":\"Day $day\"}]" workspace.code-workspace > workspace.code-workspace.new
mv workspace.code-workspace.new workspace.code-workspace
