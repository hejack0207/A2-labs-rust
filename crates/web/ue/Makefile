libs: libue.so libue.a

libue.so: ue-sim-api.c
	gcc $^ --shared -fPIC -o $@

libue.a: ue-sim-api.c
	gcc -c -fPIC $^ -o ue-sim-api.o
	ar rcs $@ ue-sim-api.o
