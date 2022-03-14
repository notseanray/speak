// I coded the run algorithm in C, just because it is easier for me to understand.

#include <stdlib.h>
#include <string.h>

typedef struct {

	char* body;
	size_t length;

// Methods

	void (*push)();

} String;

String mpush(String original, String _new) {

	String _final;

	_final.length = original.length + _new.length;
	_final.body = (char *) malloc(_final.length);

	_final.body = strcpy(_final.body, original.body);
	// Now we add the new string to the end of the original string.

	for (int i = 0; i < _new.length; i++) {
		_final.body[original.length + i] = _new.body[i];
	}

	// Now we free the memory of the strings.

	free(original.body);
	free(_new.body);

	// We return the new string.
	return _final;
};


String string() {

	String _string;
	_string.body = (char *) malloc(1);
	_string.length = 1;

	_string.push = &mpush;

	return _string;
};

typedef unsigned int uint;

#define mvector(typex) typedef struct {\
\
	##typex* body;\
	size_t length;\
\
} Vec##typex;


mvector(uint);
mvector(Vecuint);

mvector(float);
mvector(Vecfloat);

mvector(String);

#define mdec(typex) \
typedef struct {\
\
	Vec##typex keys;\
	Vec##typex values;\
\
} Dec##typex;

mdec(Vecuint);
mdec(String)

typedef struct {
	VecVecfloat* learnt_vec;
	DecVecuint* translated_deconstructed;
	DecString* raw_deconstructed;
} Learnt;

#define create_vec(name, typex) \
Vec##typex _##name;\
_##name.length = 0;\
_##name.body = (##typex *) malloc(1);

Learnt learn(DecString map, uint memory, uint multiplier) {
	// Here we have an advantage, in C, all characters are numbers, so we don't need to convert them to numbers.
	
	create_vec(kvec, uint);
	create_vec(vvec, uint);

	uint kmem;
	uint vmem;

	for (int i = 0; i < map.keys.length; i++) {
		
		if (memory >= _kvec.length) {
			kmem = _kvec.length;
		} else {
			kmem = memory;
		};

		// Split
		strtok_r(map.keys.body[i].body, ' ', &map.keys.body[i].body);

		// Translate it
		for (int j = 0; j < map.keys.body[i].length; j++) {
			_kvec.body[_kvec.length] = multiplier * (uint) map.keys.body[i].body[j];
		};

		// We free the original string.
		free(map.keys.body[i].body);
		free(map.keys.body[i].length);

		// Chunk it
	};
};
