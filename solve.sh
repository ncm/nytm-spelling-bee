grep '.....' </usr/share/dict/words | grep "${1%??????}" | grep -v "[^$1]" |
  (score=0
   while read i; do
    ((++score))
    echo -n "$i "
    rest=`echo "$1" | tr -d "$i"`
    case "$rest" in
        ("") ((score+=2)); echo -n '*' ;;
    esac
    echo
  done
  echo $score
  )
