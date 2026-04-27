/**
 * C host for the adder WebAssembly component.
 *
 * Uses the Wasmtime C API's component model support to load and run
 * a component that exports: docs:adder/add.add(u32, u32) -> u32
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <wasmtime.h>

static void exit_if_error(const char *step, wasmtime_error_t *error) {
  if (error == NULL)
    return;
  wasm_byte_vec_t error_message;
  wasmtime_error_message(error, &error_message);
  wasmtime_error_delete(error);
  fprintf(stderr, "error: failed to %s\n%.*s\n", step, (int)error_message.size,
          error_message.data);
  wasm_byte_vec_delete(&error_message);
  exit(1);
}

int main(int argc, char *argv[]) {
  if (argc != 4) {
    fprintf(stderr, "Usage: %s <x> <y> <component.wasm>\n", argv[0]);
    return 1;
  }

  uint32_t x = (uint32_t)atoi(argv[1]);
  uint32_t y = (uint32_t)atoi(argv[2]);
  const char *path = argv[3];

  // 1. Create engine with component model enabled
  wasm_config_t *config = wasm_config_new();
  wasmtime_config_wasm_component_model_set(config, true);
  wasm_engine_t *engine = wasm_engine_new_with_config(config);

  // 2. Read the component file
  FILE *f = fopen(path, "rb");
  if (!f) {
    fprintf(stderr, "error: could not open %s\n", path);
    return 1;
  }
  fseek(f, 0, SEEK_END);
  long fsize = ftell(f);
  fseek(f, 0, SEEK_SET);
  uint8_t *wasm_bytes = malloc(fsize);
  fread(wasm_bytes, 1, fsize, f);
  fclose(f);

  // 3. Compile the component
  wasmtime_component_t *component = NULL;
  exit_if_error("compile component",
      wasmtime_component_new(engine, wasm_bytes, fsize, &component));
  free(wasm_bytes);

  // 4. Create linker and add WASI P2
  wasmtime_component_linker_t *linker =
      wasmtime_component_linker_new(engine);
  exit_if_error("add WASI to linker",
      wasmtime_component_linker_add_wasip2(linker));

  // 5. Create store with WASI config
  wasmtime_store_t *store = wasmtime_store_new(engine, NULL, NULL);
  wasmtime_context_t *context = wasmtime_store_context(store);
  exit_if_error("set WASI config",
      wasmtime_context_set_wasi(context, wasi_config_new()));

  // 6. Instantiate
  wasmtime_component_instance_t instance;
  exit_if_error("instantiate component",
      wasmtime_component_linker_instantiate(linker, context, component, &instance));

  // 7. Look up the exported "add" function.
  //    The export is nested: first find the "docs:adder/add@0.1.0" instance,
  //    then the "add" function within it.
  wasmtime_component_export_index_t *iface_idx =
      wasmtime_component_instance_get_export_index(
          &instance, context, NULL,
          "docs:adder/add@0.1.0", strlen("docs:adder/add@0.1.0"));
  if (iface_idx == NULL) {
    fprintf(stderr, "error: could not find export 'docs:adder/add@0.1.0'\n");
    return 1;
  }

  wasmtime_component_export_index_t *func_idx =
      wasmtime_component_instance_get_export_index(
          &instance, context, iface_idx,
          "add", strlen("add"));
  wasmtime_component_export_index_delete(iface_idx);
  if (func_idx == NULL) {
    fprintf(stderr, "error: could not find function 'add'\n");
    return 1;
  }

  wasmtime_component_func_t func;
  bool found = wasmtime_component_instance_get_func(
      &instance, context, func_idx, &func);
  wasmtime_component_export_index_delete(func_idx);
  if (!found) {
    fprintf(stderr, "error: could not get function handle for 'add'\n");
    return 1;
  }

  // 8. Call the function: add(x, y) -> u32
  wasmtime_component_val_t args[2] = {
      {.kind = WASMTIME_COMPONENT_U32, .of.u32 = x},
      {.kind = WASMTIME_COMPONENT_U32, .of.u32 = y},
  };
  wasmtime_component_val_t results[1] = {0};

  exit_if_error("call 'add'",
      wasmtime_component_func_call(&func, context, args, 2, results, 1));

  printf("%u + %u = %u\n", x, y, results[0].of.u32);

  // 9. Cleanup
  wasmtime_component_val_delete(&results[0]);
  wasmtime_store_delete(store);
  wasmtime_component_linker_delete(linker);
  wasmtime_component_delete(component);
  wasm_engine_delete(engine);

  return 0;
}
