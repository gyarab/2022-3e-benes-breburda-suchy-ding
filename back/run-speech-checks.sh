#!/bin/bash

export FFMPEG_EXEC=ffmpeg

[ -n "$WHISPER_ROOT" ] || exit 2;

export WHISPER_MAIN="$WHISPER_ROOT/main"
export WHISPER_MODEL="$WHISPER_ROOT/model.bin"


export INPUT_FILE="$(realpath "$1")"

export TEMP_DIR=`mktemp -d`

clean_up_exit() {
    rm -r "$TEMP_DIR";
    exit 1;
}

get_char_count() {
    # remove all text in brackets (eg. "[_BEGIN_]" or "(playing music)")
    if [ "$DISABLE_CHECKS" == "true" ]; then
        echo "0";
    else
        $WHISPER_MAIN \
            --model $WHISPER_MODEL \
            --no-timestamps false \
            --print-special false \
            --language auto \
            --speed-up \
            $1 | sed -E 's/\[[^]]*\]|\([^\)]*\)|\s//g' | wc -c
    fi;
}

pushd "$TEMP_DIR"
# convert to proper format (and check audio is valid)
$FFMPEG_EXEC -i "$INPUT_FILE" -f wav -ar 16000 audio.wav || clean_up_exit

CHARACTERS=$(get_char_count audio.wav 2> /dev/null) || clean_up_exit

# allow up to 15 characters of error
(( $CHARACTERS > 15 )) && clean_up_exit
popd

rm -r "$TEMP_DIR"
