#!/usr/bin/bash

# Downloads word list of english words and filters for 

DICT_ZIP_URL=https://github.com/dwyl/english-words/raw/refs/heads/master/words_alpha.zip
DICT_ZIP_TMP_PATH=/tmp/dict.zip
DICT_TMP_PATH=/tmp/dict.txt
DICT_DATA_DIR=data/
DICT_DATA_PATH=data/dict.txt

wget $DICT_ZIP_URL -O $DICT_ZIP_TMP_PATH
unzip -p $DICT_ZIP_TMP_PATH | tr -d '\r' > $DICT_TMP_PATH # Delete windows return character
awk '/^[A-Za-z]{5}$/ {print}' $DICT_TMP_PATH > $DICT_DATA_PATH
