for f in `ls *.js`
do
    mv $f `echo $f | sed s/.js/.ts/`
done
