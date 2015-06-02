default: StringOptimize.jb

StringOptimize.jb: StringOptimize.class
	javap -c $^ > $@

StringOptimize.class: StringOptimize.java
	javac $^

clean:
	rm -f StringOptimize.jb StringOptimize.class
