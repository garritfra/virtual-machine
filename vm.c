#include <stdint.h>
#include <stdlib.h>

#define STACK_SIZE 1024
#define LENGTH(x) (sizeof(x))

typedef struct OBJECT_t {
  uint8_t type;
  union {
    uint8_t u8;
    int8_t i8;
    uint32_t u32;
    int32_t i32;
  };
} OBJECT;

typedef struct STACK_t {
  int top;
  int size;
  OBJECT *stack;
} STACK;

typedef struct VM_t {
  uint8_t *instructions; // Set of instructions
  uint8_t *stack;        // Stack used for operations
  int ip;                // Instruction pointer
} VM;

typedef uint8_t *(*instruction)(uint8_t *, STACK *);

VM *new_VM() {
  VM *vm = (VM *)malloc(sizeof(VM));
  vm->instructions = (int8_t *)malloc(1024);
  return vm;
}

void load_instructions(VM *vm, uint8_t ops[]) {
  for (int i = 0; i <= LENGTH(ops); i++) {
    vm->instructions[i] = ops[i];
  }
  printf("Instructions loaded: %x\n", *vm->instructions);
}

int main(int argc, char *argv[]) {
  VM *vm = new_VM();
  uint8_t ops[10];
  ops[0] = 0x40000001;
  ops[1] = 0x40000002;

  load_instructions(vm, ops);
  return 0;
}