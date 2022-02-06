#!/bin/bash

echo 'Pass if the output following this line is empty'
diff -u <(grep -E '^\([0-9]{3}\) [0-9]{3}-[0-9]{4}$|^[0-9]{3}-[0-9]{3}-[0-9]{4}$' file.txt) <(cat test_output.txt)
