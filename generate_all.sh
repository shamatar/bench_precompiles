#!/bin/sh
for D in $(find ./vectors -mindepth 0 -maxdepth 3 -name '*.csv') ; do
    echo $D ;
done

# for dir in vectors/    # list directories in the form "/tmp/dirname/"
# do
#     dir=${dir%*/}      # remove the trailing "/"
#     echo $dir 
#     # echo ${dir##*/} 
#     # dir=${dir%*/}
#     # cd $dir 
#     # cd current
#     # rm -rf *.csv
#     # cd ..
#     # cd proposed
#     # rm -rf *.csv
#     # cd ..
#     # dir=${dir%*/}      # remove the trailing "/"
#     # echo ${dir##*/}    # print everything after the final "/"
# done
# cd ..
cargo test --release --no-run
sleep 30
cargo test --release -- --test-threads=1 generate_

