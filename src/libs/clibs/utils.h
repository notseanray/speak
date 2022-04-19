#include <stdlib.h>

typedef struct {
	char *data;
	int size;
} String;

String StringFrom(char *data, int size) {
	// Now, let's allocate the memory for each char that we need to store.

	String s = {
		.data = (char*) malloc(sizeof(char) * size),
		.size = size
	};
};

void StringPush(String *string, String *toAppend) {
	// We create a new String with the new size.

	String s = {
		// We reallocate the memory for the new size.
		.data = (char*) realloc(string->data, sizeof(char) * (string->size + toAppend->size)),
		.size = string->size + toAppend->size
	};

	// Now, for each char in the toAppend String, we copy it to the new String.

	for (int i = 0; i < toAppend->size; i++) {
		s.data[string->size + i] = toAppend->data[i];
	}

	// We free the memory of the toAppend String.
	free(toAppend);
};

// Deallocate the memory of the String.
void deallocString(String *str) {
	free(str->data);
	free(str);
};

// ***********************************
// *				Vector
// ***********************************

typedef struct Vector {
	void **data;
	int size;
};


