#while getopts abc OPT
#do
#  case $OPT in
#     a) echo "[-a] が指定された";;
#     b) echo "[-b] が指定された";;
#     c) echo "[-c] が指定された";;
#     *) echo "該当なし（OPT=$OPT）";;
#  esac
#done
required() { [ $# -gt 1 ] || abort "option '$1' requires an argument"; }

FLAG_A='' FLAG_B='' ARG_I='' ARG_J=''

neovim_opt=''

#while getopts :a opt;do
#    case $opt in
#        l ) echo "$OPTARG";;
#        a ) echo "$OPTARG";;
#	* ) echo $opt;;
#    esac
#done

#while getopts abc OPT
#do
#  case $OPT in
#     a) echo "[-a] が指定された";;
#     b) echo "[-b] が指定された";;
#     c) echo "[-c] が指定された";;
#     *) echo "該当なし（OPT=$OPT）";;
#  esac
#done

while [ $# -gt 0 ]; do
  case $1 in
    --upgrade ) echo upgrade ;;
    *) neovim_opt="$neovim_opt $1 $2"
    #*) echo break;echo $1;echo $2; break
  esac
  shift
done
echo $neovim_opt
