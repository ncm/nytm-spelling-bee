grep '.....' </usr/share/dict/words | grep "${1%??????}" | grep -v "[^$1]" |
  while read i; do
    echo -n "$i "
    rest=`echo "$1" | tr -d "$i"`; case "$rest" in "") echo -n '*' ;; esac;
    echo
  done 
