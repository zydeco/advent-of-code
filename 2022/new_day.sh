#!/bin/sh

day=`date +%-d`
suffix=$1
name=`printf d%02d$suffix $day`
display_name="Day $day"
if [ ! -z "$suffix" ]; then
  display_name="$display_name $suffix"
fi

if [ -d "$name" ]; then
  echo "Day already exists"
  exit 1
fi

cargo new $name

jq ".folders += [{\"path\":\"$name\",\"name\":\"$display_name\"}]" workspace.code-workspace > workspace.code-workspace.new
mv workspace.code-workspace.new workspace.code-workspace
