#!/bin/bash

cargo cov clean
cargo cov test
bash <(curl -s https://codecov.io/bash) -t $DEPLOY_ENV
rm *.gcno *.gcov *.gcda