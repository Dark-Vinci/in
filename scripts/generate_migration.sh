#!/bin/sh

case $1 in
  "account")
    echo "about to cd into auth service to generate migration"
    # shellcheck disable=SC2164
    cd ../backend/account
    diesel migration generate "$2"
    command diesel print-schema > ../sdk/src/models/db/account/schema.rs;;

  "post")
    echo "about to cd into reactions service to generate migration"
    # shellcheck disable=SC2164
    cd ../backend/post/src
    diesel migration generate "$2"
    command diesel print-schema > /Users/ademolaolutomiwaabraham/Desktop/allcodes/work-go/src/new_apps/rust_build/linkedout/backend/sdk/src/models/db/post/schema.rs;;

  *)
    echo "No known service was chosen;;
esac
