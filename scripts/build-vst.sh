PACKAGE_NAME=(`./scripts/get-package-name.sh vst`)
NAME=$(echo $PACKAGE_NAME | perl -pe 's/dm_+([^\W_])/dm-\U$1/g' | perl -pe 's/(?<=[^\W_])_+([^\W_])/\U$1/g')
VST_NAME="$NAME.dylib"
BINARY_NAME="lib$PACKAGE_NAME.dylib"
MOVE_FROM="target/release/$BINARY_NAME"
MOVE_TO="target/release/$VST_NAME"

cd vst
cargo build --release

if [ -d "$MOVE_TO" ]; then
  rm -r "$MOVE_TO"
fi

if mv "$MOVE_FROM" "$MOVE_TO"; then
  echo "Finished compiling VST plugin. File can be found here: '$MOVE_TO'."
fi