#ifndef CLOX_DEBUG_H
#define CLOX_DEBUG_H

#include "chunk.h"

void disassembleChunk(Chunk* chunk, const char* name);
unsigned disassembleInstruction(Chunk* chunk, unsigned i);

#endif //CLOX_DEBUG_H
