#!/bin/bash
printf "%s Autogenerated by gen-full-schema.sh, DO NOT EDIT\n" "--"
# EXTRACT all schema.sql files
SQL_FILES=$(find $1 -type f -name "*.up.sql")
for SQL_FILE in $SQL_FILES
do
    printf "\n\n%s %s\n\n" "--" $SQL_FILE
    cat $SQL_FILE
done