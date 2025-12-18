firejail_cmd="firejail --quiet --noprofile"

for dir in "$PWD"/src/v*/; do
	if [[ "$(basename "$dir")" != $1 ]]; then
		firejail_cmd+=" --blacklist=$dir"
	fi
done

firejail_cmd+=" cbindgen --config bindgen/c/$1.toml --crate xdsim-cbinds --output bindgen/c/xdsim-$1.h"

eval $firejail_cmd
